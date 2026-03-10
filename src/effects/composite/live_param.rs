use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::signal_script::SignalScript;

use crate::color::Rgb;

pub const PRIMARY_COLOR: &str = "primary_color";
pub const SECONDARY_COLOR: &str = "secondary_color";

pub trait Interpolatable: Clone + Send + Sync + 'static {
    fn step_toward(&self, target: &Self, speed: f32) -> Self;
    fn is_reached(&self, target: &Self) -> bool;
}

impl Interpolatable for Rgb {
    fn step_toward(&self, target: &Rgb, speed: f32) -> Rgb {
        let dist = self.distance(*target);
        if dist == 0 {
            return *target;
        }
        let step_size = speed.max(1.0) as u8;
        let steps = ((dist as f32 / step_size as f32).ceil() as usize).max(1);
        if steps == 1 {
            return *target;
        }
        let t = 1.0_f32 / (steps - 1) as f32;
        self.lerp(*target, t)
    }

    fn is_reached(&self, target: &Rgb) -> bool {
        self.distance(*target) == 0
    }
}

impl Interpolatable for f32 {
    fn step_toward(&self, target: &f32, speed: f32) -> f32 {
        let diff = target - self;
        if diff.abs() <= speed {
            *target
        } else {
            self + diff.signum() * speed
        }
    }

    fn is_reached(&self, target: &f32) -> bool {
        (self - target).abs() < f32::EPSILON
    }
}

struct LiveParamInner<T> {
    current: T,
    target: T,
    speed: f32,
}

pub struct LiveParam<T: Interpolatable> {
    inner: Arc<RwLock<LiveParamInner<T>>>,
}

impl<T: Interpolatable> Clone for LiveParam<T> {
    fn clone(&self) -> Self {
        LiveParam {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T: Interpolatable> LiveParam<T> {
    pub fn new(initial: T, speed: f32) -> Self {
        LiveParam {
            inner: Arc::new(RwLock::new(LiveParamInner {
                current: initial.clone(),
                target: initial,
                speed,
            })),
        }
    }

    pub fn set(&self, target: T) {
        self.inner.write().unwrap().target = target;
    }

    pub fn set_immediate(&self, value: T) {
        let mut inner = self.inner.write().unwrap();
        inner.current = value.clone();
        inner.target = value;
    }

    pub fn get(&self) -> T {
        self.inner.read().unwrap().current.clone()
    }

    pub fn tick(&self) {
        let mut inner = self.inner.write().unwrap();
        if !inner.current.is_reached(&inner.target) {
            let next = inner.current.step_toward(&inner.target, inner.speed);
            inner.current = next;
        }
    }
}

pub struct ParameterBus {
    pub brightness: LiveParam<f32>,
    /// Rhai function definitions prepended to every script layer as a prelude.
    pub prelude: String,
    colors: RwLock<HashMap<String, LiveParam<Rgb>>>,
    animated: RwLock<HashMap<String, Arc<SignalScript>>>,
}

impl ParameterBus {
    pub fn new(brightness: f32, prelude: String) -> Self {
        ParameterBus {
            brightness: LiveParam::new(brightness, 1.0),
            prelude,
            colors: RwLock::new(HashMap::new()),
            animated: RwLock::new(HashMap::new()),
        }
    }

    pub fn register_color(&mut self, name: String, initial: Rgb, speed: f32) {
        self.colors
            .write()
            .unwrap()
            .insert(name, LiveParam::new(initial, speed));
    }

    /// All color signals currently registered, cloned so they stay live.
    pub fn all_colors(&self) -> Vec<(String, LiveParam<Rgb>)> {
        self.colors
            .read()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    /// Set a static color. Clears any animated script for this signal.
    pub fn set_color(&self, name: &str, value: Rgb) {
        let was_animated = self.animated.write().unwrap().remove(name).is_some();
        let mut colors = self.colors.write().unwrap();
        if was_animated {
            // Replace the script's LiveParam with a fresh static one.
            colors.insert(name.to_string(), LiveParam::new(value, 5.0));
        } else if let Some(p) = colors.get(name) {
            p.set(value);
        } else {
            colors.insert(name.to_string(), LiveParam::new(value, 5.0));
        }
    }

    /// Drive a color signal with a Rhai script evaluated every frame.
    /// Script scope: `time` (f64), `frame` (i64), `pi`.
    /// Returns the script code for persistence.
    pub fn set_animated(&self, name: &str, code: &str) -> Result<(), String> {
        let script = SignalScript::new(code)?;
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
        // Animated signals compute their color first so LiveParams are up to date.
        for script in self.animated.read().unwrap().values() {
            script.tick();
        }
        self.colors.read().unwrap().values().for_each(|p| p.tick());
    }
}
