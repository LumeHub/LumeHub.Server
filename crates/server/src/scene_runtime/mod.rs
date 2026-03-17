mod reload;
mod state;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[cfg(feature = "google")]
use chrono::Utc;
use engine::{EffectQueue, RenderCommand};

#[cfg(feature = "google")]
use api_google::effects::{ColorLoop, Sleep, Wake};
use application::{SceneRuntime, SceneSnapshot, StateEventBus};
use domain::Rgb;
use effects::{BuiltinEffect, LiveParam};
use persistence::PersistedState;
use store::Store;

use reload::SceneReload;
use state::{ActiveScene, SceneState, build_snapshot, extract_persisted};

pub struct RenderTaskRuntime {
    pub(crate) queue: EffectQueue,
    pub(crate) state: Arc<Mutex<SceneState>>,
    pub(crate) bus: StateEventBus,
    pub(crate) state_tx: Option<tokio::sync::watch::Sender<PersistedState>>,
    pub(crate) store: Arc<Store>,
    pub(crate) builtins: Arc<HashMap<String, BuiltinEffect>>,
    pub(crate) primary_color: Arc<LiveParam<Rgb>>,
    pub(crate) strip_len: usize,
}

impl RenderTaskRuntime {
    pub fn new(
        queue: EffectQueue,
        bus: StateEventBus,
        initial: PersistedState,
        state_tx: Option<tokio::sync::watch::Sender<PersistedState>>,
        store: Arc<Store>,
        builtins: Vec<BuiltinEffect>,
        strip_len: usize,
    ) -> Self {
        let color_speed = queue.color_speed();
        Self {
            queue,
            state: Arc::new(Mutex::new(SceneState {
                on: initial.on,
                brightness: initial.brightness,
                color: initial.color,
                scene: ActiveScene::Idle,
            })),
            bus,
            state_tx,
            store,
            builtins: Arc::new(builtins.into_iter().map(|b| (b.id(), b)).collect()),
            primary_color: Arc::new(LiveParam::new(initial.color, color_speed)),
            strip_len,
        }
    }

    fn emit(&self, snapshot: SceneSnapshot, persisted: PersistedState) {
        self.bus.notify(snapshot);
        if let Some(tx) = &self.state_tx {
            let _ = tx.send(persisted);
        }
    }

    fn with_state<F: FnOnce(&mut SceneState, &EffectQueue)>(&self, f: F) {
        let (snapshot, persisted) = {
            let mut s = self.state.lock().unwrap();
            f(&mut s, &self.queue);
            (build_snapshot(&s), extract_persisted(&s))
        };
        self.emit(snapshot, persisted);
    }
}

impl SceneRuntime for RenderTaskRuntime {
    fn set_color(&self, color: Rgb) {
        self.primary_color.set(color);
        self.with_state(|state, queue| {
            state.color = color;
            state.on = true;
            if matches!(state.scene, ActiveScene::Idle) {
                queue.send(RenderCommand::SetColor(color));
            }
        });
    }

    fn set_brightness(&self, brightness: u8) {
        self.with_state(|state, queue| {
            state.brightness = brightness;
            if let ActiveScene::Running { brightness: param } = &state.scene {
                param.set(brightness as f32);
            } else {
                queue.send(RenderCommand::SetBrightness(brightness));
            }
        });
    }

    fn set_on_off(&self, on: bool) {
        self.with_state(|state, queue| {
            state.on = on;
            if let ActiveScene::Running { brightness } = &state.scene {
                brightness.set(if on { state.brightness as f32 } else { 0.0 });
            } else {
                queue.send(RenderCommand::SetOnOff(on));
            }
        });
    }

    fn halt(&self) {
        self.with_state(|state, queue| {
            state.scene = ActiveScene::Idle;
            queue.send(RenderCommand::Halt);
        });
    }

    fn stop_effect(&self) {
        self.with_state(|state, queue| {
            let on = state.on;
            state.scene = ActiveScene::Idle;
            queue.send(RenderCommand::SetOnOff(on));
        });
    }

    #[cfg(feature = "google")]
    fn start_color_loop(&self, duration: u64) {
        self.with_state(|state, queue| {
            state.on = true;
            let start_color = state.color;
            state.scene = ActiveScene::Timed {
                name: "colorLoop".to_string(),
                end_unix_timestamp_sec: Utc::now().timestamp() as u64 + duration,
            };
            queue.enqueue(Box::new(ColorLoop {
                duration,
                start_color,
                colors: vec![
                    Rgb::new(255, 0, 0),
                    Rgb::new(255, 127, 0),
                    Rgb::new(255, 255, 0),
                    Rgb::new(0, 255, 0),
                    Rgb::new(0, 0, 255),
                    Rgb::new(75, 0, 130),
                    Rgb::new(148, 0, 211),
                ],
            }));
        });
    }

    #[cfg(feature = "google")]
    fn start_sleep(&self, duration: u64) {
        self.with_state(|state, queue| {
            state.on = true;
            let start_brightness = state.brightness;
            state.scene = ActiveScene::Timed {
                name: "sleep".to_string(),
                end_unix_timestamp_sec: Utc::now().timestamp() as u64 + duration,
            };
            queue.enqueue(Box::new(Sleep {
                duration,
                start_brightness,
                target_color: Rgb::BLACK,
            }));
        });
    }

    #[cfg(feature = "google")]
    fn start_wake(&self, duration: u64) {
        self.with_state(|state, queue| {
            state.on = true;
            let end_brightness = state.brightness;
            let start_color = state.color;
            state.scene = ActiveScene::Timed {
                name: "wake".to_string(),
                end_unix_timestamp_sec: Utc::now().timestamp() as u64 + duration,
            };
            queue.enqueue(Box::new(Wake {
                duration,
                end_brightness,
                start_color,
            }));
        });
    }

    fn reload_active(&self) {
        SceneReload::from_runtime(self).spawn();
    }

    fn snapshot(&self) -> SceneSnapshot {
        build_snapshot(&self.state.lock().unwrap())
    }
}
