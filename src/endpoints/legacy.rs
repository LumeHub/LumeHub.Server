use actix_web::{HttpResponse, Responder, post};
use serde::Serialize;

#[derive(Serialize)]
struct LedState {
    #[serde(rename = "ledState")]
    led_state: bool,
}

#[post("/led/state/get")]
pub async fn led_state_get() -> impl Responder {
    let led_state = LedState { led_state: true };
    HttpResponse::Ok().json(led_state)
}
