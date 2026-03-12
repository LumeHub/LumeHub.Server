use crate::Rgb;

pub struct DeviceState {
    pub on: bool,
    pub brightness: u8,
    pub color: Rgb,
}

impl Default for DeviceState {
    fn default() -> Self {
        Self {
            on: true,
            brightness: 255,
            color: Rgb::BLACK,
        }
    }
}
