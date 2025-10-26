use crate::color::Rgb;
use crate::effects::EffectQueue;
use crate::effects::color_loop::ColorLoop;
use crate::effects::fade_color::FadeColor;
use crate::effects::sleep::Sleep;
use crate::effects::wake::Wake;
use crate::state::LumeState;
use actix_web::web;
use chrono::Utc;
use std::sync::Mutex;

pub struct LumeService<'a> {
    pub lume_state: &'a web::Data<Mutex<LumeState>>,
    effect_queue: &'a web::Data<EffectQueue>,
}

impl<'a> LumeService<'a> {
    pub fn new(
        lume_state: &'a web::Data<Mutex<LumeState>>,
        effect_queue: &'a web::Data<EffectQueue>,
    ) -> Self {
        LumeService {
            lume_state,
            effect_queue,
        }
    }

    pub fn set_on_off(&mut self, on: bool) {
        let mut state = self.lume_state.lock().unwrap();
        state.is_on = on;
        state.active_light_effect = None;
        state.light_effect_end_unix_timestamp_sec = None;
        let effect = Box::new(FadeColor {
            color: if on { state.active_color } else { Rgb::BLACK }
                .with_brightness(state.brightness),
        });
        self.effect_queue.enqueue(effect);
    }

    pub fn set_brightness(&mut self, brightness: u8) {
        let mut state = self.lume_state.lock().unwrap();
        state.brightness = brightness;
        state.active_light_effect = None;
        state.light_effect_end_unix_timestamp_sec = None;
        let effect = Box::new(FadeColor {
            color: state.active_color.with_brightness(state.brightness),
        });
        self.effect_queue.enqueue(effect);
    }

    pub fn set_color(&mut self, color: Rgb) {
        let mut state = self.lume_state.lock().unwrap();
        state.active_color = color;
        state.is_on = true;
        state.active_light_effect = None;
        state.light_effect_end_unix_timestamp_sec = None;
        let effect = Box::new(FadeColor {
            color: state.active_color.with_brightness(state.brightness),
        });
        self.effect_queue.enqueue(effect);
    }

    pub fn start_color_loop(&mut self, duration: u64) {
        let mut state = self.lume_state.lock().unwrap();
        state.is_on = true;
        state.active_light_effect = Some("colorLoop".to_string());
        state.light_effect_end_unix_timestamp_sec = Some(Utc::now().timestamp() as u64 + duration);
        let start_color = state.active_color;

        self.effect_queue.enqueue(Box::new(FadeColor {
            color: start_color.with_brightness(state.brightness),
        }));
        self.effect_queue.enqueue(Box::new(ColorLoop {
            duration,
            start_color,
            colors: vec![
                Rgb::new(255, 0, 0),   // Red
                Rgb::new(255, 127, 0), // Orange
                Rgb::new(255, 255, 0), // Yellow
                Rgb::new(0, 255, 0),   // Green
                Rgb::new(0, 0, 255),   // Blue
                Rgb::new(75, 0, 130),  // Indigo
                Rgb::new(148, 0, 211), // Violet
            ],
        }));
    }

    pub fn start_sleep_effect(&mut self, duration: u64) {
        let mut state = self.lume_state.lock().unwrap();
        state.is_on = true;
        state.active_light_effect = Some("sleep".to_string());
        state.light_effect_end_unix_timestamp_sec = Some(Utc::now().timestamp() as u64 + duration);
        let start_brightness = state.brightness;
        let target_color = Rgb::BLACK;
        self.effect_queue.enqueue(Box::new(FadeColor {
            color: state.active_color.with_brightness(start_brightness),
        }));
        self.effect_queue.enqueue(Box::new(Sleep {
            duration,
            start_brightness,
            target_color,
        }));
    }

    pub fn start_wake_effect(&mut self, duration: u64) {
        let mut state = self.lume_state.lock().unwrap();
        state.is_on = true;
        state.active_light_effect = Some("wake".to_string());
        state.light_effect_end_unix_timestamp_sec = Some(Utc::now().timestamp() as u64 + duration);
        let end_brightness = state.brightness;
        let start_color = state.active_color;

        self.effect_queue.enqueue(Box::new(FadeColor {
            color: start_color.with_brightness(0),
        }));
        self.effect_queue.enqueue(Box::new(Wake {
            duration,
            end_brightness,
            start_color,
        }));
    }

    pub fn stop_light_effect(&mut self) {
        let mut state = self.lume_state.lock().unwrap();
        state.active_light_effect = None;
        state.light_effect_end_unix_timestamp_sec = None;
        let effect = Box::new(FadeColor {
            color: state.active_color.with_brightness(state.brightness),
        });
        self.effect_queue.enqueue(effect);
    }
}
