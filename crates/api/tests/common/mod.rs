use std::sync::Arc;

use application::{SceneRuntime, SceneSnapshot};
use domain::Rgb;
use sqlx::sqlite::SqlitePoolOptions;
use store::Store;

pub struct MockRuntime;

impl SceneRuntime for MockRuntime {
    fn set_color(&self, _: Rgb) {}
    fn set_brightness(&self, _: u8) {}
    fn set_on_off(&self, _: bool) {}
    fn halt(&self) {}
    fn stop_effect(&self) {}
    fn reload_active(&self) {}
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
