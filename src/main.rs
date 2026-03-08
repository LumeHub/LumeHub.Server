mod color;
mod controller;
mod effects;
mod endpoints;
mod lume_service;
mod settings;
mod state;

use actix_web::{App, HttpServer, web};
use effects::EffectQueue;
use settings::Settings;

use controller::create_controller;
use endpoints::google::commands::CommandDispatcher;

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
            ))
        }
    };

    let (effect_queue, rx_effect_processor) = EffectQueue::new();
    let effect_registry = effects::build_registry();

    let lume_state = state::lume_app_data();
    let command_dispatcher = CommandDispatcher::new();

    controller::effect_processor::spawn(led, rx_effect_processor);

    let ip_address = app_settings.server.ip_address;
    let port = app_settings.server.port;

    println!("Server running at http://{}:{}", ip_address, port);

    let effects_config = app_settings.effects.clone();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(effect_queue.clone()))
            .app_data(web::Data::new(effect_registry.clone()))
            .app_data(web::Data::new(effects_config.clone()))
            .app_data(lume_state.clone())
            .app_data(web::Data::new(command_dispatcher.clone()))
            .configure(endpoints::effects::config)
            .configure(endpoints::legacy::config)
            .configure(endpoints::google::config)
            .configure(endpoints::oauth::config)
    })
    .bind((ip_address.as_str(), port))?
    .run()
    .await
}
