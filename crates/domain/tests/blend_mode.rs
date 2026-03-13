use domain::BlendMode;

#[test]
fn add_parses_from_str() {
    assert!(matches!(BlendMode::from("add"), BlendMode::Add));
}

#[test]
fn override_is_fallback_for_unknown_string() {
    assert!(matches!(BlendMode::from("override"), BlendMode::Override));
    assert!(matches!(BlendMode::from("xyz"), BlendMode::Override));
    assert!(matches!(BlendMode::from(""), BlendMode::Override));
}

#[test]
fn default_is_override() {
    assert!(matches!(BlendMode::default(), BlendMode::Override));
}
