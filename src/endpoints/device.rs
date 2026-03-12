use actix_web::{HttpResponse, Responder, get, patch, web};
use application::{SceneRuntime, SceneSnapshot};
use domain::Rgb;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct DeviceStateResponse {
    on: bool,
    brightness: u8,
    color: Rgb,
    active_effect: Option<String>,
}

impl From<SceneSnapshot> for DeviceStateResponse {
    fn from(s: SceneSnapshot) -> Self {
        Self {
            on: s.on,
            brightness: s.brightness,
            color: s.color,
            active_effect: s.active_effect,
        }
    }
}

#[get("/device/state")]
pub async fn get_state(runtime: web::Data<dyn SceneRuntime>) -> impl Responder {
    HttpResponse::Ok().json(DeviceStateResponse::from(runtime.snapshot()))
}

#[derive(Deserialize)]
struct PatchStateRequest {
    on: Option<bool>,
    brightness: Option<u8>,
    color: Option<Rgb>,
}

#[patch("/device/state")]
pub async fn patch_state(
    runtime: web::Data<dyn SceneRuntime>,
    body: web::Json<PatchStateRequest>,
) -> impl Responder {
    if let Some(on) = body.on {
        runtime.set_on_off(on);
    }
    if let Some(brightness) = body.brightness {
        runtime.set_brightness(brightness);
    }
    if let Some(color) = body.color {
        runtime.set_color(color);
    }
    HttpResponse::Ok().json(DeviceStateResponse::from(runtime.snapshot()))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_state).service(patch_state);
}
