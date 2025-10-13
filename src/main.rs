mod controller;
mod endpoints;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};

use controller::{Controller, color::Rgb, drivers::console::Console};
use endpoints::legacy;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut led = Console::new(10);
    led.fill(Rgb::new(255, 0, 0));
    led.show();
    led.fill(Rgb::new(0, 255, 0));
    led.show();
    led.fill(Rgb::new(0, 0, 255));
    led.show();
    led.fill(Rgb::new(255, 255, 255));
    led.show();
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(legacy::led_state_get)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
