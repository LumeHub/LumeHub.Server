use crate::color::Rgb;
use crate::effects::EffectQueue;
use crate::effects::fade_color::FadeColor;
use crate::state::LumeState;
use actix_web::web;
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
        let effect = if on {
            Box::new(FadeColor {
                color: state.active_color.with_brightness(state.brightness),
            })
        } else {
            Box::new(FadeColor {
                color: Rgb::BLACK.with_brightness(state.brightness),
            })
        };
        self.effect_queue.enqueue(effect);
    }

    pub fn set_brightness(&mut self, brightness: u8) {
        let mut state = self.lume_state.lock().unwrap();
        state.brightness = brightness;
        let effect = Box::new(FadeColor {
            color: state.active_color.with_brightness(state.brightness),
        });
        self.effect_queue.enqueue(effect);
    }

    pub fn set_color(&mut self, color: Rgb) {
        let mut state = self.lume_state.lock().unwrap();
        state.active_color = color;
        state.is_on = true; // Setting color implies turning on
        let effect = Box::new(FadeColor {
            color: state.active_color.with_brightness(state.brightness),
        });
        self.effect_queue.enqueue(effect);
    }
}
