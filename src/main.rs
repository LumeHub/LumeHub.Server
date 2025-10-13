mod controller;
mod endpoints;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use config::{Config, Environment, File};
use serde::{Deserialize, Serialize};

use controller::{LedControllerConfig, color::Rgb, create_controller};
use endpoints::legacy;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    pub ip_address: Option<String>,
    pub port: Option<u16>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Settings {
    pub led_controller: LedControllerConfig,
    pub server: ServerConfig,
}

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

    let app_settings = settings
        .try_deserialize::<Settings>()
        .expect("Failed to deserialize application settings");

    let mut led = match create_controller(&app_settings.led_controller) {
        Ok(controller) => controller,
        Err(e) => {
            eprintln!("Error creating LED controller: {}", e);
            // Fallback to a console controller or exit
            Box::new(controller::drivers::console::Console::new(
                app_settings.led_controller.pixel_count,
            ))
        }
    };
    let ip_address = app_settings.server.ip_address.unwrap_or_else(|| "127.0.0.1".to_string());
    let port = app_settings.server.port.unwrap_or(8080);

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(legacy::led_state_get)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((ip_address.as_str(), port))?
    .run()
    .await?;

    println!("Server running at http://{}:{}", ip_address, port);
    Ok(())
}
