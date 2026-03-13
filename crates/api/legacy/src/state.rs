use actix_web::{HttpResponse, Responder, post, web};
use application::SceneRuntime;
use serde::Serialize;

#[derive(Serialize)]
pub struct LedStateResponse {
    #[serde(rename = "ledState")]
    pub led_state: bool,
}

#[post("/led/state/get")]
pub async fn led_state_get(runtime: web::Data<dyn SceneRuntime>) -> impl Responder {
    HttpResponse::Ok().json(LedStateResponse {
        led_state: runtime.snapshot().on,
    })
}

#[post("/led/state/on")]
pub async fn led_state_on(runtime: web::Data<dyn SceneRuntime>) -> impl Responder {
    runtime.set_on_off(true);
    HttpResponse::Ok().json(LedStateResponse {
        led_state: runtime.snapshot().on,
    })
}

#[post("/led/state/off")]
pub async fn led_state_off(runtime: web::Data<dyn SceneRuntime>) -> impl Responder {
    runtime.set_on_off(false);
    HttpResponse::Ok().json(LedStateResponse {
        led_state: runtime.snapshot().on,
    })
}
