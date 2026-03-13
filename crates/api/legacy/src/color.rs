use actix_web::{HttpResponse, Responder, post, web};
use application::SceneRuntime;
use domain::Rgb;
use serde::{Deserialize, Serialize};

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
pub async fn led_get_color(runtime: web::Data<dyn SceneRuntime>) -> impl Responder {
    HttpResponse::Ok().json(LedColorResponse::from(runtime.snapshot().color))
}

#[post("/led/setColor")]
pub async fn led_set_color(
    runtime: web::Data<dyn SceneRuntime>,
    body: web::Bytes,
) -> impl Responder {
    let req: SetColorRequest = match serde_json::from_slice(&body) {
        Ok(r) => r,
        Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
    };
    let color = Rgb {
        r: req.red,
        g: req.green,
        b: req.blue,
    };
    runtime.set_color(color);
    HttpResponse::Ok().json(LedColorResponse::from(runtime.snapshot().color))
}
