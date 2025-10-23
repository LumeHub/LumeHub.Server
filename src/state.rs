use crate::color::Rgb;
use serde::Serialize;
use std::sync::Mutex;
use actix_web::web;

#[derive(Serialize, Clone, Copy)]
pub struct LumeState {
    pub active_color: Rgb,
    pub is_on: bool,
}

impl Default for LumeState {
    fn default() -> Self {
        Self {
            active_color: Rgb::BLACK,
            is_on: true,
        }
    }
}

pub fn lume_app_data() -> web::Data<Mutex<LumeState>> {
    web::Data::new(Mutex::new(LumeState::default()))
}
