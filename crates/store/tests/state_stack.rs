mod helpers;

use domain::{BlendMode, Rgb};
use helpers::in_memory_store;
use store::{StackDevice, StackLayer};

fn sample_layers() -> Vec<StackLayer> {
    vec![StackLayer {
        effect_id: "builtin:rainbow".to_string(),
        zone_id: "all".to_string(),
        blend_mode: BlendMode::Override,
        enabled: true,
        params: Default::default(),
    }]
}

fn sample_device() -> StackDevice {
    StackDevice {
        on: true,
        brightness: 200,
        color: Rgb::new(10, 20, 30),
    }
}

#[tokio::test]
async fn depth_starts_at_zero() {
    let store = in_memory_store().await;
    assert_eq!(store.state_stack_depth().await.unwrap(), 0);
}

#[tokio::test]
async fn push_increments_depth() {
    let store = in_memory_store().await;
    store
        .push_state(&sample_layers(), &sample_device())
        .await
        .unwrap();
    store
        .push_state(&sample_layers(), &sample_device())
        .await
        .unwrap();
    assert_eq!(store.state_stack_depth().await.unwrap(), 2);
}

#[tokio::test]
async fn pop_returns_last_pushed_entry() {
    let store = in_memory_store().await;
    let layers_a = sample_layers();
    let device_a = sample_device();
    let layers_b = vec![StackLayer {
        effect_id: "builtin:aurora".to_string(),
        zone_id: "all".to_string(),
        blend_mode: BlendMode::Add,
        enabled: false,
        params: Default::default(),
    }];
    let device_b = StackDevice {
        on: false,
        brightness: 50,
        color: Rgb::new(255, 0, 0),
    };

    store.push_state(&layers_a, &device_a).await.unwrap();
    store.push_state(&layers_b, &device_b).await.unwrap();

    let popped = store.pop_state().await.unwrap().expect("entry expected");
    assert_eq!(popped.layers, layers_b);
    assert_eq!(popped.device, device_b);
    assert_eq!(store.state_stack_depth().await.unwrap(), 1);
}

#[tokio::test]
async fn pop_returns_none_when_empty() {
    let store = in_memory_store().await;
    assert!(store.pop_state().await.unwrap().is_none());
}

#[tokio::test]
async fn peek_returns_last_pushed_without_removing() {
    let store = in_memory_store().await;
    store
        .push_state(&sample_layers(), &sample_device())
        .await
        .unwrap();
    let peeked = store.peek_state().await.unwrap().expect("entry expected");
    assert_eq!(peeked.device, sample_device());
    assert_eq!(store.state_stack_depth().await.unwrap(), 1);
}

#[tokio::test]
async fn list_returns_entries_in_lifo_order() {
    let store = in_memory_store().await;
    let device_first = sample_device();
    let device_second = StackDevice {
        on: false,
        brightness: 0,
        color: Rgb::BLACK,
    };
    store
        .push_state(&sample_layers(), &device_first)
        .await
        .unwrap();
    store
        .push_state(&sample_layers(), &device_second)
        .await
        .unwrap();

    let entries = store.list_state_stack().await.unwrap();
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].device, device_second);
    assert_eq!(entries[1].device, device_first);
}

#[tokio::test]
async fn clear_drops_every_entry() {
    let store = in_memory_store().await;
    store
        .push_state(&sample_layers(), &sample_device())
        .await
        .unwrap();
    store
        .push_state(&sample_layers(), &sample_device())
        .await
        .unwrap();
    store.clear_state_stack().await.unwrap();
    assert_eq!(store.state_stack_depth().await.unwrap(), 0);
    assert!(store.peek_state().await.unwrap().is_none());
}

#[tokio::test]
async fn pushed_at_is_populated_with_unix_timestamp() {
    let store = in_memory_store().await;
    store
        .push_state(&sample_layers(), &sample_device())
        .await
        .unwrap();
    let entry = store.peek_state().await.unwrap().unwrap();
    assert!(entry.pushed_at > 0);
}
