pub mod helpers;

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
async fn get_nonexistent_returns_404() {
    let store = helpers::in_memory_store().await;
    let err = store.get_zone("nonexistent").await.unwrap_err();
    assert!(matches!(err, store::StoreError::NotFound));
}
