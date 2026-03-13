mod scene_runtime;
mod settings;

use std::sync::Arc;

use actix_web::{App, HttpServer, web};
use application::SceneRuntime;
use effects::config::EffectsConfig;
use engine::EffectQueue;
use settings::Settings;

use scene_runtime::RenderTaskRuntime;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_settings = Settings::new().expect("Failed to load configuration");

    let led = match drivers::create(&app_settings.led_controller.driver) {
        Ok(controller) => controller,
        Err(e) => {
            eprintln!("Error creating LED controller: {}", e);
            Box::new(drivers::Console::new(
                app_settings.led_controller.driver.pixel_count,
            ))
        }
    };

    let (effect_queue, rx_effect_processor) =
        EffectQueue::new(app_settings.led_controller.crossfade_ms);
    let effect_registry = effects::build_registry();

    let runtime: Arc<dyn SceneRuntime> = Arc::new(RenderTaskRuntime::new(effect_queue.clone()));

    #[cfg(feature = "google")]
    let command_dispatcher = api_google::commands::CommandDispatcher::new();

    let crossfade_frames = app_settings.led_controller.crossfade_ms as usize / 16;
    engine::spawn(Box::new(led), rx_effect_processor, crossfade_frames);

    let ip_address = app_settings.server.ip_address;
    let port = app_settings.server.port;

    println!("Server running at http://{}:{}", ip_address, port);

    let effects_config = EffectsConfig::from_dir(&app_settings.config_dir);

    HttpServer::new(move || {
        let app = App::new()
            .app_data(web::Data::new(effect_queue.clone()))
            .app_data(web::Data::new(effect_registry.clone()))
            .app_data(web::Data::new(effects_config.clone()))
            .app_data(web::Data::from(Arc::clone(&runtime)))
            .configure(api::device::config)
            .configure(api::effects::config)
            .configure(api_legacy::config);

        #[cfg(feature = "google")]
        let app = app
            .app_data(web::Data::new(command_dispatcher.clone()))
            .configure(api_google::config)
            .configure(api_google::oauth::config);

        app
    })
    .bind((ip_address.as_str(), port))?
    .run()
    .await
}
