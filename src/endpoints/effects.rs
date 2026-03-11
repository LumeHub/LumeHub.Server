use std::collections::HashMap;
use std::sync::Mutex;

use actix_web::{HttpResponse, Responder, delete, get, post, put, web};
use serde::{Deserialize, Serialize};

use crate::color::Rgb;
use crate::effects::EffectQueue;
use crate::effects::builder::{build_composite, build_prelude};
use crate::effects::composite::PRIMARY_COLOR;
use crate::effects::config::{EffectPreset, EffectsConfig};
use crate::effects::registry::EffectRegistry;
use crate::lume_service::LumeService;
use crate::state::LumeState;

#[derive(Serialize)]
struct PresetsResponse<'a> {
    presets: &'a HashMap<String, EffectPreset>,
}

#[post("/effects/presets/{name}/execute")]
pub async fn execute_preset(
    effect_queue: web::Data<EffectQueue>,
    registry: web::Data<EffectRegistry>,
    lume_state: web::Data<Mutex<LumeState>>,
    effects: web::Data<EffectsConfig>,
    path: web::Path<String>,
) -> impl Responder {
    let preset_name = path.into_inner();
    let preset = match effects.presets.get(&preset_name) {
        Some(p) => p.clone(),
        None => return HttpResponse::NotFound().body("unknown preset"),
    };

    let (initial_color, signal_colors, signal_scripts, initial_brightness, is_on) = {
        let state = lume_state.lock().unwrap();
        (
            state.active_color,
            state.signal_colors.clone(),
            state.signal_scripts.clone(),
            state.brightness,
            state.is_on,
        )
    };

    let prelude = build_prelude(&effects.functions);
    let (composite, bus) = match build_composite(
        &registry,
        &preset,
        initial_color,
        &signal_colors,
        if is_on { initial_brightness } else { 0 },
        effect_queue.brightness_speed(),
        prelude,
        &effects.presets,
    ) {
        Ok(result) => result,
        Err(msg) => return HttpResponse::BadRequest().body(msg),
    };

    // Apply user animated overrides — these beat preset animations.
    for (name, code) in &signal_scripts {
        let _ = bus.set_animated(name, code);
    }
    // Apply user static overrides — these beat everything (incl. animations).
    for (name, &color) in &signal_colors {
        bus.set_color(name, color);
    }

    let mut state = lume_state.lock().unwrap();
    state.active_light_effect = Some(preset_name);
    state.light_effect_end_unix_timestamp_sec = None;
    state.active_param_bus = Some(bus);
    effect_queue.enqueue(Box::new(composite));
    HttpResponse::Ok().finish()
}

#[get("/effects/presets")]
pub async fn list_presets(effects: web::Data<EffectsConfig>) -> impl Responder {
    HttpResponse::Ok().json(PresetsResponse {
        presets: &effects.presets,
    })
}

#[delete("/effects/active")]
pub async fn delete_active(
    lume_state: web::Data<Mutex<LumeState>>,
    effect_queue: web::Data<EffectQueue>,
) -> impl Responder {
    LumeService::new(&lume_state, &effect_queue).halt_effect();
    HttpResponse::NoContent().finish()
}

#[get("/effects/signals")]
pub async fn get_signals(lume_state: web::Data<Mutex<LumeState>>) -> impl Responder {
    let state = lume_state.lock().unwrap();
    let signals: HashMap<String, Rgb> = if let Some(bus) = &state.active_param_bus {
        bus.all_colors()
            .into_iter()
            .map(|(name, param)| (name, param.get()))
            .collect()
    } else {
        HashMap::new()
    };
    HttpResponse::Ok().json(signals)
}

#[derive(Deserialize)]
#[serde(untagged)]
enum SetSignalRequest {
    Color { r: u8, g: u8, b: u8 },
    Script { code: String },
}

#[put("/effects/signals/{name}")]
pub async fn set_signal(
    lume_state: web::Data<Mutex<LumeState>>,
    path: web::Path<String>,
    body: web::Bytes,
) -> impl Responder {
    let req: SetSignalRequest = match serde_json::from_slice(&body) {
        Ok(r) => r,
        Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
    };
    let signal_name = path.into_inner();
    let mut state = lume_state.lock().unwrap();

    match req {
        SetSignalRequest::Color { r, g, b } => {
            let color = Rgb { r, g, b };
            state.signal_scripts.remove(&signal_name);
            if signal_name == PRIMARY_COLOR {
                state.active_color = color;
            } else {
                state.signal_colors.insert(signal_name.clone(), color);
            }
            if let Some(bus) = &state.active_param_bus {
                bus.set_color(&signal_name, color);
            }
        }
        SetSignalRequest::Script { code } => {
            if let Some(bus) = &state.active_param_bus
                && let Err(e) = bus.set_animated(&signal_name, &code)
            {
                return HttpResponse::BadRequest().body(format!("script error: {}", e));
            }
            state.signal_scripts.insert(signal_name, code);
        }
    }

    HttpResponse::Ok().finish()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(execute_preset)
        .service(list_presets)
        .service(delete_active)
        .service(get_signals)
        .service(set_signal);
}
