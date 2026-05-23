use domain::{TransitionCurve, TransitionSpec};

#[test]
fn instant_constant_has_zero_duration() {
    assert!(TransitionSpec::INSTANT.is_instant());
    assert_eq!(TransitionSpec::INSTANT.duration_ms, 0);
}

#[test]
fn non_zero_duration_is_not_instant() {
    let spec = TransitionSpec::new(100, TransitionCurve::Linear);
    assert!(!spec.is_instant());
}

#[test]
fn default_is_instant() {
    assert_eq!(TransitionSpec::default(), TransitionSpec::INSTANT);
}

#[test]
fn frames_divides_duration_by_frame_step() {
    let spec = TransitionSpec::new(320, TransitionCurve::Linear);
    assert_eq!(spec.frames(16), 20);
}

#[test]
fn frames_zero_step_yields_zero() {
    let spec = TransitionSpec::new(320, TransitionCurve::Linear);
    assert_eq!(spec.frames(0), 0);
}

#[test]
fn instant_spec_yields_zero_frames() {
    assert_eq!(TransitionSpec::INSTANT.frames(16), 0);
}

#[test]
fn serde_round_trips() {
    let spec = TransitionSpec::new(500, TransitionCurve::Exp);
    let json = serde_json::to_string(&spec).unwrap();
    let back: TransitionSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(spec, back);
}

#[test]
fn serde_defaults_curve_when_field_omitted() {
    let parsed: TransitionSpec = serde_json::from_str(r#"{"duration_ms": 250}"#).unwrap();
    assert_eq!(parsed.duration_ms, 250);
    assert_eq!(parsed.curve, TransitionCurve::EaseInOut);
}
