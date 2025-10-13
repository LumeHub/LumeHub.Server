mod controller;
mod endpoints;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use config::{Config, Environment, File};

use controller::{LedControllerConfig, color::Rgb, create_controller};
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
    let settings = Config::builder()
        .add_source(File::with_name("config.toml").required(false))
        .add_source(Environment::with_prefix("LUMEHUB"))
        .build()
        .expect("Failed to load configuration");

    let led_config = settings
        .try_deserialize::<LedControllerConfig>()
        .expect("Failed to deserialize LED controller configuration");

    let mut led = match create_controller(&led_config) {
        Ok(controller) => controller,
        Err(e) => {
            eprintln!("Error creating LED controller: {}", e);
            // Fallback to a console controller or exit
            Box::new(controller::drivers::console::Console::new(
                led_config.pixel_count,
            ))
        }
    };
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(legacy::led_state_get)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    println!("Server running at http://127.0.0.1:8080");
    Ok(())
}
