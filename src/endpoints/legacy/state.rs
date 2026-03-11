use std::sync::Mutex;

use actix_web::{HttpResponse, Responder, post, web};
use serde::Serialize;

use crate::effects::EffectQueue;
use crate::lume_service::LumeService;
use crate::state::LumeState;

#[derive(Serialize)]
pub struct LedStateResponse {
    #[serde(rename = "ledState")]
    pub led_state: bool,
}

#[post("/led/state/get")]
pub async fn led_state_get(state: web::Data<Mutex<LumeState>>) -> impl Responder {
    let state = state.lock().unwrap();
    HttpResponse::Ok().json(LedStateResponse {
        led_state: state.is_on,
    })
}

#[post("/led/state/on")]
pub async fn led_state_on(
    lume_state: web::Data<Mutex<LumeState>>,
    effect_queue: web::Data<EffectQueue>,
) -> impl Responder {
    LumeService::new(&lume_state, &effect_queue).set_on_off(true);
    let state = lume_state.lock().unwrap();
    HttpResponse::Ok().json(LedStateResponse {
        led_state: state.is_on,
    })
}

#[post("/led/state/off")]
pub async fn led_state_off(
    lume_state: web::Data<Mutex<LumeState>>,
    effect_queue: web::Data<EffectQueue>,
) -> impl Responder {
    LumeService::new(&lume_state, &effect_queue).set_on_off(false);
    let state = lume_state.lock().unwrap();
    HttpResponse::Ok().json(LedStateResponse {
        led_state: state.is_on,
    })
}
