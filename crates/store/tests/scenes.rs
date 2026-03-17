pub mod helpers;

#[tokio::test]
async fn active_excluded_from_list() {
    let store = helpers::in_memory_store().await;
    let scenes = store.get_scenes().await.unwrap();
    assert!(scenes.iter().all(|s| s.id != store::ACTIVE_SCENE_ID));
}

#[tokio::test]
async fn create_and_list() {
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
async fn cannot_delete_active() {
    let store = helpers::in_memory_store().await;
    let err = store
        .delete_scene(store::ACTIVE_SCENE_ID)
        .await
        .unwrap_err();
    assert!(matches!(err, store::StoreError::NotFound));
}
