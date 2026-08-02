use domain::{Rgb, TransitionSpec};

use crate::SceneSnapshot;

pub trait SceneRuntime: Send + Sync {
    fn default_spec(&self) -> TransitionSpec;

    fn set_color_with(&self, color: Rgb, spec: TransitionSpec);
    fn set_color(&self, color: Rgb) {
        self.set_color_with(color, self.default_spec());
    }

    fn set_brightness_with(&self, brightness: u8, spec: TransitionSpec);
    fn set_brightness(&self, brightness: u8) {
        self.set_brightness_with(brightness, self.default_spec());
    }

    fn set_on_off_with(&self, on: bool, spec: TransitionSpec);
    fn set_on_off(&self, on: bool) {
        self.set_on_off_with(on, self.default_spec());
    }

    fn halt(&self);
    fn stop_effect(&self);

    fn start_color_loop(&self, _duration: u64) {}
    fn start_sleep(&self, _duration: u64) {}
    fn start_wake(&self, _duration: u64) {}

    fn reload_active_with(&self, spec: TransitionSpec);
    fn reload_active(&self) {
        self.reload_active_with(self.default_spec());
    }

    fn snapshot(&self) -> SceneSnapshot;
}
