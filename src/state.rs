use std::sync::{Arc, Mutex};

use actix_web::web;
use serde::Serialize;

use crate::color::Rgb;
use crate::effects::composite::ParameterBus;

#[derive(Serialize, Clone)]
pub struct LumeState {
    pub active_color: Rgb,
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
            is_on: true,
            brightness: 100,
            active_light_effect: None,
            light_effect_end_unix_timestamp_sec: None,
            active_param_bus: None,
        }
    }
}

pub fn lume_app_data() -> web::Data<Mutex<LumeState>> {
    web::Data::new(Mutex::new(LumeState::default()))
}
