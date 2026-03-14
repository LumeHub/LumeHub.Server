mod helpers;

use domain::{BlendMode, ParamControl, ParamDef, ParamValue, Rgb};

#[tokio::test]
async fn create_and_get_zone() {
    let store = helpers::in_memory_store().await;
    let zone = store.create_zone("living room", 0, 59, 8).await.unwrap();
    assert_eq!(zone.name, "living room");
    assert_eq!(zone.start_pixel, 0);
    assert_eq!(zone.end_pixel, 59);
    assert_eq!(zone.transition_length, 8);

    let fetched = store.get_zone(&zone.id).await.unwrap();
    assert_eq!(fetched.id, zone.id);
    assert_eq!(fetched.name, "living room");
}

#[tokio::test]
async fn list_zones() {
    let store = helpers::in_memory_store().await;
    store.create_zone("a", 0, 10, 4).await.unwrap();
    store.create_zone("b", 11, 20, 4).await.unwrap();
    let zones = store.get_zones().await.unwrap();
    assert_eq!(zones.len(), 2);
}

#[tokio::test]
async fn update_zone() {
    let store = helpers::in_memory_store().await;
    let zone = store.create_zone("old", 0, 10, 8).await.unwrap();
    let updated = store.update_zone(&zone.id, "new", 5, 50, 16).await.unwrap();
    assert_eq!(updated.name, "new");
    assert_eq!(updated.start_pixel, 5);
    assert_eq!(updated.end_pixel, 50);
    assert_eq!(updated.transition_length, 16);
}

#[tokio::test]
async fn delete_zone() {
    let store = helpers::in_memory_store().await;
    let zone = store.create_zone("tmp", 0, 10, 8).await.unwrap();
    store.delete_zone(&zone.id).await.unwrap();
    let err = store.get_zone(&zone.id).await.unwrap_err();
    assert!(matches!(err, store::StoreError::NotFound));
}

#[tokio::test]
async fn zone_not_found() {
    let store = helpers::in_memory_store().await;
    let err = store.get_zone("nonexistent").await.unwrap_err();
    assert!(matches!(err, store::StoreError::NotFound));
}

fn sample_params() -> Vec<ParamDef> {
    vec![
        ParamDef {
            name: "speed".to_string(),
            label: "Speed".to_string(),
            control: ParamControl::Slider {
                min: 0.5,
                max: 5.0,
                step: Some(0.5),
            },
            default: ParamValue::Number(1.0),
        },
        ParamDef {
            name: "color".to_string(),
            label: "Color".to_string(),
            control: ParamControl::Color,
            default: ParamValue::Color(Rgb { r: 255, g: 0, b: 0 }),
        },
        ParamDef {
            name: "reverse".to_string(),
            label: "Reverse".to_string(),
            control: ParamControl::Toggle,
            default: ParamValue::Bool(false),
        },
        ParamDef {
            name: "mode".to_string(),
            label: "Mode".to_string(),
            control: ParamControl::Select {
                options: vec!["linear".to_string(), "ease".to_string()],
            },
            default: ParamValue::Select("linear".to_string()),
        },
    ]
}

#[tokio::test]
async fn create_effect_with_params_round_trips() {
    let store = helpers::in_memory_store().await;
    let params = sample_params();
    let effect = store
        .create_effect("wave", "let c = primary_color; c", &params)
        .await
        .unwrap();

    assert_eq!(effect.name, "wave");
    assert_eq!(effect.script, "let c = primary_color; c");
    assert_eq!(effect.params.len(), 4);

    let fetched = store.get_effect(&effect.id).await.unwrap();
    assert_eq!(fetched.params[0].name, "speed");
    assert!(matches!(
        fetched.params[0].control,
        ParamControl::Slider { min, max, .. } if (min - 0.5).abs() < f32::EPSILON && (max - 5.0).abs() < f32::EPSILON
    ));
    assert_eq!(fetched.params[0].default, ParamValue::Number(1.0));
    assert_eq!(
        fetched.params[1].default,
        ParamValue::Color(Rgb { r: 255, g: 0, b: 0 })
    );
    assert_eq!(fetched.params[2].default, ParamValue::Bool(false));
    assert_eq!(
        fetched.params[3].default,
        ParamValue::Select("linear".to_string())
    );
}

