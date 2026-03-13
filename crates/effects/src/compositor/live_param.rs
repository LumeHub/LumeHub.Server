use std::sync::{Arc, RwLock};

use domain::Rgb;

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
        self.lerp(*target, 1.0 / (steps - 1) as f32)
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

#[derive(Clone)]
pub struct LiveParam<T: Interpolatable> {
    inner: Arc<RwLock<LiveParamInner<T>>>,
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
            inner.current = inner.current.step_toward(&inner.target, inner.speed);
        }
    }
}
