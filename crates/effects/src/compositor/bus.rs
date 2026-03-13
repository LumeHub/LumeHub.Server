use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

use rhai::{AST, Engine, Scope};

use application::{BusProxy, SignalError};
use domain::Rgb;

use super::live_param::LiveParam;
use crate::error::EffectError;
use crate::rhai::{make_signal_engine, parse_color};

pub const PRIMARY_COLOR: &str = "primary_color";
pub(crate) const SECONDARY_COLOR: &str = "secondary_color";
pub(crate) const DEFAULT_SIGNAL_SPEED: f32 = 5.0;

struct SignalScript {
    engine: Engine,
    ast: AST,
    live: LiveParam<Rgb>,
    frame: AtomicU64,
    from_color: Rgb,
    fade_frames: usize,
}

impl SignalScript {
    fn new(code: &str, from_color: Rgb, fade_frames: usize) -> Result<Arc<Self>, EffectError> {
        let engine = make_signal_engine();
        let ast = engine.compile(code).map_err(EffectError::ScriptCompile)?;
        Ok(Arc::new(Self {
            engine,
            ast,
            live: LiveParam::new(from_color, f32::MAX),
            frame: AtomicU64::new(0),
            from_color,
            fade_frames,
        }))
    }

    fn tick(&self) {
        let frame = self.frame.fetch_add(1, Ordering::Relaxed);
        let mut scope = Scope::new();
        scope.push("time", frame as f64);
        scope.push("frame", frame as i64);
        scope.push("pi", std::f64::consts::PI);
        let script_color = self
            .engine
            .eval_ast_with_scope::<rhai::Dynamic>(&mut scope, &self.ast)
            .map(parse_color)
            .unwrap_or(Rgb::BLACK);
        let color = if self.fade_frames > 0 && (frame as usize) < self.fade_frames {
            let t = (frame as f32 + 1.0) / self.fade_frames as f32;
            self.from_color.lerp(script_color, t)
        } else {
            script_color
        };
        self.live.set_immediate(color);
    }
}

pub struct ParameterBus {
    pub brightness: LiveParam<f32>,
    colors: RwLock<HashMap<String, LiveParam<Rgb>>>,
    animated: RwLock<HashMap<String, Arc<SignalScript>>>,
    transition_frames: usize,
}

impl ParameterBus {
    pub fn new(brightness: f32, brightness_speed: f32) -> Self {
        let transition_frames = (255.0 / brightness_speed.max(1.0)).round() as usize;
        ParameterBus {
            brightness: LiveParam::new(brightness, brightness_speed),
            colors: RwLock::new(HashMap::new()),
            animated: RwLock::new(HashMap::new()),
            transition_frames,
        }
    }

    pub fn register_color(&mut self, name: String, initial: Rgb, speed: f32) {
        self.colors
            .write()
            .unwrap()
            .insert(name, LiveParam::new(initial, speed));
    }

    pub fn all_colors(&self) -> HashMap<String, Rgb> {
        self.colors
            .read()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.clone(), v.get()))
            .collect()
    }

    pub fn set_color(&self, name: &str, value: Rgb) {
        let was_animated = self.animated.write().unwrap().remove(name).is_some();
        let mut colors = self.colors.write().unwrap();
        if was_animated {
            // Animation params snap immediately; create a normal-speed param to fade from current.
            let current = colors.get(name).map(|p| p.get()).unwrap_or(Rgb::BLACK);
            let param = LiveParam::new(current, DEFAULT_SIGNAL_SPEED);
            param.set(value);
            colors.insert(name.to_string(), param);
        } else {
            colors
                .entry(name.to_string())
                .or_insert_with(|| LiveParam::new(value, DEFAULT_SIGNAL_SPEED))
                .set(value);
        }
    }

    pub fn set_animated(&self, name: &str, code: &str) -> Result<(), EffectError> {
        let current = self
            .colors
            .read()
            .unwrap()
            .get(name)
            .map(|p| p.get())
            .unwrap_or(Rgb::BLACK);
        let script = SignalScript::new(code, current, self.transition_frames)?;
        let live = script.live.clone();
        self.colors.write().unwrap().insert(name.to_string(), live);
        self.animated
            .write()
            .unwrap()
            .insert(name.to_string(), script);
        Ok(())
    }

    pub fn tick(&self) {
        self.brightness.tick();
        self.animated
            .read()
            .unwrap()
            .values()
            .for_each(|s| s.tick());
        self.colors.read().unwrap().values().for_each(|p| p.tick());
    }
}

impl BusProxy for ParameterBus {
    fn set_color(&self, name: &str, color: Rgb) {
        ParameterBus::set_color(self, name, color);
    }

    fn set_brightness(&self, value: f32) {
        self.brightness.set(value);
    }

    fn set_animated(&self, name: &str, code: &str) -> Result<(), SignalError> {
        ParameterBus::set_animated(self, name, code)
            .map_err(|e| SignalError::ScriptCompile(e.to_string()))
    }

    fn all_colors(&self) -> HashMap<String, Rgb> {
        ParameterBus::all_colors(self)
    }
}
