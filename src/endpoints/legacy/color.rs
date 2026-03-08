use std::sync::Mutex;

use actix_web::{HttpResponse, Responder, post, web};
use serde::{Deserialize, Serialize};

use crate::color::Rgb;
use crate::effects::EffectQueue;
use crate::lume_service::LumeService;
use crate::state::LumeState;

#[derive(Deserialize)]
pub struct SetColorRequest {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Serialize)]
pub struct LedColorResponse {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
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

#[post("/led/getColor")]
pub async fn led_get_color(state: web::Data<Mutex<LumeState>>) -> impl Responder {
    let state = state.lock().unwrap();
    HttpResponse::Ok().json(LedColorResponse::from(state.active_color))
}

#[post("/led/setColor")]
pub async fn led_set_color(
    lume_state: web::Data<Mutex<LumeState>>,
    effect_queue: web::Data<EffectQueue>,
    req: web::Json<SetColorRequest>,
) -> impl Responder {
    let color = Rgb {
        r: req.red,
        g: req.green,
        b: req.blue,
    };
    LumeService::new(&lume_state, &effect_queue).set_color(color);
    let state = lume_state.lock().unwrap();
    HttpResponse::Ok().json(LedColorResponse::from(state.active_color))
}
