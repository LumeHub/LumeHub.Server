pub mod helpers;

use domain::{BlendMode, ParamValue};

#[tokio::test]
async fn starts_empty() {
    let store = helpers::in_memory_store().await;
    let active = store.get_active_layers().await.unwrap();
    assert!(active.is_empty());
}

#[tokio::test]
async fn layer_lifecycle() {
    let store = helpers::in_memory_store().await;
    let effect = store.create_effect("fx", "code", &[]).await.unwrap();
    let zone = store.create_zone("z", 0, 10, 4).await.unwrap();

    let layer = store
        .add_active_layer(
            &effect.id,
            &zone.id,
            BlendMode::Override,
            &helpers::make_params(),
        )
        .await
        .unwrap();
    assert_eq!(layer.params["speed"], ParamValue::Number(2.0));

    let updated = store
        .update_active_layer(&layer.id, &zone.id, false, BlendMode::Add, &[].into())
        .await
        .unwrap();
    assert!(!updated.enabled);
    assert!(matches!(updated.blend_mode, BlendMode::Add));

    store.remove_active_layer(&layer.id).await.unwrap();
    let active = store.get_active_layers().await.unwrap();
    assert!(active.is_empty());
}

#[tokio::test]
async fn clear() {
    let store = helpers::in_memory_store().await;
    let effect = store.create_effect("fx", "code", &[]).await.unwrap();
    let zone = store.create_zone("z", 0, 10, 4).await.unwrap();

    store
        .add_active_layer(&effect.id, &zone.id, BlendMode::Override, &[].into())
        .await
        .unwrap();
    store
        .add_active_layer(&effect.id, &zone.id, BlendMode::Add, &[].into())
        .await
        .unwrap();

    store.clear_active_scene().await.unwrap();
    let active = store.get_active_layers().await.unwrap();
    assert!(active.is_empty());
}

#[tokio::test]
async fn save_and_load() {
    let store = helpers::in_memory_store().await;
    let effect = store.create_effect("fx", "code", &[]).await.unwrap();
    let zone = store.create_zone("z", 0, 10, 4).await.unwrap();

    store
        .add_active_layer(
            &effect.id,
            &zone.id,
            BlendMode::Override,
            &helpers::make_params(),
        )
        .await
        .unwrap();

    let saved = store.save_active_as_scene("my scene").await.unwrap();
    assert_eq!(saved.name, "my scene");
    let layers = store.get_layers(&saved.id).await.unwrap();
    assert_eq!(layers.len(), 1);
    assert_eq!(layers[0].params["speed"], ParamValue::Number(2.0));

    let other = store.create_scene("other").await.unwrap();
    store
        .add_layer(&other.id, &effect.id, &zone.id, BlendMode::Add, &[].into())
        .await
        .unwrap();
    store.load_scene_into_active(&other.id).await.unwrap();

    let active = store.get_active_layers().await.unwrap();
    assert_eq!(active.len(), 1);
    assert!(matches!(active[0].blend_mode, BlendMode::Add));
}

#[tokio::test]
async fn load_nonexistent_returns_not_found() {
    let store = helpers::in_memory_store().await;
    let err = store
        .load_scene_into_active("nonexistent")
        .await
        .unwrap_err();
    assert!(matches!(err, store::StoreError::NotFound));
}

#[tokio::test]
async fn overwrite_and_cannot_overwrite_self() {
    let store = helpers::in_memory_store().await;
    let effect = store.create_effect("fx", "code", &[]).await.unwrap();
    let zone = store.create_zone("z", 0, 10, 4).await.unwrap();
    let scene = store.create_scene("s").await.unwrap();

    store
        .add_layer(
            &scene.id,
            &effect.id,
            &zone.id,
            BlendMode::Override,
            &[].into(),
        )
        .await
        .unwrap();
    store
        .add_active_layer(
            &effect.id,
            &zone.id,
            BlendMode::Add,
            &helpers::make_params(),
        )
        .await
        .unwrap();
    store
        .add_active_layer(&effect.id, &zone.id, BlendMode::Screen, &[].into())
        .await
        .unwrap();

    store.overwrite_scene_from_active(&scene.id).await.unwrap();

    let layers = store.get_layers(&scene.id).await.unwrap();
    assert_eq!(layers.len(), 2);
    assert!(matches!(layers[0].blend_mode, BlendMode::Add));

    let err = store
        .overwrite_scene_from_active(store::ACTIVE_SCENE_ID)
        .await
        .unwrap_err();
    assert!(matches!(err, store::StoreError::NotFound));
}
