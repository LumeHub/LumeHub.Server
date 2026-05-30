use crate::common::{self, payload};
use mqtt::handlers::route;
use serde_json::json;

#[tokio::test]
async fn scene_create() {
    let runtime = common::MockRuntime::new();
    let store = common::in_memory_store().await;
    let ctx = common::make_ctx(runtime, store.clone()).await;

    route(
        &ctx,
        "lumehub/scenes/create",
        &payload(json!({"name": "Sunset"})),
    )
    .await;

    let scenes = store.get_scenes().await.unwrap();
    assert_eq!(scenes.len(), 1);
    assert_eq!(scenes[0].name, "Sunset");
}

#[tokio::test]
async fn scene_update() {
    let runtime = common::MockRuntime::new();
    let store = common::in_memory_store().await;
    let ctx = common::make_ctx(runtime, store.clone()).await;

    let scene = store.create_scene("Old").await.unwrap();
    let id = scene.id;

    route(
        &ctx,
        &format!("lumehub/scenes/{id}/update"),
        &payload(json!({"name": "New"})),
    )
    .await;

    let updated = store.get_scene(&id).await.unwrap();
    assert_eq!(updated.name, "New");
}

#[tokio::test]
async fn scene_delete() {
    let runtime = common::MockRuntime::new();
    let store = common::in_memory_store().await;
    let ctx = common::make_ctx(runtime, store.clone()).await;

    let scene = store.create_scene("TempScene").await.unwrap();
    let id = scene.id;

    route(&ctx, &format!("lumehub/scenes/{id}/delete"), b"").await;

    let scenes = store.get_scenes().await.unwrap();
    assert!(scenes.iter().all(|s| s.id != id));
}

#[tokio::test]
async fn scene_load_into_active() {
    let runtime = common::MockRuntime::new();
    let store = common::in_memory_store().await;
    let ctx = common::make_ctx(runtime.clone(), store.clone()).await;

    let effect = store.create_effect("e", "fn main() {}", &[]).await.unwrap();
    let zone = store.create_zone("z", 0, 10, 0).await.unwrap();
    let scene = store.create_scene("MyScene").await.unwrap();
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

    assert!(store.get_active_layers().await.unwrap().is_empty());

    route(&ctx, &format!("lumehub/scenes/{}/load", scene.id), b"").await;

    let layers = store.get_active_layers().await.unwrap();
    assert!(!layers.is_empty());
    assert_eq!(*runtime.reload_count.lock().unwrap(), 1);
}
