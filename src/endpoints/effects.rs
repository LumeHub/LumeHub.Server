use std::sync::Mutex;

use actix_web::{HttpResponse, Responder, get, post, web};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::effects::EffectQueue;
use crate::effects::registry::EffectRegistry;
use crate::settings::{EffectPreset, EffectsConfig};
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
    presets: &'a std::collections::HashMap<String, EffectPreset>,
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
        Some(preset) => preset,
        None => return HttpResponse::NotFound().body("unknown preset"),
    };

    let mut params = preset.params.clone();
    if !body.is_empty() {
        let overrides = match serde_json::from_slice::<ExecutePresetRequest>(&body) {
            Ok(parsed) => parsed.overrides,
            Err(err) => {
                return HttpResponse::BadRequest().body(format!("Json deserialize error: {}", err));
            }
        };
        if let Some(overrides) = overrides {
            merge_values(&mut params, overrides);
        }
    }

    match registry.build(&preset.effect, params) {
        Ok(effect) => {
            let mut state = lume_state.lock().unwrap();
            state.is_on = true;
            state.active_light_effect = Some(preset_name);
            state.light_effect_end_unix_timestamp_sec = None;
            effect_queue.enqueue(effect);
            HttpResponse::Ok().finish()
        }
        Err(err) => HttpResponse::BadRequest().body(err.0),
    }
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
