use crate::common::{self, payload};
use mqtt::handlers::route;
use serde_json::json;

#[tokio::test]
async fn effect_create() {
    let runtime = common::MockRuntime::new();
    let store = common::in_memory_store().await;
    let ctx = common::make_ctx(runtime, store.clone()).await;

    route(
        &ctx,
        "lumehub/effects/create",
        &payload(json!({"name": "Blink", "script": "fn main() {}"})),
    )
    .await;

    let effects = store.get_effects().await.unwrap();
    assert_eq!(effects.len(), 1);
    assert_eq!(effects[0].name, "Blink");
}

#[tokio::test]
async fn effect_update() {
    let runtime = common::MockRuntime::new();
    let store = common::in_memory_store().await;
    let ctx = common::make_ctx(runtime, store.clone()).await;

    let effect = store
        .create_effect("Old", "fn main() {}", &[])
        .await
        .unwrap();
    let id = effect.id;

    route(
        &ctx,
        &format!("lumehub/effects/{id}/update"),
        &payload(json!({"name": "New"})),
    )
    .await;

    let updated = store.get_effect(&id).await.unwrap();
    assert_eq!(updated.name, "New");
}

#[tokio::test]
async fn effect_delete() {
    let runtime = common::MockRuntime::new();
    let store = common::in_memory_store().await;
    let ctx = common::make_ctx(runtime, store.clone()).await;

    let effect = store
        .create_effect("Del", "fn main() {}", &[])
        .await
        .unwrap();
    let id = effect.id;

    route(&ctx, &format!("lumehub/effects/{id}/delete"), b"").await;

    let effects = store.get_effects().await.unwrap();
    assert!(effects.iter().all(|e| e.id != id));
}
