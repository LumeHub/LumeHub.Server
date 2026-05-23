use std::sync::Arc;

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
