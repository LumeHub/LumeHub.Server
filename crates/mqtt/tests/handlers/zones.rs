use crate::common::{self, payload};
use mqtt::handlers::route;
use serde_json::json;

#[tokio::test]
async fn zone_create() {
    let runtime = common::MockRuntime::new();
    let store = common::in_memory_store().await;
    let ctx = common::make_ctx(runtime, store.clone()).await;

    route(
        &ctx,
        "lumehub/zones/create",
        &payload(json!({"name": "Left", "start_pixel": 0, "end_pixel": 49})),
    )
    .await;

    let zones = store.get_zones().await.unwrap();
    assert_eq!(zones.len(), 1);
    assert_eq!(zones[0].name, "Left");
    assert_eq!(zones[0].start_pixel, 0);
    assert_eq!(zones[0].end_pixel, 49);
    assert_eq!(zones[0].transition_length, 0);
}

#[tokio::test]
async fn zone_create_with_transition() {
    let runtime = common::MockRuntime::new();
    let store = common::in_memory_store().await;
    let ctx = common::make_ctx(runtime, store.clone()).await;

    route(
        &ctx,
        "lumehub/zones/create",
        &payload(json!({"name": "Z", "start_pixel": 0, "end_pixel": 10, "transition_length": 8})),
    )
    .await;

    let zones = store.get_zones().await.unwrap();
    assert_eq!(zones[0].transition_length, 8);
}

#[tokio::test]
async fn zone_update() {
    let runtime = common::MockRuntime::new();
    let store = common::in_memory_store().await;
    let ctx = common::make_ctx(runtime, store.clone()).await;

    let zone = store.create_zone("Old", 0, 10, 0).await.unwrap();
    let id = zone.id;

    route(
        &ctx,
        &format!("lumehub/zones/{id}/update"),
        &payload(json!({"name": "New", "start_pixel": 5, "end_pixel": 20})),
    )
    .await;

    let updated = store.get_zone(&id).await.unwrap();
    assert_eq!(updated.name, "New");
    assert_eq!(updated.start_pixel, 5);
    assert_eq!(updated.end_pixel, 20);
}

#[tokio::test]
async fn zone_delete() {
    let runtime = common::MockRuntime::new();
    let store = common::in_memory_store().await;
    let ctx = common::make_ctx(runtime, store.clone()).await;

    let zone = store.create_zone("Temp", 0, 10, 0).await.unwrap();
    let id = zone.id;

    route(&ctx, &format!("lumehub/zones/{id}/delete"), b"").await;

    let zones = store.get_zones().await.unwrap();
    assert!(zones.iter().all(|z| z.id != id));
}

#[tokio::test]
async fn zone_create_ignores_invalid_json() {
    let runtime = common::MockRuntime::new();
    let store = common::in_memory_store().await;
    let ctx = common::make_ctx(runtime, store.clone()).await;

    route(&ctx, "lumehub/zones/create", b"bad").await;

    let zones = store.get_zones().await.unwrap();
    assert!(zones.is_empty());
}
