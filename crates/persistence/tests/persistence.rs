use std::collections::HashMap;

use domain::Rgb;
use persistence::{PersistedState, load, save};

#[test]
fn load_returns_defaults_when_file_missing() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("state.json");

    let state = load(&path).expect("load should succeed on missing file");
    assert!(state.on);
    assert_eq!(state.brightness, 255);
    assert_eq!(state.color, Rgb::BLACK);
}

#[test]
fn save_and_load_round_trips() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("state.json");

    let original = PersistedState {
        on: false,
        brightness: 128,
        color: Rgb::new(10, 20, 30),
        active_effect: Some("lava".to_string()),
        signal_colors: HashMap::from([("primary_color".to_string(), Rgb::new(255, 0, 0))]),
        signal_scripts: HashMap::from([("wave".to_string(), "sin(t)".to_string())]),
    };

    save(&path, &original).expect("save failed");
    let restored = load(&path).expect("load failed");

    assert_eq!(restored.on, original.on);
    assert_eq!(restored.brightness, original.brightness);
    assert_eq!(restored.color, original.color);
    assert_eq!(restored.active_effect, original.active_effect);
    assert_eq!(restored.signal_colors, original.signal_colors);
    assert_eq!(restored.signal_scripts, original.signal_scripts);
}

#[test]
fn load_with_corrupt_file_returns_error() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("state.json");
    std::fs::write(&path, "not valid json {{{{").unwrap();

    assert!(load(&path).is_err());
}

#[test]
fn missing_optional_fields_deserialize_to_defaults() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("state.json");
    // Old state file without signal fields
    std::fs::write(
        &path,
        r#"{"on":true,"brightness":200,"color":{"r":0,"g":0,"b":0}}"#,
    )
    .unwrap();

    let state = load(&path).expect("load failed");
    assert!(state.active_effect.is_none());
    assert!(state.signal_colors.is_empty());
    assert!(state.signal_scripts.is_empty());
}
