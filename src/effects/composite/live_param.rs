use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::color::Rgb;

pub const BRIGHTNESS: &str = "brightness";
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
    colors: HashMap<String, LiveParam<Rgb>>,
    scalars: HashMap<String, LiveParam<f32>>,
}

impl ParameterBus {
    pub fn new(brightness: f32) -> Self {
        let mut bus = ParameterBus {
            colors: HashMap::new(),
            scalars: HashMap::new(),
        };
        bus.register_scalar(BRIGHTNESS.to_string(), brightness, 1.0);
        bus
    }

    pub fn register_color(&mut self, name: String, initial: Rgb, speed: f32) {
        self.colors.insert(name, LiveParam::new(initial, speed));
    }

    pub fn register_scalar(&mut self, name: String, initial: f32, speed: f32) {
        self.scalars.insert(name, LiveParam::new(initial, speed));
    }

    pub fn get_color(&self, name: &str) -> Option<LiveParam<Rgb>> {
        self.colors.get(name).cloned()
    }

    pub fn get_scalar(&self, name: &str) -> Option<LiveParam<f32>> {
        self.scalars.get(name).cloned()
    }

    pub fn set_color(&self, name: &str, value: Rgb) {
        if let Some(p) = self.colors.get(name) {
            p.set(value);
        }
    }

    pub fn set_scalar(&self, name: &str, value: f32) {
        if let Some(p) = self.scalars.get(name) {
            p.set(value);
        }
    }

    pub fn tick(&self) {
        self.colors.values().for_each(|p| p.tick());
        self.scalars.values().for_each(|p| p.tick());
    }
}
