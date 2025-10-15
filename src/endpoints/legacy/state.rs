use std::sync::Mutex;

use crate::{
    color::Rgb,
    effects::{EffectQueue, fade_color::FadeColor},
    state::LumeState,
};
use actix_web::{HttpResponse, Responder, post, web};
use serde::Serialize;

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
    state: web::Data<Mutex<LumeState>>,
    effect_queue: web::Data<EffectQueue>,
) -> impl Responder {
    let mut state = state.lock().unwrap();
    state.is_on = true;
    let effect = FadeColor {
        color: state.active_color,
    };
    effect_queue.enqueue(Box::new(effect));
    HttpResponse::Ok().json(LedStateResponse {
        led_state: state.is_on,
    })
}

#[post("/led/state/off")]
pub async fn led_state_off(
    state: web::Data<Mutex<LumeState>>,
    effect_queue: web::Data<EffectQueue>,
) -> impl Responder {
    let mut state = state.lock().unwrap();
    state.is_on = false;
    let effect = FadeColor { color: Rgb::BLACK };
    effect_queue.enqueue(Box::new(effect));
    HttpResponse::Ok().json(LedStateResponse {
        led_state: state.is_on,
    })
}
