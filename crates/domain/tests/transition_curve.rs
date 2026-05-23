use domain::TransitionCurve;

const EPS: f32 = 1e-5;

fn approx(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() < EPS,
        "expected {expected}, got {actual}"
    );
}

const ALL: [TransitionCurve; 5] = [
    TransitionCurve::Linear,
    TransitionCurve::EaseIn,
    TransitionCurve::EaseOut,
    TransitionCurve::EaseInOut,
    TransitionCurve::Exp,
];

#[test]
fn endpoints_are_exact_for_every_curve() {
    for curve in ALL {
        approx(curve.weight(0.0), 0.0);
        approx(curve.weight(1.0), 1.0);
    }
}

#[test]
fn input_below_zero_clamps_to_zero() {
    for curve in ALL {
        approx(curve.weight(-0.5), 0.0);
        approx(curve.weight(-10.0), 0.0);
    }
}

#[test]
fn input_above_one_clamps_to_one() {
    for curve in ALL {
        approx(curve.weight(1.5), 1.0);
        approx(curve.weight(10.0), 1.0);
    }
}

#[test]
fn linear_is_identity() {
    approx(TransitionCurve::Linear.weight(0.25), 0.25);
    approx(TransitionCurve::Linear.weight(0.5), 0.5);
    approx(TransitionCurve::Linear.weight(0.75), 0.75);
}

#[test]
fn ease_in_starts_below_linear() {
    let c = TransitionCurve::EaseIn;
    assert!(c.weight(0.25) < 0.25);
    assert!(c.weight(0.5) < 0.5);
    assert!(c.weight(0.75) < 0.75);
}

#[test]
fn ease_out_starts_above_linear() {
    let c = TransitionCurve::EaseOut;
    assert!(c.weight(0.25) > 0.25);
    assert!(c.weight(0.5) > 0.5);
    assert!(c.weight(0.75) > 0.75);
}

#[test]
fn ease_in_out_is_symmetric_at_midpoint() {
    approx(TransitionCurve::EaseInOut.weight(0.5), 0.5);
}

#[test]
fn every_curve_is_monotonic_non_decreasing() {
    for curve in ALL {
        let mut prev = curve.weight(0.0);
        for i in 1..=100 {
            let cur = curve.weight(i as f32 / 100.0);
            assert!(
                cur >= prev,
                "{curve:?} non-monotonic at {i}: {prev} -> {cur}"
            );
            prev = cur;
        }
    }
}

#[test]
fn default_curve_is_ease_in_out() {
    assert_eq!(TransitionCurve::default(), TransitionCurve::EaseInOut);
}

#[test]
fn serde_uses_snake_case_variant_names() {
    assert_eq!(
        serde_json::to_string(&TransitionCurve::EaseInOut).unwrap(),
        "\"ease_in_out\""
    );
    assert_eq!(
        serde_json::to_string(&TransitionCurve::EaseIn).unwrap(),
        "\"ease_in\""
    );
    assert_eq!(
        serde_json::to_string(&TransitionCurve::Exp).unwrap(),
        "\"exp\""
    );
}

#[test]
fn serde_deserializes_from_snake_case() {
    let parsed: TransitionCurve = serde_json::from_str("\"ease_in\"").unwrap();
    assert_eq!(parsed, TransitionCurve::EaseIn);
}
