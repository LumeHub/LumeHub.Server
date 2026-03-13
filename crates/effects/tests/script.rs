use std::collections::HashMap;
use std::sync::Arc;

use domain::Rgb;
use effects::build_registry;
use effects::builder::{InitialState, build_composite};
use effects::bus::ParameterBus;
use effects::preset::{EffectPreset, LayerPreset};
use engine::Effect;

fn make_bus() -> Arc<ParameterBus> {
    Arc::new(ParameterBus::new(255.0, 255.0))
}

fn preset(code: &str) -> EffectPreset {
    let mut params = HashMap::new();
    params.insert(
        "code".to_string(),
        serde_json::Value::String(code.to_string()),
    );
    EffectPreset {
        signals: HashMap::new(),
        layers: vec![LayerPreset {
            effect: "script".to_string(),
            mode: None,
            opacity_gradient: None,
            params,
        }],
    }
}

#[test]
fn script_renders_solid_color() {
    let layer = build_registry()
        .build_layer(
            "script",
            serde_json::json!({ "code": "rgb(255, 0, 0)" }),
            make_bus(),
            "",
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
            make_bus(),
            "",
        )
        .unwrap();
    let frame = layer.render(4);
    assert_eq!(frame[0], Rgb::new(255, 0, 0));
    assert_eq!(frame[1], Rgb::BLACK);
}

#[test]
fn script_compile_error_returns_err() {
    let result = build_registry().build_layer(
        "script",
        serde_json::json!({ "code": "this is not valid rhai !!!" }),
        make_bus(),
        "",
    );
    assert!(result.is_err());
}

#[test]
fn script_unknown_layer_name_returns_err() {
    assert!(
        build_registry()
            .build_layer("nonexistent", serde_json::json!({}), make_bus(), "")
            .is_err()
    );
}

#[test]
fn build_composite_initializes_primary_color_from_initial() {
    let initial = Rgb::new(100, 50, 25);
    let state = InitialState {
        color: initial,
        brightness: 255,
        brightness_speed: 255.0,
        signal_colors: HashMap::new(),
    };
    let (composite, bus) = build_composite(
        &build_registry(),
        &preset("rgb(0, 0, 0)"),
        &state,
        String::new(),
        &HashMap::new(),
    )
    .unwrap();

    assert_eq!(bus.all_colors()["primary_color"], initial);
    assert!(composite.frames(&[Rgb::BLACK; 4]).next().is_some());
}

#[test]
fn prelude_functions_are_available_in_script() {
    let layer = build_registry()
        .build_layer(
            "script",
            serde_json::json!({ "code": "rgb(double_red(100), 0, 0)" }),
            make_bus(),
            "fn double_red(r) { r * 2 }",
        )
        .unwrap();
    assert!(layer.render(1).iter().all(|p| p.r == 200));
}
