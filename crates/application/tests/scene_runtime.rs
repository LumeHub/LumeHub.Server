use std::sync::Mutex;

use application::{SceneRuntime, SceneSnapshot};
use domain::{Rgb, TransitionCurve, TransitionSpec};

#[derive(Default)]
struct Recorder {
    default: TransitionSpec,
    last_color: Mutex<Option<(Rgb, TransitionSpec)>>,
    last_brightness: Mutex<Option<(u8, TransitionSpec)>>,
    last_on_off: Mutex<Option<(bool, TransitionSpec)>>,
    last_reload: Mutex<Option<TransitionSpec>>,
}

impl Recorder {
    fn new(default: TransitionSpec) -> Self {
        Self {
            default,
            ..Default::default()
        }
    }
}

impl SceneRuntime for Recorder {
    fn default_spec(&self) -> TransitionSpec {
        self.default
    }
    fn set_color_with(&self, color: Rgb, spec: TransitionSpec) {
        *self.last_color.lock().unwrap() = Some((color, spec));
    }
    fn set_brightness_with(&self, b: u8, spec: TransitionSpec) {
        *self.last_brightness.lock().unwrap() = Some((b, spec));
    }
    fn set_on_off_with(&self, on: bool, spec: TransitionSpec) {
        *self.last_on_off.lock().unwrap() = Some((on, spec));
    }
    fn halt(&self) {}
    fn stop_effect(&self) {}
    fn reload_active_with(&self, spec: TransitionSpec) {
        *self.last_reload.lock().unwrap() = Some(spec);
    }
    fn snapshot(&self) -> SceneSnapshot {
        SceneSnapshot {
            on: true,
            brightness: 0,
            color: Rgb::BLACK,
            active_effect: None,
            light_effect_end_unix_timestamp_sec: None,
        }
    }
}

fn fade() -> TransitionSpec {
    TransitionSpec::new(500, TransitionCurve::EaseInOut)
}

#[test]
fn set_color_delegates_to_set_color_with_using_default_spec() {
    let rec = Recorder::new(fade());
    rec.set_color(Rgb::new(1, 2, 3));
    let (color, spec) = rec.last_color.lock().unwrap().unwrap();
    assert_eq!(color, Rgb::new(1, 2, 3));
    assert_eq!(spec, fade());
}

#[test]
fn set_brightness_delegates_to_set_brightness_with_using_default_spec() {
    let rec = Recorder::new(fade());
    rec.set_brightness(123);
    let (value, spec) = rec.last_brightness.lock().unwrap().unwrap();
    assert_eq!(value, 123);
    assert_eq!(spec, fade());
}

#[test]
fn set_on_off_delegates_to_set_on_off_with_using_default_spec() {
    let rec = Recorder::new(fade());
    rec.set_on_off(false);
    let (value, spec) = rec.last_on_off.lock().unwrap().unwrap();
    assert!(!value);
    assert_eq!(spec, fade());
}

#[test]
fn reload_active_delegates_to_reload_active_with_using_default_spec() {
    let rec = Recorder::new(fade());
    rec.reload_active();
    assert_eq!(rec.last_reload.lock().unwrap().unwrap(), fade());
}

#[test]
fn explicit_with_call_overrides_default_spec() {
    let rec = Recorder::new(TransitionSpec::INSTANT);
    let custom = TransitionSpec::new(900, TransitionCurve::EaseIn);
    rec.set_color_with(Rgb::new(10, 20, 30), custom);
    assert_eq!(rec.last_color.lock().unwrap().unwrap().1, custom);
}
