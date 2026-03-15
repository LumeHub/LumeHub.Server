use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use application::StateEventBus;
use domain::{BlendMode, ParamDef, ParamValue};
use effects::composite::CompositeEffect;
use effects::scene_builder::LayerSpec;
use effects::{BuiltinEffect, LiveParam, build_composite};
use engine::{EffectQueue, RenderCommand};
use persistence::PersistedState;
use store::{LayerRecord, Store, ZoneRecord};
use tokio::sync::watch;

use super::RenderTaskRuntime;
use super::state::{ActiveScene, SceneState, build_snapshot, extract_persisted};

type LayerSpecData = (
    String,
    Vec<ParamDef>,
    HashMap<String, ParamValue>,
    BlendMode,
    ZoneRecord,
);

pub(super) struct SceneReload {
    store: Arc<Store>,
    builtins: Arc<HashMap<String, BuiltinEffect>>,
    state: Arc<Mutex<SceneState>>,
    queue: EffectQueue,
    bus: StateEventBus,
    state_tx: Option<watch::Sender<PersistedState>>,
    strip_len: usize,
    brightness_val: f32,
}

impl SceneReload {
    pub(super) fn from_runtime(rt: &RenderTaskRuntime) -> Self {
        let brightness_val = {
            let s = rt.state.lock().unwrap();
            if s.on { s.brightness as f32 } else { 0.0 }
        };
        Self {
            store: Arc::clone(&rt.store),
            builtins: Arc::clone(&rt.builtins),
            state: Arc::clone(&rt.state),
            queue: rt.queue.clone(),
            bus: rt.bus.clone(),
            state_tx: rt.state_tx.clone(),
            strip_len: rt.strip_len,
            brightness_val,
        }
    }

    pub(super) fn spawn(self) {
        tokio::spawn(async move { self.run().await });
    }

    async fn run(self) {
        let Some(enabled) = self.enabled_layers().await else {
            self.go_idle();
            return;
        };

        let specs_data = self.resolve_specs(&enabled).await;
        if specs_data.is_empty() {
            self.go_idle();
            return;
        }

        let brightness = Arc::new(LiveParam::new(
            self.brightness_val,
            self.queue.brightness_speed(),
        ));
        let specs = to_layer_specs(&specs_data);

        match build_composite(&specs, self.strip_len, Arc::clone(&brightness)) {
            Ok(composite) => self.start(brightness, composite),
            Err(e) => eprintln!("warning: failed to build composite: {e}"),
        }
    }

    fn go_idle(&self) {
        let (snapshot, persisted) = {
            let mut s = self.state.lock().unwrap();
            s.scene = ActiveScene::Idle;
            self.queue.send(RenderCommand::Halt);
            (build_snapshot(&s), extract_persisted(&s))
        };
        self.emit(snapshot, persisted);
    }

    fn start(&self, brightness: Arc<LiveParam<f32>>, composite: CompositeEffect) {
        let (snapshot, persisted) = {
            let mut s = self.state.lock().unwrap();
            s.scene = ActiveScene::Running { brightness };
            (build_snapshot(&s), extract_persisted(&s))
        };
        self.queue.enqueue(Box::new(composite));
        self.emit(snapshot, persisted);
    }

    fn emit(&self, snapshot: application::SceneSnapshot, persisted: PersistedState) {
        self.bus.notify(snapshot);
        if let Some(tx) = &self.state_tx {
            let _ = tx.send(persisted);
        }
    }

    async fn enabled_layers(&self) -> Option<Vec<LayerRecord>> {
        let layers = self
            .store
            .get_active_layers()
            .await
            .inspect_err(|e| eprintln!("warning: failed to load active layers: {e}"))
            .ok()?;
        let enabled: Vec<_> = layers.into_iter().filter(|l| l.enabled).collect();
        (!enabled.is_empty()).then_some(enabled)
    }

    async fn resolve_specs(&self, layers: &[LayerRecord]) -> Vec<LayerSpecData> {
        let mut specs = Vec::with_capacity(layers.len());
        for layer in layers {
            if let Some(s) = self.resolve_one(layer).await {
                specs.push(s);
            }
        }
        specs
    }

    async fn resolve_one(&self, layer: &LayerRecord) -> Option<LayerSpecData> {
        let (script, defs) = self.resolve_effect(&layer.effect_id).await?;
        let zone = self.resolve_zone(&layer.zone_id).await?;
        Some((script, defs, layer.params.clone(), layer.blend_mode, zone))
    }

    async fn resolve_effect(&self, effect_id: &str) -> Option<(String, Vec<ParamDef>)> {
        if let Some(b) = self.builtins.get(effect_id) {
            return Some((b.script.clone(), b.params.clone()));
        }
        self.store
            .get_effect(effect_id)
            .await
            .inspect_err(|e| eprintln!("warning: effect '{effect_id}' not found: {e}"))
            .ok()
            .map(|e| (e.script, e.params))
    }

    async fn resolve_zone(&self, zone_id: &str) -> Option<ZoneRecord> {
        if zone_id == "all" {
            return Some(ZoneRecord {
                id: "all".into(),
                name: "Full Strip".into(),
                start_pixel: 0,
                end_pixel: self.strip_len as u32,
                transition_length: 0,
            });
        }
        self.store
            .get_zone(zone_id)
            .await
            .inspect_err(|e| eprintln!("warning: zone '{zone_id}' not found: {e}"))
            .ok()
    }
}

fn to_layer_specs(data: &[LayerSpecData]) -> Vec<LayerSpec<'_>> {
    data.iter()
        .map(|(script, defs, params, mode, zone)| LayerSpec {
            script: script.as_str(),
            param_defs: defs.as_slice(),
            params,
            blend_mode: *mode,
            zone_start: zone.start_pixel as usize,
            zone_end: zone.end_pixel as usize,
            zone_transition: zone.transition_length as usize,
        })
        .collect()
}
