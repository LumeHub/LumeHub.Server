use std::collections::HashMap;

use domain::{BlendMode, ParamValue};
use serde_json::{Value, json};

#[test]
fn device_command_partial_fields() {
    let v: Value = json!({"state": "ON"});
    assert_eq!(v["state"], "ON");
    assert!(v.get("brightness").is_none() || v["brightness"].is_null());
}

#[test]
fn param_value_roundtrip() {
    let number: ParamValue =
        serde_json::from_value(json!({"type": "number", "value": 2.5})).unwrap();
    assert_eq!(number, ParamValue::Number(2.5));

    let bool_val: ParamValue =
        serde_json::from_value(json!({"type": "bool", "value": true})).unwrap();
    assert_eq!(bool_val, ParamValue::Bool(true));

    let select: ParamValue =
        serde_json::from_value(json!({"type": "select", "value": "option_a"})).unwrap();
    assert_eq!(select, ParamValue::Select("option_a".into()));
}

#[test]
fn blend_mode_roundtrip() {
    for variant in [
        BlendMode::Override,
        BlendMode::Add,
        BlendMode::Screen,
        BlendMode::Multiply,
    ] {
        let serialized = serde_json::to_string(&variant).unwrap();
        let back: BlendMode = serde_json::from_str(&serialized).unwrap();
        assert_eq!(back, variant);
    }
}

#[test]
fn blend_mode_serializes_lowercase() {
    assert_eq!(
        serde_json::to_string(&BlendMode::Override).unwrap(),
        "\"override\""
    );
    assert_eq!(serde_json::to_string(&BlendMode::Add).unwrap(), "\"add\"");
}

#[test]
fn add_layer_command_defaults() {
    let v: Value = json!({"effect_id": "builtin:solid", "zone_id": "zone-1"});
    let blend: BlendMode =
        serde_json::from_value(v.get("blend_mode").cloned().unwrap_or(json!("override"))).unwrap();
    assert_eq!(blend, BlendMode::Override);

    let params: HashMap<String, ParamValue> =
        serde_json::from_value(v.get("params").cloned().unwrap_or(json!({}))).unwrap();
    assert!(params.is_empty());
}
