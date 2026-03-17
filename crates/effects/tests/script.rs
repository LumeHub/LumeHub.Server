use domain::Rgb;
use effects::build_registry;
use engine::Effect;

#[test]
fn script_renders_solid_color() {
    let layer = build_registry()
        .build_layer(
            "script",
            serde_json::json!({ "code": "rgb(255, 0, 0)" }),
            4,
            0,
        )
        .unwrap();
    assert!(layer.render(4).iter().all(|p| *p == Rgb::new(255, 0, 0)));
}

#[test]
fn script_pixel_variable_is_in_scope() {
    let layer = build_registry()
        .build_layer(
            "script",
            serde_json::json!({ "code": "if pixel == 0 { rgb(255, 0, 0) } else { rgb(0, 0, 0) }" }),
            4,
            0,
        )
        .unwrap();
    let frame = layer.render(4);
    assert_eq!(frame[0], Rgb::new(255, 0, 0));
    assert_eq!(frame[1], Rgb::BLACK);
}

#[test]
fn script_compile_error_returns_err() {
    let result =
        build_registry().build_layer("script", serde_json::json!({ "code": "@@@ invalid" }), 4, 0);
    assert!(result.is_err());
}

#[test]
fn script_unknown_layer_name_returns_err() {
    assert!(
        build_registry()
            .build_layer("nonexistent", serde_json::json!({}), 4, 0)
            .is_err()
    );
}

#[test]
fn wave_returns_normalized_sine() {
    let layer = build_registry()
        .build_layer(
            "script",
            serde_json::json!({ "code": "if wave(0.0) > 0.49 && wave(0.0) < 0.51 { rgb(255, 0, 0) } else { rgb(0, 0, 0) }" }),
            1,
            0,
        )
        .unwrap();
    assert_eq!(layer.render(1)[0], Rgb::new(255, 0, 0));
}

#[test]
fn smoothstep_clamps_below_lo() {
    let layer = build_registry()
        .build_layer(
            "script",
            serde_json::json!({ "code": "if smoothstep(10.0, 20.0, 5.0) == 0.0 { rgb(255, 0, 0) } else { rgb(0, 0, 0) }" }),
            1,
            0,
        )
        .unwrap();
    assert_eq!(layer.render(1)[0], Rgb::new(255, 0, 0));
}

#[test]
fn smoothstep_clamps_above_hi() {
    let layer = build_registry()
        .build_layer(
            "script",
            serde_json::json!({ "code": "if smoothstep(10.0, 20.0, 25.0) == 1.0 { rgb(255, 0, 0) } else { rgb(0, 0, 0) }" }),
            1,
            0,
        )
        .unwrap();
    assert_eq!(layer.render(1)[0], Rgb::new(255, 0, 0));
}

#[test]
fn remap_maps_range() {
    let layer = build_registry()
        .build_layer(
            "script",
            serde_json::json!({ "code": "if remap(0.5, 0.0, 1.0, 0.0, 100.0) == 50.0 { rgb(255, 0, 0) } else { rgb(0, 0, 0) }" }),
            1,
            0,
        )
        .unwrap();
    assert_eq!(layer.render(1)[0], Rgb::new(255, 0, 0));
}

#[test]
fn build_composite_creates_valid_effect() {
    use domain::BlendMode;
    use effects::scene_builder::LayerSpec;
    use effects::{LiveParam, build_composite};
    use std::collections::HashMap;
    use std::sync::Arc;

    let params = HashMap::new();
    let specs = [LayerSpec {
        script: "rgb(255, 0, 0)",
        param_defs: &[],
        params: &params,
        blend_mode: BlendMode::Override,
        zone_start: 0,
        zone_end: 4,
        zone_transition: 0,
    }];
    let brightness = Arc::new(LiveParam::new(255.0f32, f32::MAX));
    let primary = Arc::new(LiveParam::new(Rgb::BLACK, f32::MAX));
    let composite = build_composite(&specs, 4, brightness, primary).unwrap();
    let frame = composite.frames(&[Rgb::BLACK; 4]).next().unwrap();
    assert!(frame.iter().all(|p| *p == Rgb::new(255, 0, 0)));
}