#[tokio::test]
async fn list_effects() {
    let store = helpers::in_memory_store().await;
    store
        .create_effect("a", "#{r:0,g:0,b:0}", &[])
        .await
        .unwrap();
    store
        .create_effect("b", "#{r:0,g:0,b:0}", &[])
        .await
        .unwrap();
    let effects = store.get_effects().await.unwrap();
    assert_eq!(effects.len(), 2);
}

#[tokio::test]
async fn update_effect() {
    let store = helpers::in_memory_store().await;
    let effect = store.create_effect("old", "code", &[]).await.unwrap();
    let updated = store
        .update_effect(&effect.id, "new", "new_code", &sample_params())
        .await
        .unwrap();
    assert_eq!(updated.name, "new");
    assert_eq!(updated.script, "new_code");
    assert_eq!(updated.params.len(), 4);
}

#[tokio::test]
async fn delete_effect() {
    let store = helpers::in_memory_store().await;
    let effect = store.create_effect("tmp", "code", &[]).await.unwrap();
    store.delete_effect(&effect.id).await.unwrap();
    let err = store.get_effect(&effect.id).await.unwrap_err();
    assert!(matches!(err, store::StoreError::NotFound));
}

#[tokio::test]
async fn active_scene_excluded_from_list() {
    let store = helpers::in_memory_store().await;
    let scenes = store.get_scenes().await.unwrap();
    assert!(scenes.iter().all(|s| s.id != store::ACTIVE_SCENE_ID));
}

#[tokio::test]
async fn create_and_list_scenes() {
    let store = helpers::in_memory_store().await;
    store.create_scene("night").await.unwrap();
    store.create_scene("party").await.unwrap();
    let scenes = store.get_scenes().await.unwrap();
    assert_eq!(scenes.len(), 2);
}

#[tokio::test]
async fn update_scene() {
    let store = helpers::in_memory_store().await;
    let scene = store.create_scene("old").await.unwrap();
    let updated = store.update_scene(&scene.id, "new").await.unwrap();
    assert_eq!(updated.name, "new");
}

#[tokio::test]
async fn delete_scene() {
    let store = helpers::in_memory_store().await;
    let scene = store.create_scene("tmp").await.unwrap();
    store.delete_scene(&scene.id).await.unwrap();
    let err = store.get_scene(&scene.id).await.unwrap_err();
    assert!(matches!(err, store::StoreError::NotFound));
}

#[tokio::test]
async fn cannot_delete_active_scene() {
    let store = helpers::in_memory_store().await;
    let err = store
        .delete_scene(store::ACTIVE_SCENE_ID)
        .await
        .unwrap_err();
    assert!(matches!(err, store::StoreError::NotFound));
}

fn make_params() -> std::collections::HashMap<String, ParamValue> {
    [("speed".to_string(), ParamValue::Number(2.0))].into()
}

#[tokio::test]
async fn add_and_get_layers() {
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
            &make_params(),
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
        .update_layer(&layer.id, false, BlendMode::Screen, &make_params())
        .await
        .unwrap();
    assert!(!updated.enabled);
    assert!(matches!(updated.blend_mode, BlendMode::Screen));
    assert_eq!(updated.params["speed"], ParamValue::Number(2.0));
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

    // Reverse the order
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

    store.remove_layer(&layer.id).await.unwrap();
    let layers = store.get_layers(&scene.id).await.unwrap();
    assert!(layers.is_empty());
}

