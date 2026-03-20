use domain::{ParamValue, Rgb};

#[test]
fn device_color_serializes_without_value_field() {
    let json = serde_json::to_string(&ParamValue::DeviceColor).unwrap();
    assert_eq!(json, r#"{"type":"device_color"}"#);
}

#[test]
fn device_color_deserializes_from_type_field() {
    let v: ParamValue = serde_json::from_str(r#"{"type":"device_color"}"#).unwrap();
    assert_eq!(v, ParamValue::DeviceColor);
}

#[test]
fn color_roundtrips() {
    let original = ParamValue::Color(Rgb {
        r: 10,
        g: 20,
        b: 30,
    });
    let json = serde_json::to_string(&original).unwrap();
    let parsed: ParamValue = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, original);
}
