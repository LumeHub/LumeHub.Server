use std::sync::{Arc, Mutex};

use application::{SceneRuntime, SceneSnapshot};
use domain::{Rgb, TransitionSpec};
use sqlx::sqlite::SqlitePoolOptions;
use store::Store;

pub struct MockRuntime;

impl SceneRuntime for MockRuntime {
    fn default_spec(&self) -> TransitionSpec {
        TransitionSpec::INSTANT
    }
    fn set_color_with(&self, _: Rgb, _: TransitionSpec) {}
    fn set_brightness_with(&self, _: u8, _: TransitionSpec) {}
    fn set_on_off_with(&self, _: bool, _: TransitionSpec) {}
    fn halt(&self) {}
    fn stop_effect(&self) {}
    fn reload_active_with(&self, _: TransitionSpec) {}
    fn snapshot(&self) -> SceneSnapshot {
        SceneSnapshot {
            on: true,
            brightness: 255,
            color: Rgb::BLACK,
            active_effect: None,
            light_effect_end_unix_timestamp_sec: None,
        }
    }
}

pub async fn make_store() -> Arc<Store> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    Arc::new(Store::from_pool(pool).await.unwrap())
}

pub fn mock_runtime() -> Arc<dyn SceneRuntime> {
    Arc::new(MockRuntime)
}

pub struct DefaultSpecRuntime {
    default: TransitionSpec,
    pub last_color: Mutex<Option<(Rgb, TransitionSpec)>>,
    pub last_brightness: Mutex<Option<(u8, TransitionSpec)>>,
    pub last_on_off: Mutex<Option<(bool, TransitionSpec)>>,
    pub last_reload: Mutex<Option<TransitionSpec>>,
}

impl DefaultSpecRuntime {
    pub fn new(default: TransitionSpec) -> Self {
        Self {
            default,
            last_color: Mutex::new(None),
            last_brightness: Mutex::new(None),
            last_on_off: Mutex::new(None),
            last_reload: Mutex::new(None),
        }
    }
}

impl SceneRuntime for DefaultSpecRuntime {
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
            brightness: 255,
            color: Rgb::BLACK,
            active_effect: None,
            light_effect_end_unix_timestamp_sec: None,
        }
    }
}
