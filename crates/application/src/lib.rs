use std::collections::HashMap;
use std::sync::Arc;

use domain::Rgb;

pub trait BusProxy: Send + Sync {
    fn set_color(&self, name: &str, color: Rgb);
    fn set_brightness(&self, value: f32);
    fn set_animated(&self, name: &str, code: &str) -> Result<(), String>;
    fn all_colors(&self) -> HashMap<String, Rgb>;
}

pub enum SignalValue {
    Color(Rgb),
    Script(String),
}

pub struct SceneSnapshot {
    pub on: bool,
    pub brightness: u8,
    pub color: Rgb,
    pub active_effect: Option<String>,
    pub light_effect_end_unix_timestamp_sec: Option<u64>,
}

pub struct SignalOverrides {
    pub colors: HashMap<String, Rgb>,
    pub scripts: HashMap<String, String>,
}

pub trait SceneRuntime: Send + Sync {
    fn set_color(&self, color: Rgb);
    fn set_brightness(&self, brightness: u8);
    fn set_on_off(&self, on: bool);
    fn halt(&self);
    fn stop_effect(&self);
    fn start_color_loop(&self, duration: u64);
    fn start_sleep(&self, duration: u64);
    fn start_wake(&self, duration: u64);
    fn set_signal(&self, name: &str, value: SignalValue) -> Result<(), String>;
    fn attach_bus(&self, bus: Arc<dyn BusProxy>, effect_name: String);
    fn snapshot(&self) -> SceneSnapshot;
    fn bus_colors(&self) -> HashMap<String, Rgb>;
    fn signal_overrides(&self) -> SignalOverrides;
}
