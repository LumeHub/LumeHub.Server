pub mod helpers;

use domain::{BlendMode, ParamValue};

#[tokio::test]
async fn add_and_get() {
    let store = helpers::in_memory_store().await;
    let effect = store.create_effect("fx", "code", &[]).await.unwrap();
    let zone = store.create_zone("z", 0, 10, 4).await.unwrap();
    let scene = store.create_scene("s").await.unwrap();

    let l0 = store
        .add_layer(
            &scene.id,
            &effect.id,
            &zone.id,
            BlendMode::Override,
            &helpers::make_params(),
        )
        .await
        .unwrap();
    let l1 = store
        .add_layer(&scene.id, &effect.id, &zone.id, BlendMode::Add, &[].into())
        .await
        .unwrap();

    assert_eq!(l0.position, 0);
    assert_eq!(l1.position, 1);
    assert!(matches!(l0.blend_mode, BlendMode::Override));
    assert!(matches!(l1.blend_mode, BlendMode::Add));
    assert_eq!(l0.params["speed"], ParamValue::Number(2.0));

    let layers = store.get_layers(&scene.id).await.unwrap();
    assert_eq!(layers.len(), 2);
    assert_eq!(layers[0].id, l0.id);
    assert_eq!(layers[1].id, l1.id);
}

#[tokio::test]
async fn update_layer() {
    let store = helpers::in_memory_store().await;
    let effect = store.create_effect("fx", "code", &[]).await.unwrap();
    let zone = store.create_zone("z", 0, 10, 4).await.unwrap();
    let scene = store.create_scene("s").await.unwrap();
    let layer = store
        .add_layer(
            &scene.id,
            &effect.id,
            &zone.id,
            BlendMode::Override,
            &[].into(),
        )
        .await
        .unwrap();

    let updated = store
        .update_layer(
            &layer.id,
            &scene.id,
            &zone.id,
            false,
            BlendMode::Screen,
            &helpers::make_params(),
        )
        .await
        .unwrap();
    assert!(!updated.enabled);
    assert!(matches!(updated.blend_mode, BlendMode::Screen));
    assert_eq!(updated.params["speed"], ParamValue::Number(2.0));
}

#[tokio::test]
async fn remove_layer() {
    let store = helpers::in_memory_store().await;
    let effect = store.create_effect("fx", "code", &[]).await.unwrap();
    let zone = store.create_zone("z", 0, 10, 4).await.unwrap();
    let scene = store.create_scene("s").await.unwrap();
    let layer = store
        .add_layer(
            &scene.id,
            &effect.id,
            &zone.id,
            BlendMode::Override,
            &[].into(),
        )
        .await
        .unwrap();

    store.remove_layer(&layer.id, &scene.id).await.unwrap();
    let layers = store.get_layers(&scene.id).await.unwrap();
    assert!(layers.is_empty());
}

#[tokio::test]
async fn reorder_layers() {
    let store = helpers::in_memory_store().await;
    let effect = store.create_effect("fx", "code", &[]).await.unwrap();
    let zone = store.create_zone("z", 0, 10, 4).await.unwrap();
    let scene = store.create_scene("s").await.unwrap();

    let l0 = store
        .add_layer(
            &scene.id,
            &effect.id,
            &zone.id,
            BlendMode::Override,
            &[].into(),
        )
        .await
        .unwrap();
    let l1 = store
        .add_layer(
            &scene.id,
            &effect.id,
            &zone.id,
            BlendMode::Override,
            &[].into(),
        )
        .await
        .unwrap();
    let l2 = store
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
        .reorder_layers(&scene.id, &[l2.id.clone(), l1.id.clone(), l0.id.clone()])
        .await
        .unwrap();

    let layers = store.get_layers(&scene.id).await.unwrap();
    assert_eq!(layers[0].id, l2.id);
    assert_eq!(layers[1].id, l1.id);
    assert_eq!(layers[2].id, l0.id);
}

#[tokio::test]
async fn add_layer_defaults_opacity_to_one() {
    let store = helpers::in_memory_store().await;
    let effect = store.create_effect("fx", "code", &[]).await.unwrap();
    let zone = store.create_zone("z", 0, 10, 4).await.unwrap();
    let scene = store.create_scene("s").await.unwrap();
    let layer = store
        .add_layer(
            &scene.id,
            &effect.id,
            &zone.id,
            BlendMode::Override,
            &[].into(),
        )
        .await
        .unwrap();
    assert_eq!(layer.opacity, 1.0);
}

#[tokio::test]
async fn replace_active_layers_persists_opacity() {
    use store::NewLayer;
    let store = helpers::in_memory_store().await;
    let effect = store.create_effect("fx", "code", &[]).await.unwrap();
    let zone = store.create_zone("z", 0, 10, 4).await.unwrap();

    store
        .replace_active_layers(&[NewLayer {
            effect_id: effect.id.clone(),
            zone_id: zone.id.clone(),
            blend_mode: BlendMode::Override,
            params: [].into(),
            opacity: 0.42,
        }])
        .await
        .unwrap();

    let layers = store.get_active_layers().await.unwrap();
    assert_eq!(layers.len(), 1);
    assert!((layers[0].opacity - 0.42).abs() < 1e-5);
}

#[tokio::test]
async fn deleting_scene_cascades_to_layers() {
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

    store.delete_scene(&scene.id).await.unwrap();

    let layers = store.get_layers(&scene.id).await.unwrap();
    assert!(layers.is_empty());
}
