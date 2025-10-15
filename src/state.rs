use crate::color::Rgb;
use serde::Serialize;

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
