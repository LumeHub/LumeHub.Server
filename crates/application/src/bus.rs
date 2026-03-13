use std::collections::HashMap;

use domain::Rgb;

use crate::SignalError;

pub trait BusProxy: Send + Sync {
    fn set_color(&self, name: &str, color: Rgb);
    fn set_brightness(&self, value: f32);
    fn set_animated(&self, name: &str, code: &str) -> Result<(), SignalError>;
    fn all_colors(&self) -> HashMap<String, Rgb>;
}

pub enum SignalValue {
    Color(Rgb),
    Script(String),
}
