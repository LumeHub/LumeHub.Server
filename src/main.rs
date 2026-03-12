mod controller;
mod effects;
mod endpoints;
mod scene_runtime;
mod settings;

use std::sync::Arc;

use actix_web::{App, HttpServer, web};
use application::SceneRuntime;
use effects::EffectQueue;
use effects::config::EffectsConfig;
use settings::Settings;

use controller::create_controller;
use endpoints::google::commands::CommandDispatcher;
use scene_runtime::RenderTaskRuntime;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_settings = Settings::new().expect("Failed to load configuration");

    let led = match create_controller(&app_settings.led_controller) {
        Ok(controller) => controller,
        Err(e) => {
            eprintln!("Error creating LED controller: {}", e);
            Box::new(controller::drivers::console::Console::new(
                app_settings.led_controller.pixel_count,
            ))
        }
    };

    let (effect_queue, rx_effect_processor) =
        EffectQueue::new(app_settings.led_controller.crossfade_ms);
    let effect_registry = effects::build_registry();

    let runtime: Arc<dyn SceneRuntime> = Arc::new(RenderTaskRuntime::new(effect_queue.clone()));
    let command_dispatcher = CommandDispatcher::new();

    let crossfade_frames = app_settings.led_controller.crossfade_ms as usize / 16;
    engine::spawn(Box::new(led), rx_effect_processor, crossfade_frames);

    let ip_address = app_settings.server.ip_address;
    let port = app_settings.server.port;

    println!("Server running at http://{}:{}", ip_address, port);

    let effects_config = EffectsConfig::from_dir(&app_settings.config_dir);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(effect_queue.clone()))
            .app_data(web::Data::new(effect_registry.clone()))
            .app_data(web::Data::new(effects_config.clone()))
            .app_data(web::Data::from(Arc::clone(&runtime)))
            .app_data(web::Data::new(command_dispatcher.clone()))
            .configure(endpoints::device::config)
            .configure(endpoints::effects::config)
            .configure(endpoints::legacy::config)
            .configure(endpoints::google::config)
            .configure(endpoints::oauth::config)
    })
    .bind((ip_address.as_str(), port))?
    .run()
    .await
}
