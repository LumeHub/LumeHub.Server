use crate::common;

use mqtt::handlers::route;

#[tokio::test]
async fn active_scene_set_by_name() {
    let runtime = common::MockRuntime::new();
    let store = common::in_memory_store().await;
    let ctx = common::make_ctx(runtime.clone(), store.clone()).await;

    let effect = store.create_effect("e", "fn main() {}", &[]).await.unwrap();
    let zone = store.create_zone("z", 0, 10, 0).await.unwrap();
    let scene = store.create_scene("Sunset").await.unwrap();
    store
        .add_active_layer(
            &effect.id,
            &zone.id,
            domain::BlendMode::Override,
            &Default::default(),
        )
        .await
        .unwrap();
    store.overwrite_scene_from_active(&scene.id).await.unwrap();
    store.clear_active_scene().await.unwrap();

    route(&ctx, "lumehub/active_scene/set", b"Sunset").await;

    let layers = store.get_active_layers().await.unwrap();
    assert!(!layers.is_empty());
    assert_eq!(*runtime.reload_count.lock().unwrap(), 1);
}

#[tokio::test]
async fn active_scene_set_empty_clears() {
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

    route(&ctx, "lumehub/active_scene/set", b"").await;

    assert!(store.get_active_layers().await.unwrap().is_empty());
    assert_eq!(*runtime.halt_count.lock().unwrap(), 1);
}
