mod common;

use api::transition::TransitionQuery;
use application::SceneRuntime;
use domain::{TransitionCurve, TransitionSpec};

use common::DefaultSpecRuntime;

fn runtime_with(default: TransitionSpec) -> DefaultSpecRuntime {
    DefaultSpecRuntime::new(default)
}

#[test]
fn empty_query_resolves_to_runtime_default_spec() {
    let runtime = runtime_with(TransitionSpec::new(750, TransitionCurve::EaseInOut));
    let query = TransitionQuery::default();
    let spec = query.resolve(&runtime as &dyn SceneRuntime);
    assert_eq!(spec, runtime.default_spec());
}

#[test]
fn fade_ms_overrides_duration_but_keeps_default_curve() {
    let runtime = runtime_with(TransitionSpec::new(800, TransitionCurve::EaseOut));
    let query = TransitionQuery {
        fade_ms: Some(120),
        curve: None,
    };
    let spec = query.resolve(&runtime as &dyn SceneRuntime);
    assert_eq!(spec.duration_ms, 120);
    assert_eq!(spec.curve, TransitionCurve::EaseOut);
}

#[test]
fn curve_overrides_default_but_keeps_default_duration() {
    let runtime = runtime_with(TransitionSpec::new(500, TransitionCurve::Linear));
    let query = TransitionQuery {
        fade_ms: None,
        curve: Some(TransitionCurve::Exp),
    };
    let spec = query.resolve(&runtime as &dyn SceneRuntime);
    assert_eq!(spec.duration_ms, 500);
    assert_eq!(spec.curve, TransitionCurve::Exp);
}

#[test]
fn fade_ms_and_curve_override_both_fields() {
    let runtime = runtime_with(TransitionSpec::new(300, TransitionCurve::EaseInOut));
    let query = TransitionQuery {
        fade_ms: Some(2000),
        curve: Some(TransitionCurve::EaseIn),
    };
    let spec = query.resolve(&runtime as &dyn SceneRuntime);
    assert_eq!(spec, TransitionSpec::new(2000, TransitionCurve::EaseIn));
}

#[test]
fn fade_ms_zero_resolves_to_instant_duration() {
    let runtime = runtime_with(TransitionSpec::new(500, TransitionCurve::EaseInOut));
    let query = TransitionQuery {
        fade_ms: Some(0),
        curve: None,
    };
    let spec = query.resolve(&runtime as &dyn SceneRuntime);
    assert_eq!(spec.duration_ms, 0);
    assert!(spec.is_instant());
}
