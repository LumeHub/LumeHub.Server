use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use chrono::Utc;
use engine::{EffectQueue, RenderCommand};

use effects::bus::PRIMARY_COLOR;

use crate::color_loop::ColorLoop;
use crate::sleep::Sleep;
use crate::wake::Wake;
use application::{BusProxy, SceneRuntime, SceneSnapshot, SignalOverrides, SignalValue};
use domain::Rgb;

struct SceneState {
    on: bool,
    brightness: u8,
    color: Rgb,
    active_effect: Option<String>,
    light_effect_end_unix_timestamp_sec: Option<u64>,
    active_bus: Option<Arc<dyn BusProxy>>,
    signal_colors: HashMap<String, Rgb>,
    signal_scripts: HashMap<String, String>,
}

impl Default for SceneState {
    fn default() -> Self {
        Self {
            on: true,
            brightness: 255,
            color: Rgb::BLACK,
            active_effect: None,
            light_effect_end_unix_timestamp_sec: None,
            active_bus: None,
            signal_colors: HashMap::new(),
            signal_scripts: HashMap::new(),
        }
    }
}

pub struct RenderTaskRuntime {
    queue: EffectQueue,
    state: Arc<Mutex<SceneState>>,
}

impl RenderTaskRuntime {
    pub fn new(queue: EffectQueue) -> Self {
        Self {
            queue,
            state: Arc::new(Mutex::new(SceneState::default())),
        }
    }
}

impl SceneRuntime for RenderTaskRuntime {
    fn set_color(&self, color: Rgb) {
        let mut state = self.state.lock().unwrap();
        state.color = color;
        state.on = true;
        if let Some(bus) = &state.active_bus {
            bus.set_color(PRIMARY_COLOR, color);
            return;
        }
        state.active_effect = None;
        state.light_effect_end_unix_timestamp_sec = None;
        state.active_bus = None;
        state.signal_scripts.remove(PRIMARY_COLOR);
        state.signal_colors.remove(PRIMARY_COLOR);
        self.queue.send(RenderCommand::SetColor(color));
    }

    fn set_brightness(&self, brightness: u8) {
        let mut state = self.state.lock().unwrap();
        state.brightness = brightness;
        if let Some(bus) = &state.active_bus {
            bus.set_brightness(brightness as f32);
            return;
        }
        state.active_effect = None;
        state.light_effect_end_unix_timestamp_sec = None;
        self.queue.send(RenderCommand::SetBrightness(brightness));
    }

    fn set_on_off(&self, on: bool) {
        let mut state = self.state.lock().unwrap();
        state.on = on;
        if let Some(bus) = &state.active_bus {
            bus.set_brightness(if on { state.brightness as f32 } else { 0.0 });
            return;
        }
        state.active_effect = None;
        state.light_effect_end_unix_timestamp_sec = None;
        state.active_bus = None;
        self.queue.send(RenderCommand::SetOnOff(on));
    }

    fn halt(&self) {
        let mut state = self.state.lock().unwrap();
        state.active_effect = None;
        state.light_effect_end_unix_timestamp_sec = None;
        state.active_bus = None;
        self.queue.send(RenderCommand::Halt);
    }

    fn stop_effect(&self) {
        let mut state = self.state.lock().unwrap();
        state.active_effect = None;
        state.light_effect_end_unix_timestamp_sec = None;
        state.active_bus = None;
        let on = state.on;
        self.queue.send(RenderCommand::SetOnOff(on));
    }

    fn start_color_loop(&self, duration: u64) {
        let mut state = self.state.lock().unwrap();
        state.on = true;
        state.active_effect = Some("colorLoop".to_string());
        state.light_effect_end_unix_timestamp_sec = Some(Utc::now().timestamp() as u64 + duration);
        state.active_bus = None;
        let start_color = state.color;
        self.queue.enqueue(Box::new(ColorLoop {
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

    fn start_sleep(&self, duration: u64) {
        let mut state = self.state.lock().unwrap();
        state.on = true;
        state.active_effect = Some("sleep".to_string());
        state.light_effect_end_unix_timestamp_sec = Some(Utc::now().timestamp() as u64 + duration);
        state.active_bus = None;
        let start_brightness = state.brightness;
        self.queue.enqueue(Box::new(Sleep {
            duration,
            start_brightness,
            target_color: Rgb::BLACK,
        }));
    }

    fn start_wake(&self, duration: u64) {
        let mut state = self.state.lock().unwrap();
        state.on = true;
        state.active_effect = Some("wake".to_string());
        state.light_effect_end_unix_timestamp_sec = Some(Utc::now().timestamp() as u64 + duration);
        state.active_bus = None;
        let end_brightness = state.brightness;
        let start_color = state.color;
        self.queue.enqueue(Box::new(Wake {
            duration,
            end_brightness,
            start_color,
        }));
    }

    fn set_signal(&self, name: &str, value: SignalValue) -> Result<(), String> {
        let mut state = self.state.lock().unwrap();
        match value {
            SignalValue::Color(color) => {
                state.signal_scripts.remove(name);
                if name == PRIMARY_COLOR {
                    state.color = color;
                } else {
                    state.signal_colors.insert(name.to_string(), color);
                }
                if let Some(bus) = &state.active_bus {
                    bus.set_color(name, color);
                }
            }
            SignalValue::Script(code) => {
                if let Some(bus) = &state.active_bus {
                    bus.set_animated(name, &code)?;
                }
                state.signal_scripts.insert(name.to_string(), code);
            }
        }
        Ok(())
    }

    fn attach_bus(&self, bus: Arc<dyn BusProxy>, effect_name: String) {
        let mut state = self.state.lock().unwrap();
        state.active_effect = Some(effect_name);
        state.light_effect_end_unix_timestamp_sec = None;
        state.active_bus = Some(bus);
    }

    fn snapshot(&self) -> SceneSnapshot {
        let state = self.state.lock().unwrap();
        SceneSnapshot {
            on: state.on,
            brightness: state.brightness,
            color: state.color,
            active_effect: state.active_effect.clone(),
            light_effect_end_unix_timestamp_sec: state.light_effect_end_unix_timestamp_sec,
        }
    }

    fn bus_colors(&self) -> HashMap<String, Rgb> {
        let state = self.state.lock().unwrap();
        if let Some(bus) = &state.active_bus {
            bus.all_colors()
        } else {
            HashMap::new()
        }
    }

    fn signal_overrides(&self) -> SignalOverrides {
        let state = self.state.lock().unwrap();
        SignalOverrides {
            colors: state.signal_colors.clone(),
            scripts: state.signal_scripts.clone(),
        }
    }
}