#[tokio::test]
async fn save_active_creates_named_scene() {
    let store = helpers::in_memory_store().await;
    let effect = store.create_effect("fx", "code", &[]).await.unwrap();
    let zone = store.create_zone("z", 0, 10, 4).await.unwrap();

    store
        .add_layer(
            store::ACTIVE_SCENE_ID,
            &effect.id,
            &zone.id,
            BlendMode::Override,
            &make_params(),
        )
        .await
        .unwrap();

    let saved = store.save_active_as_scene("my scene").await.unwrap();
    assert_eq!(saved.name, "my scene");

    let layers = store.get_layers(&saved.id).await.unwrap();
    assert_eq!(layers.len(), 1);
    assert_eq!(layers[0].params["speed"], ParamValue::Number(2.0));
}

#[tokio::test]
async fn load_scene_replaces_active() {
    let store = helpers::in_memory_store().await;
    let effect = store.create_effect("fx", "code", &[]).await.unwrap();
    let zone = store.create_zone("z", 0, 10, 4).await.unwrap();
    let scene = store.create_scene("s").await.unwrap();

    store
        .add_layer(
            &scene.id,
            &effect.id,
            &zone.id,
            BlendMode::Add,
            &make_params(),
        )
        .await
        .unwrap();

    // Put something in active first to verify it gets replaced
    store
        .add_layer(
            store::ACTIVE_SCENE_ID,
            &effect.id,
            &zone.id,
            BlendMode::Override,
            &[].into(),
        )
        .await
        .unwrap();

    store.load_scene_into_active(&scene.id).await.unwrap();

    let active = store.get_active_layers().await.unwrap();
    assert_eq!(active.len(), 1);
    assert!(matches!(active[0].blend_mode, BlendMode::Add));
    assert_eq!(active[0].params["speed"], ParamValue::Number(2.0));
}

#[tokio::test]
async fn clear_active_removes_all_layers() {
    let store = helpers::in_memory_store().await;
    let effect = store.create_effect("fx", "code", &[]).await.unwrap();
    let zone = store.create_zone("z", 0, 10, 4).await.unwrap();

    store
        .add_layer(
            store::ACTIVE_SCENE_ID,
            &effect.id,
            &zone.id,
            BlendMode::Override,
            &[].into(),
        )
        .await
        .unwrap();
    store
        .add_layer(
            store::ACTIVE_SCENE_ID,
            &effect.id,
            &zone.id,
            BlendMode::Add,
            &[].into(),
        )
        .await
        .unwrap();

    store.clear_active_scene().await.unwrap();
    let active = store.get_active_layers().await.unwrap();
    assert!(active.is_empty());
}

#[tokio::test]
async fn overwrite_scene_from_active() {
    let store = helpers::in_memory_store().await;
    let effect = store.create_effect("fx", "code", &[]).await.unwrap();
    let zone = store.create_zone("z", 0, 10, 4).await.unwrap();
    let scene = store.create_scene("s").await.unwrap();

    // Scene has one layer
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

    // Active has two layers with custom params
    store
        .add_layer(
            store::ACTIVE_SCENE_ID,
            &effect.id,
            &zone.id,
            BlendMode::Add,
            &make_params(),
        )
        .await
        .unwrap();
    store
        .add_layer(
            store::ACTIVE_SCENE_ID,
            &effect.id,
            &zone.id,
            BlendMode::Screen,
            &[].into(),
        )
        .await
        .unwrap();

    store.overwrite_scene_from_active(&scene.id).await.unwrap();

    let layers = store.get_layers(&scene.id).await.unwrap();
    assert_eq!(layers.len(), 2);
    assert!(matches!(layers[0].blend_mode, BlendMode::Add));
}

#[tokio::test]
async fn cannot_overwrite_active_with_itself() {
    let store = helpers::in_memory_store().await;
    let err = store
        .overwrite_scene_from_active(store::ACTIVE_SCENE_ID)
        .await
        .unwrap_err();
    assert!(matches!(err, store::StoreError::NotFound));
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

    // After scene deleted, layers should be gone (cascade)
    let layers = store.get_layers(&scene.id).await.unwrap();
    assert!(layers.is_empty());
}
