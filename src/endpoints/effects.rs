use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use actix_web::{HttpResponse, Responder, get, post, web};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::color::Rgb;
use crate::effects::EffectQueue;
use crate::effects::composite::{CompositeEffect, CompositeLayer, CompositeMode};
use crate::effects::composite::{ParameterBus, SECONDARY_COLOR};
use crate::effects::registry::EffectRegistry;
use crate::settings::{EffectPreset, EffectsConfig, LayerPreset};
use crate::state::LumeState;

#[derive(Deserialize)]
struct ExecuteEffectRequest {
    effect: String,
    params: Value,
}

#[derive(Deserialize)]
struct ExecutePresetRequest {
    overrides: Option<Value>,
}

#[derive(Serialize)]
struct PresetsResponse<'a> {
    presets: &'a HashMap<String, EffectPreset>,
}

#[post("/effects/execute")]
pub async fn execute(
    effect_queue: web::Data<EffectQueue>,
    registry: web::Data<EffectRegistry>,
    lume_state: web::Data<Mutex<LumeState>>,
    body: web::Bytes,
) -> impl Responder {
    if body.is_empty() {
        return HttpResponse::BadRequest().body("Json deserialize error: empty body");
    }

    let parsed = match serde_json::from_slice::<ExecuteEffectRequest>(&body) {
        Ok(parsed) => parsed,
        Err(err) => {
            return HttpResponse::BadRequest().body(format!("Json deserialize error: {}", err));
        }
    };

    match registry.build(&parsed.effect, parsed.params) {
        Ok(effect) => {
            let mut state = lume_state.lock().unwrap();
            state.is_on = true;
            state.active_light_effect = Some(parsed.effect);
            state.light_effect_end_unix_timestamp_sec = None;
            state.active_param_bus = None;
            effect_queue.enqueue(effect);
            HttpResponse::Ok().finish()
        }
        Err(err) => HttpResponse::BadRequest().body(err.0),
    }
}

#[post("/effects/presets/{name}/execute")]
pub async fn execute_preset(
    effect_queue: web::Data<EffectQueue>,
    registry: web::Data<EffectRegistry>,
    lume_state: web::Data<Mutex<LumeState>>,
    effects: web::Data<EffectsConfig>,
    path: web::Path<String>,
    body: web::Bytes,
) -> impl Responder {
    let preset_name = path.into_inner();
    let preset = match effects.presets.get(&preset_name) {
        Some(preset) => preset.clone(),
        None => return HttpResponse::NotFound().body("unknown preset"),
    };

    match preset {
        EffectPreset::Single { effect, mut params } => {
            if !body.is_empty() {
                let overrides = match serde_json::from_slice::<ExecutePresetRequest>(&body) {
                    Ok(parsed) => parsed.overrides,
                    Err(err) => {
                        return HttpResponse::BadRequest()
                            .body(format!("Json deserialize error: {}", err));
                    }
                };
                if let Some(overrides) = overrides {
                    merge_values(&mut params, overrides);
                }
            }

            match registry.build(&effect, params) {
                Ok(built) => {
                    let mut state = lume_state.lock().unwrap();
                    state.is_on = true;
                    state.active_light_effect = Some(preset_name);
                    state.light_effect_end_unix_timestamp_sec = None;
                    state.active_param_bus = None;
                    effect_queue.enqueue(built);
                    HttpResponse::Ok().finish()
                }
                Err(err) => HttpResponse::BadRequest().body(err.0),
            }
        }

        EffectPreset::Composite {
            layers: layer_presets,
        } => {
            let (initial_color, initial_brightness) = {
                let state = lume_state.lock().unwrap();
                (state.active_color, state.brightness)
            };

            match build_composite(&registry, &layer_presets, initial_color, initial_brightness) {
                Ok((composite, bus)) => {
                    let mut state = lume_state.lock().unwrap();
                    state.is_on = true;
                    state.active_light_effect = Some(preset_name);
                    state.light_effect_end_unix_timestamp_sec = None;
                    state.active_param_bus = Some(bus);
                    effect_queue.enqueue(Box::new(composite));
                    HttpResponse::Ok().finish()
                }
                Err(response) => response,
            }
        }
    }
}

fn build_composite(
    registry: &EffectRegistry,
    layer_presets: &[LayerPreset],
    initial_color: Rgb,
    initial_brightness: u8,
) -> Result<(CompositeEffect, Arc<ParameterBus>), HttpResponse> {
    let color_signals: HashMap<String, (Rgb, f32)> = layer_presets
        .iter()
        .flat_map(|layer| layer.bindings.values())
        .fold(HashMap::new(), |mut acc, binding| {
            acc.entry(binding.signal.clone()).or_insert_with(|| {
                let speed = binding.speed.unwrap_or(5.0);
                let initial = binding
                    .initial
                    .as_ref()
                    .and_then(|v| serde_json::from_value::<Rgb>(v.clone()).ok())
                    .unwrap_or_else(|| {
                        if binding.signal == SECONDARY_COLOR {
                            Rgb::BLACK
                        } else {
                            initial_color
                        }
                    });
                (initial, speed)
            });
            acc
        });

    let bus = color_signals.into_iter().fold(
        ParameterBus::new(initial_brightness as f32),
        |mut bus, (name, (initial, speed))| {
            bus.register_color(name, initial, speed);
            bus
        },
    );
    let bus = Arc::new(bus);

    let layers = layer_presets
        .iter()
        .map(|layer_preset| {
            registry
                .build_layer(
                    &layer_preset.effect,
                    layer_preset.params.clone(),
                    &layer_preset.bindings,
                    &bus,
                )
                .map(|effect| CompositeLayer {
                    effect,
                    mode: layer_preset
                        .mode
                        .as_deref()
                        .map(CompositeMode::from_str)
                        .unwrap_or(CompositeMode::Override),
                    opacity_gradient: layer_preset.opacity_gradient,
                })
                .map_err(|err| {
                    HttpResponse::BadRequest().body(format!(
                        "Error building layer '{}': {}",
                        layer_preset.effect, err.0
                    ))
                })
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok((
        CompositeEffect {
            layers,
            bus: Arc::clone(&bus),
        },
        bus,
    ))
}

#[get("/effects/presets")]
pub async fn list_presets(effects: web::Data<EffectsConfig>) -> impl Responder {
    HttpResponse::Ok().json(PresetsResponse {
        presets: &effects.presets,
    })
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(execute)
        .service(execute_preset)
        .service(list_presets);
}

fn merge_values(base: &mut Value, overrides: Value) {
    match (base, overrides) {
        (Value::Object(base_map), Value::Object(override_map)) => {
            for (key, override_value) in override_map {
                match base_map.get_mut(&key) {
                    Some(base_value) => merge_values(base_value, override_value),
                    None => {
                        base_map.insert(key, override_value);
                    }
                }
            }
        }
        (base_value, override_value) => {
            *base_value = override_value;
        }
    }
}
