use std::sync::Mutex;

use crate::{
    color::Rgb,
    effects::{EffectQueue, fade_color::FadeColor},
    state::LumeState,
};
use actix_web::{HttpResponse, Responder, post, web};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct LedStateResponse {
    #[serde(rename = "ledState")]
    led_state: bool,
}

#[derive(Serialize)]
struct LedColorResponse {
    red: u8,
    green: u8,
    blue: u8,
}

impl From<Rgb> for LedColorResponse {
    fn from(rgb: Rgb) -> Self {
        Self {
            red: rgb.r,
            green: rgb.g,
            blue: rgb.b,
        }
    }
}

#[derive(Deserialize)]
struct SetColorRequest {
    red: u8,
    green: u8,
    blue: u8,
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

#[post("/led/getColor")]
pub async fn led_get_color(state: web::Data<Mutex<LumeState>>) -> impl Responder {
    let state = state.lock().unwrap();
    HttpResponse::Ok().json(LedColorResponse::from(state.active_color))
}

#[post("/led/setColor")]
pub async fn led_set_color(
    state: web::Data<Mutex<LumeState>>,
    effect_queue: web::Data<EffectQueue>,
    req: web::Json<SetColorRequest>,
) -> impl Responder {
    let mut state = state.lock().unwrap();
    state.active_color = Rgb {
        r: req.red,
        g: req.green,
        b: req.blue,
    };
    if state.is_on {
        let effect = FadeColor {
            color: state.active_color,
        };
        effect_queue.enqueue(Box::new(effect));
    }
    HttpResponse::Ok().json(LedColorResponse::from(state.active_color))
}

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(led_state_get)
        .service(led_state_on)
        .service(led_state_off)
        .service(led_get_color)
        .service(led_set_color);
}
