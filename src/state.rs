use std::collections::HashMap;
use std::sync::Arc;

use serde::Serialize;

use crate::effects::composite::ParameterBus;
use domain::Rgb;

#[derive(Serialize, Clone)]
pub struct LumeState {
    pub active_color: Rgb,
    /// Persists static named color signals across preset switches.
    /// primary_color is NOT stored here — use active_color for that.
    pub signal_colors: HashMap<String, Rgb>,
    /// Persists animated color signal scripts across preset switches.
    pub signal_scripts: HashMap<String, String>,
    pub is_on: bool,
    pub brightness: u8,
    pub active_light_effect: Option<String>,
    pub light_effect_end_unix_timestamp_sec: Option<u64>,
    #[serde(skip)]
    pub active_param_bus: Option<Arc<ParameterBus>>,
}

impl Default for LumeState {
    fn default() -> Self {
        Self {
            active_color: Rgb::BLACK,
            signal_colors: HashMap::new(),
            signal_scripts: HashMap::new(),
            is_on: true,
            brightness: 255,
            active_light_effect: None,
            light_effect_end_unix_timestamp_sec: None,
            active_param_bus: None,
        }
    }
}
