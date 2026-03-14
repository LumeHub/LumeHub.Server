use application::{SceneSnapshot, StateEventBus};
use domain::Rgb;

fn dummy_snapshot() -> SceneSnapshot {
    SceneSnapshot {
        on: true,
        brightness: 200,
        color: Rgb::new(10, 20, 30),
        active_effect: None,
        light_effect_end_unix_timestamp_sec: None,
    }
}

#[test]
fn subscriber_receives_notified_snapshot() {
    let bus = StateEventBus::new();
    let mut rx = bus.subscribe();

    let snap = dummy_snapshot();
    bus.notify(snap.clone());

    let received = rx.try_recv().expect("should have received snapshot");
    assert_eq!(received.on, snap.on);
    assert_eq!(received.brightness, snap.brightness);
    assert_eq!(received.color.r, snap.color.r);
}

#[test]
fn multiple_subscribers_each_receive_snapshot() {
    let bus = StateEventBus::new();
    let mut rx1 = bus.subscribe();
    let mut rx2 = bus.subscribe();

    bus.notify(dummy_snapshot());

    assert!(rx1.try_recv().is_ok());
    assert!(rx2.try_recv().is_ok());
}

#[test]
fn subscriber_created_after_notify_misses_past_events() {
    let bus = StateEventBus::new();
    bus.notify(dummy_snapshot());

    let mut late_rx = bus.subscribe();
    assert!(late_rx.try_recv().is_err());
}
