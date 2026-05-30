use crate::common::{self, payload};
use mqtt::handlers::route;
use serde_json::json;

#[tokio::test]
async fn unknown_topic_is_ignored() {
    let runtime = common::MockRuntime::new();
    let store = common::in_memory_store().await;
    let ctx = common::make_ctx(runtime.clone(), store.clone()).await;

    route(&ctx, "lumehub/unknown/topic", b"anything").await;
    assert!(store.get_zones().await.unwrap().is_empty());
}

#[tokio::test]
async fn unrelated_prefix_is_ignored() {
    let runtime = common::MockRuntime::new();
    let store = common::in_memory_store().await;
    let ctx = common::make_ctx(runtime.clone(), store.clone()).await;

    route(&ctx, "other/device/set", &payload(json!({"state": "OFF"}))).await;
    assert!(*runtime.on.lock().unwrap());
}
