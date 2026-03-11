use std::sync::Mutex;

use actix_web::{HttpResponse, Responder, get, patch, web};
use serde::{Deserialize, Serialize};

use crate::color::Rgb;
use crate::effects::EffectQueue;
use crate::lume_service::LumeService;
use crate::state::LumeState;

#[derive(Serialize)]
struct DeviceStateResponse {
    on: bool,
    brightness: u8,
    color: Rgb,
    active_effect: Option<String>,
}

#[get("/device/state")]
pub async fn get_state(state: web::Data<Mutex<LumeState>>) -> impl Responder {
    let state = state.lock().unwrap();
    HttpResponse::Ok().json(DeviceStateResponse {
        on: state.is_on,
        brightness: state.brightness,
        color: state.active_color,
        active_effect: state.active_light_effect.clone(),
    })
}

#[derive(Deserialize)]
struct PatchStateRequest {
    on: Option<bool>,
    brightness: Option<u8>,
    color: Option<Rgb>,
}

#[patch("/device/state")]
pub async fn patch_state(
    lume_state: web::Data<Mutex<LumeState>>,
    effect_queue: web::Data<EffectQueue>,
    body: web::Json<PatchStateRequest>,
) -> impl Responder {
    let mut service = LumeService::new(&lume_state, &effect_queue);
    if let Some(on) = body.on {
        service.set_on_off(on);
    }
    if let Some(brightness) = body.brightness {
        service.set_brightness(brightness);
    }
    if let Some(color) = body.color {
        service.set_color(color);
    }
    let state = lume_state.lock().unwrap();
    HttpResponse::Ok().json(DeviceStateResponse {
        on: state.is_on,
        brightness: state.brightness,
        color: state.active_color,
        active_effect: state.active_light_effect.clone(),
    })
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_state).service(patch_state);
}
