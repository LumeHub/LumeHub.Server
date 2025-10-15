mod color;
mod controller;
mod effects;
mod endpoints;
mod settings;
mod state;

use std::{sync::Mutex, thread, time::Duration};

use actix_web::{App, HttpServer, web};
use effects::EffectQueue;
use settings::Settings;
use state::LumeState;

use controller::{Controller, create_controller};
use endpoints::legacy;

fn run_effects_thread(
    mut led: Box<dyn Controller>,
    rx: std::sync::mpsc::Receiver<Box<dyn effects::Effect + Send>>,
) {
    thread::spawn(move || {
        loop {
            if let Ok(effect) = rx.recv() {
                for frame in effect.frames(led.as_pixel_slice()) {
                    led.as_pixel_slice_mut().copy_from_slice(&frame);
                    led.show();
                    thread::sleep(Duration::from_millis(10));
                }
            }
        }
    });
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
            ))
        }
    };

    let (effect_queue, rx) = EffectQueue::new();
    run_effects_thread(led, rx);

    let lume_state = web::Data::new(Mutex::new(LumeState::default()));

    let ip_address = app_settings.server.ip_address;
    let port = app_settings.server.port;

    println!("Server running at http://{}:{}", ip_address, port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(effect_queue.clone()))
            .app_data(lume_state.clone())
            .configure(legacy::config)
    })
    .bind((ip_address.as_str(), port))?
    .run()
    .await
}
