use crate::common::{self, payload};
use mqtt::handlers::route;
use serde_json::json;

#[tokio::test]
async fn active_layers_clear() {
    let runtime = common::MockRuntime::new();
    let store = common::in_memory_store().await;
    let ctx = common::make_ctx(runtime.clone(), store.clone()).await;

    let effect = store.create_effect("e", "fn main() {}", &[]).await.unwrap();
    let zone = store.create_zone("z", 0, 10, 0).await.unwrap();
    store
        .add_active_layer(
            &effect.id,
            &zone.id,
            domain::BlendMode::Override,
            &Default::default(),
        )
        .await
        .unwrap();

    assert!(!store.get_active_layers().await.unwrap().is_empty());

    route(&ctx, "lumehub/active_layers/clear", b"").await;

    assert!(store.get_active_layers().await.unwrap().is_empty());
    assert_eq!(*runtime.halt_count.lock().unwrap(), 1);
}

#[tokio::test]
async fn active_layers_add() {
    let runtime = common::MockRuntime::new();
    let store = common::in_memory_store().await;
    let ctx = common::make_ctx(runtime.clone(), store.clone()).await;

    let effect = store.create_effect("e", "fn main() {}", &[]).await.unwrap();
    let zone = store.create_zone("z", 0, 10, 0).await.unwrap();

    route(
        &ctx,
        "lumehub/active_layers/add",
        &payload(json!({"effect_id": effect.id, "zone_id": zone.id})),
    )
    .await;

    let layers = store.get_active_layers().await.unwrap();
    assert_eq!(layers.len(), 1);
    assert_eq!(layers[0].effect_id, effect.id);
    assert_eq!(*runtime.reload_count.lock().unwrap(), 1);
}

#[tokio::test]
async fn active_layers_set_replaces_all() {
    let runtime = common::MockRuntime::new();
    let store = common::in_memory_store().await;
    let ctx = common::make_ctx(runtime.clone(), store.clone()).await;

    let e1 = store
        .create_effect("e1", "fn main() {}", &[])
        .await
        .unwrap();
    let e2 = store
        .create_effect("e2", "fn main() {}", &[])
        .await
        .unwrap();
    let zone = store.create_zone("z", 0, 10, 0).await.unwrap();

    store
        .add_active_layer(
            &e1.id,
            &zone.id,
            domain::BlendMode::Override,
            &Default::default(),
        )
        .await
        .unwrap();

    route(
        &ctx,
        "lumehub/active_layers/set",
        &payload(json!([{"effect_id": e2.id, "zone_id": zone.id}])),
    )
    .await;

    let layers = store.get_active_layers().await.unwrap();
    assert_eq!(layers.len(), 1);
    assert_eq!(layers[0].effect_id, e2.id);
}

#[tokio::test]
async fn active_layer_update() {
    let runtime = common::MockRuntime::new();
    let store = common::in_memory_store().await;
    let ctx = common::make_ctx(runtime.clone(), store.clone()).await;

    let effect = store.create_effect("e", "fn main() {}", &[]).await.unwrap();
    let z1 = store.create_zone("z1", 0, 10, 0).await.unwrap();
    let z2 = store.create_zone("z2", 10, 20, 0).await.unwrap();
    let layer = store
        .add_active_layer(
            &effect.id,
            &z1.id,
            domain::BlendMode::Override,
            &Default::default(),
        )
        .await
        .unwrap();

    route(
        &ctx,
        &format!("lumehub/active_layers/{}/update", layer.id),
        &payload(json!({"zone_id": z2.id})),
    )
    .await;

    let updated = store.get_active_layer(&layer.id).await.unwrap();
    assert_eq!(updated.zone_id, z2.id);
    assert_eq!(*runtime.reload_count.lock().unwrap(), 1);
}

#[tokio::test]
async fn active_layer_delete() {
    let runtime = common::MockRuntime::new();
    let store = common::in_memory_store().await;
    let ctx = common::make_ctx(runtime.clone(), store.clone()).await;

    let effect = store.create_effect("e", "fn main() {}", &[]).await.unwrap();
    let zone = store.create_zone("z", 0, 10, 0).await.unwrap();
    let layer = store
        .add_active_layer(
            &effect.id,
            &zone.id,
            domain::BlendMode::Override,
            &Default::default(),
        )
        .await
        .unwrap();

    route(
        &ctx,
        &format!("lumehub/active_layers/{}/delete", layer.id),
        b"",
    )
    .await;

    assert!(store.get_active_layers().await.unwrap().is_empty());
    assert_eq!(*runtime.reload_count.lock().unwrap(), 1);
}

#[tokio::test]
async fn active_layers_reorder() {
    let runtime = common::MockRuntime::new();
    let store = common::in_memory_store().await;
    let ctx = common::make_ctx(runtime.clone(), store.clone()).await;

    let effect = store.create_effect("e", "fn main() {}", &[]).await.unwrap();
    let zone = store.create_zone("z", 0, 10, 0).await.unwrap();
    let l1 = store
        .add_active_layer(
            &effect.id,
            &zone.id,
            domain::BlendMode::Override,
            &Default::default(),
        )
        .await
        .unwrap();
    let l2 = store
        .add_active_layer(
            &effect.id,
            &zone.id,
            domain::BlendMode::Add,
            &Default::default(),
        )
        .await
        .unwrap();

    route(
        &ctx,
        "lumehub/active_layers/reorder",
        &payload(json!({"ordered_ids": [l2.id, l1.id]})),
    )
    .await;

    let layers = store.get_active_layers().await.unwrap();
    assert_eq!(layers[0].id, l2.id);
    assert_eq!(layers[1].id, l1.id);
    assert_eq!(*runtime.reload_count.lock().unwrap(), 1);
}
