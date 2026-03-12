use std::collections::HashMap;
use std::sync::Arc;

use actix_web::{HttpResponse, Responder, delete, get, post, put, web};
use serde::{Deserialize, Serialize};

use crate::effects::EffectQueue;
use crate::effects::builder::{build_composite, build_prelude};
use crate::effects::config::{EffectPreset, EffectsConfig};
use crate::effects::registry::EffectRegistry;
use application::{BusProxy, SceneRuntime, SignalValue};
use domain::Rgb;

#[derive(Serialize)]
struct PresetsResponse<'a> {
    presets: &'a HashMap<String, EffectPreset>,
}

#[post("/effects/presets/{name}/execute")]
pub async fn execute_preset(
    effect_queue: web::Data<EffectQueue>,
    registry: web::Data<EffectRegistry>,
    runtime: web::Data<dyn SceneRuntime>,
    effects: web::Data<EffectsConfig>,
    path: web::Path<String>,
) -> impl Responder {
    let preset_name = path.into_inner();
    let preset = match effects.presets.get(&preset_name) {
        Some(p) => p.clone(),
        None => return HttpResponse::NotFound().body("unknown preset"),
    };

    let snap = runtime.snapshot();
    let overrides = runtime.signal_overrides();
    let initial_color = snap.color;
    let initial_brightness = if snap.on { snap.brightness } else { 0 };
    let signal_colors = overrides.colors;
    let signal_scripts = overrides.scripts;

    let prelude = build_prelude(&effects.functions);
    let (composite, bus) = match build_composite(
        &registry,
        &preset,
        initial_color,
        &signal_colors,
        initial_brightness,
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

    runtime.attach_bus(Arc::clone(&bus) as Arc<dyn BusProxy>, preset_name);
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
pub async fn delete_active(runtime: web::Data<dyn SceneRuntime>) -> impl Responder {
    runtime.halt();
    HttpResponse::NoContent().finish()
}

#[get("/effects/signals")]
pub async fn get_signals(runtime: web::Data<dyn SceneRuntime>) -> impl Responder {
    HttpResponse::Ok().json(runtime.bus_colors())
}

#[derive(Deserialize)]
#[serde(untagged)]
enum SetSignalRequest {
    Color { r: u8, g: u8, b: u8 },
    Script { code: String },
}

#[put("/effects/signals/{name}")]
pub async fn set_signal(
    runtime: web::Data<dyn SceneRuntime>,
    path: web::Path<String>,
    body: web::Bytes,
) -> impl Responder {
    let req: SetSignalRequest = match serde_json::from_slice(&body) {
        Ok(r) => r,
        Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
    };
    let signal_name = path.into_inner();

    let result = match req {
        SetSignalRequest::Color { r, g, b } => {
            runtime.set_signal(&signal_name, SignalValue::Color(Rgb { r, g, b }))
        }
        SetSignalRequest::Script { code } => {
            runtime.set_signal(&signal_name, SignalValue::Script(code))
        }
    };

    match result {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::BadRequest().body(format!("script error: {}", e)),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(execute_preset)
        .service(list_presets)
        .service(delete_active)
        .service(get_signals)
        .service(set_signal);
}
