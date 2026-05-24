mod scene_runtime;
mod settings;

use std::sync::Arc;

use actix_web::{App, HttpServer, web};
use application::{SceneRuntime, StateEventBus};
use domain::TransitionSpec;
use effects::load_builtins;
use engine::EffectQueue;
use settings::Settings;
use store::Store;

use scene_runtime::RenderTaskRuntime;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_settings = Settings::new().unwrap_or_else(|e| {
        eprintln!("error: failed to load configuration: {}", e);
        std::process::exit(1);
    });

    let led = match drivers::create(&app_settings.led_controller.driver) {
        Ok(controller) => controller,
        Err(e) => {
            eprintln!("Error creating LED controller: {}", e);
            Box::new(drivers::Console::new(
                app_settings.led_controller.driver.pixel_count,
            ))
        }
    };

    let strip_len = app_settings.led_controller.driver.pixel_count;

    let default_spec = TransitionSpec::new(
        app_settings.transitions.default_fade_ms,
        app_settings.transitions.default_curve,
    );

    let (effect_queue, rx_effect_processor) = EffectQueue::new(default_spec);

    let state_file = app_settings.state_dir.join("state.json");
    let initial_state = persistence::load(&state_file).unwrap_or_else(|e| {
        eprintln!("warning: failed to load state: {}", e);
        persistence::PersistedState::default()
    });

    let db_path = app_settings.state_dir.join("lumehub.db");
    let store = Arc::new(Store::open(&db_path).await.unwrap_or_else(|e| {
        eprintln!("error: failed to open database: {}", e);
        std::process::exit(1);
    }));

    let builtins = load_builtins();
    for e in effects::validate_builtin_scripts() {
        eprintln!("warning: {}", e);
    }
    let builtins_data = web::Data::new(builtins.clone());

    let (state_tx, mut state_rx) = tokio::sync::watch::channel(initial_state.clone());
    tokio::spawn(async move {
        loop {
            if state_rx.changed().await.is_err() {
                break;
            }
            let state = state_rx.borrow_and_update().clone();
            if let Ok(json) = serde_json::to_string(&state)
                && let Err(e) = tokio::fs::write(&state_file, json).await
            {
                eprintln!("warning: failed to save state: {}", e);
            }
        }
    });

    let event_bus = StateEventBus::new();
    let runtime: Arc<dyn SceneRuntime> = Arc::new(RenderTaskRuntime::new(
        effect_queue.clone(),
        event_bus.clone(),
        initial_state,
        Some(state_tx),
        Arc::clone(&store),
        builtins,
        strip_len,
    ));

    // Restore active scene from __active__ on startup
    runtime.reload_active();

    #[cfg(feature = "google")]
    let command_dispatcher = api_google::commands::CommandDispatcher::new();

    engine::spawn(Box::new(led), rx_effect_processor);

    let ip_address = app_settings.server.ip_address;
    let port = app_settings.server.port;

    println!("Server running at http://{}:{}", ip_address, port);

    let _mdns = app_settings.mdns.enable.then(|| {
        let mdns_config = discovery::Config {
            name: app_settings.mdns.name.clone(),
            port,
        };
        discovery::advertise(&mdns_config)
            .map_err(|e| eprintln!("warning: mDNS advertisement failed: {}", e))
            .ok()
    });

    HttpServer::new(move || {
        let app = App::new()
            .app_data(web::Data::new(effect_queue.clone()))
            .app_data(web::Data::new(store.clone()))
            .app_data(web::Data::new(strip_len))
            .app_data(builtins_data.clone())
            .app_data(web::Data::from(Arc::clone(&runtime)))
            .app_data(web::Data::new(event_bus.clone()))
            .configure(api::device::config)
            .configure(api::effects::config)
            .configure(api::events::config)
            .configure(api::zones::config)
            // state_stack and active_scene must precede scenes: /scenes/active/* must
            // match before the wildcard /scenes/{id}
            .configure(api::state_stack::config)
            .configure(api::active_scene::config)
            .configure(api::scenes::config)
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
