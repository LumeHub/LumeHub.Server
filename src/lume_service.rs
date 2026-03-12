use actix_web::web;
use chrono::Utc;
use std::sync::Mutex;

use engine::RenderCommand;

use crate::effects::EffectQueue;
use crate::effects::color_loop::ColorLoop;
use crate::effects::composite::live_param::PRIMARY_COLOR;
use crate::effects::sleep::Sleep;
use crate::effects::wake::Wake;
use crate::state::LumeState;
use domain::Rgb;

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
        if let Some(bus) = &state.active_param_bus {
            bus.brightness
                .set(if on { state.brightness as f32 } else { 0.0 });
            return;
        }
        state.active_light_effect = None;
        state.light_effect_end_unix_timestamp_sec = None;
        self.effect_queue.send(RenderCommand::SetOnOff(on));
    }

    pub fn set_brightness(&mut self, brightness: u8) {
        let mut state = self.lume_state.lock().unwrap();
        state.brightness = brightness;
        if let Some(bus) = &state.active_param_bus {
            bus.brightness.set(brightness as f32);
            return;
        }
        state.active_light_effect = None;
        state.light_effect_end_unix_timestamp_sec = None;
        self.effect_queue
            .send(RenderCommand::SetBrightness(brightness));
    }

    pub fn set_color(&mut self, color: Rgb) {
        let mut state = self.lume_state.lock().unwrap();
        state.active_color = color;
        state.is_on = true;
        state.active_light_effect = None;
        state.light_effect_end_unix_timestamp_sec = None;
        state.active_param_bus = None;
        state.signal_scripts.remove(PRIMARY_COLOR);
        state.signal_colors.remove(PRIMARY_COLOR);
        self.effect_queue.send(RenderCommand::SetColor(color));
    }

    pub fn start_color_loop(&mut self, duration: u64) {
        let mut state = self.lume_state.lock().unwrap();
        state.is_on = true;
        state.active_light_effect = Some("colorLoop".to_string());
        state.light_effect_end_unix_timestamp_sec = Some(Utc::now().timestamp() as u64 + duration);
        state.active_param_bus = None;
        let start_color = state.active_color;

        self.effect_queue.enqueue(Box::new(ColorLoop {
            duration,
            start_color,
            colors: vec![
                Rgb::new(255, 0, 0),
                Rgb::new(255, 127, 0),
                Rgb::new(255, 255, 0),
                Rgb::new(0, 255, 0),
                Rgb::new(0, 0, 255),
                Rgb::new(75, 0, 130),
                Rgb::new(148, 0, 211),
            ],
        }));
    }

    pub fn start_sleep_effect(&mut self, duration: u64) {
        let mut state = self.lume_state.lock().unwrap();
        state.is_on = true;
        state.active_light_effect = Some("sleep".to_string());
        state.light_effect_end_unix_timestamp_sec = Some(Utc::now().timestamp() as u64 + duration);
        state.active_param_bus = None;
        let start_brightness = state.brightness;
        self.effect_queue.enqueue(Box::new(Sleep {
            duration,
            start_brightness,
            target_color: Rgb::BLACK,
        }));
    }

    pub fn start_wake_effect(&mut self, duration: u64) {
        let mut state = self.lume_state.lock().unwrap();
        state.is_on = true;
        state.active_light_effect = Some("wake".to_string());
        state.light_effect_end_unix_timestamp_sec = Some(Utc::now().timestamp() as u64 + duration);
        state.active_param_bus = None;
        let end_brightness = state.brightness;
        let start_color = state.active_color;

        self.effect_queue.enqueue(Box::new(Wake {
            duration,
            end_brightness,
            start_color,
        }));
    }

    pub fn halt_effect(&mut self) {
        let mut state = self.lume_state.lock().unwrap();
        state.active_light_effect = None;
        state.light_effect_end_unix_timestamp_sec = None;
        state.active_param_bus = None;
        self.effect_queue.send(RenderCommand::Halt);
    }

    pub fn stop_light_effect(&mut self) {
        let mut state = self.lume_state.lock().unwrap();
        state.active_light_effect = None;
        state.light_effect_end_unix_timestamp_sec = None;
        state.active_param_bus = None;
        self.effect_queue.send(RenderCommand::SetOnOff(state.is_on));
    }
}
