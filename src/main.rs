mod color;
mod controller;
mod endpoints;
mod settings;

use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use settings::Settings;

use controller::{Controller, create_controller};
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
    let app_settings = Settings::new().expect("Failed to load configuration");

    let led = match create_controller(&app_settings.led_controller) {
        Ok(controller) => controller,
        Err(e) => {
            eprintln!("Error creating LED controller: {}", e);
            // Fallback to a console controller or exit
            Box::new(controller::drivers::console::Console::new(
                app_settings.led_controller.pixel_count,
            )) as Box<dyn Controller>
        }
    };

    let ip_address = app_settings.server.ip_address;
    let port = app_settings.server.port;

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
