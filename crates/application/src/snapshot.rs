use std::collections::HashMap;

use domain::Rgb;
use serde::Serialize;

#[derive(Clone, Serialize)]
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
