use api_google::device::{
    device_states_from_snapshot, google_to_internal_brightness, internal_to_google_brightness,
};
use application::SceneSnapshot;
use domain::Rgb;

#[test]
fn google_100pct_maps_to_255() {
    assert_eq!(google_to_internal_brightness(100), 255);
}

#[test]
fn google_0pct_maps_to_0() {
    assert_eq!(google_to_internal_brightness(0), 0);
}

#[test]
fn google_50pct_maps_near_128() {
    let v = google_to_internal_brightness(50);
    assert!((v as i16 - 128).abs() <= 1);
}

#[test]
fn internal_255_maps_to_100pct() {
    assert_eq!(internal_to_google_brightness(255), 100);
}

#[test]
fn internal_0_maps_to_0pct() {
    assert_eq!(internal_to_google_brightness(0), 0);
}

#[test]
fn brightness_roundtrip_is_within_one() {
    for pct in 0u8..=100 {
        let internal = google_to_internal_brightness(pct);
        let back = internal_to_google_brightness(internal);
        assert!(
            (back as i16 - pct as i16).abs() <= 1,
            "roundtrip failed for {pct}%"
        );
    }
}

#[test]
fn device_states_reflects_snapshot() {
    let snap = SceneSnapshot {
        on: true,
        brightness: 255,
        color: Rgb::new(10, 20, 30),
        active_effect: Some("rainbow".to_string()),
        light_effect_end_unix_timestamp_sec: Some(9999),
    };
    let states = device_states_from_snapshot(&snap);
    assert_eq!(states.on, Some(true));
    assert_eq!(states.online, Some(true));
    assert_eq!(states.brightness, Some(100));
    assert_eq!(states.active_light_effect.as_deref(), Some("rainbow"));
    assert_eq!(states.light_effect_end_unix_timestamp_sec, Some(9999));
}
