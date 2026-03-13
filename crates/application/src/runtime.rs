use std::collections::HashMap;
use std::sync::Arc;

use domain::Rgb;

use crate::{BusProxy, SceneSnapshot, SignalOverrides, SignalValue};

pub trait SceneRuntime: Send + Sync {
    fn set_color(&self, color: Rgb);
    fn set_brightness(&self, brightness: u8);
    fn set_on_off(&self, on: bool);
    fn halt(&self);
    fn stop_effect(&self);
    fn start_color_loop(&self, _duration: u64) {}
    fn start_sleep(&self, _duration: u64) {}
    fn start_wake(&self, _duration: u64) {}
    fn set_signal(&self, name: &str, value: SignalValue) -> Result<(), String>;
    fn attach_bus(&self, bus: Arc<dyn BusProxy>, effect_name: String);
    fn snapshot(&self) -> SceneSnapshot;
    fn bus_colors(&self) -> HashMap<String, Rgb>;
    fn signal_overrides(&self) -> SignalOverrides;
}
