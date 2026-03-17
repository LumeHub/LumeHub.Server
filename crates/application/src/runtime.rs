use domain::Rgb;

use crate::SceneSnapshot;

pub trait SceneRuntime: Send + Sync {
    fn set_color(&self, color: Rgb);
    fn set_brightness(&self, brightness: u8);
    fn set_on_off(&self, on: bool);
    fn halt(&self);
    fn stop_effect(&self);
    fn start_color_loop(&self, _duration: u64) {}
    fn start_sleep(&self, _duration: u64) {}
    fn start_wake(&self, _duration: u64) {}
    fn reload_active(&self);
    fn snapshot(&self) -> SceneSnapshot;
}
