use domain::{BlendMode, Rgb};
use effects::scene_builder::LayerSpec;
use effects::{LiveParam, build_composite};
use engine::effect::Effect;
use std::collections::HashMap;
use std::sync::Arc;

fn render(code: &str, len: usize) -> Vec<Rgb> {
    let params = HashMap::new();
    let specs = [LayerSpec {
        script: code,
        param_defs: &[],
        params: &params,
        blend_mode: BlendMode::Override,
        zone_start: 0,
        zone_end: len,
        zone_transition: 0,
    }];
    let brightness = Arc::new(LiveParam::new(255.0f32, f32::MAX));
    let primary = Arc::new(LiveParam::new(Rgb::BLACK, f32::MAX));
    let composite = build_composite(&specs, len, brightness, primary).unwrap();
    composite.frames(&vec![Rgb::BLACK; len]).next().unwrap()
}

#[test]
fn script_compile_error_returns_err() {
    let params = HashMap::new();
    let specs = [LayerSpec {
        script: "@@@ invalid",
        param_defs: &[],
        params: &params,
        blend_mode: BlendMode::Override,
        zone_start: 0,
        zone_end: 4,
        zone_transition: 0,
    }];
    let brightness = Arc::new(LiveParam::new(255.0f32, f32::MAX));
    let primary = Arc::new(LiveParam::new(Rgb::BLACK, f32::MAX));
    assert!(build_composite(&specs, 4, brightness, primary).is_err());
}

#[test]
fn script_renders_solid_color() {
    assert!(
        render("rgb(255, 0, 0)", 4)
            .iter()
            .all(|p| *p == Rgb::new(255, 0, 0))
    );
}

#[test]
fn script_pixel_variable_is_in_scope() {
    let frame = render("if pixel == 0 { rgb(255, 0, 0) } else { rgb(0, 0, 0) }", 4);
    assert_eq!(frame[0], Rgb::new(255, 0, 0));
    assert_eq!(frame[1], Rgb::BLACK);
}

#[test]
fn wave_returns_normalized_sine() {
    let px = render(
        "if wave(0.0) > 0.49 && wave(0.0) < 0.51 { rgb(255, 0, 0) } else { rgb(0, 0, 0) }",
        1,
    );
    assert_eq!(px[0], Rgb::new(255, 0, 0));
}

#[test]
fn smoothstep_clamps_below_lo() {
    let px = render(
        "if smoothstep(10.0, 20.0, 5.0) == 0.0 { rgb(255, 0, 0) } else { rgb(0, 0, 0) }",
        1,
    );
    assert_eq!(px[0], Rgb::new(255, 0, 0));
}

#[test]
fn smoothstep_clamps_above_hi() {
    let px = render(
        "if smoothstep(10.0, 20.0, 25.0) == 1.0 { rgb(255, 0, 0) } else { rgb(0, 0, 0) }",
        1,
    );
    assert_eq!(px[0], Rgb::new(255, 0, 0));
}

#[test]
fn remap_maps_range() {
    let px = render(
        "if remap(0.5, 0.0, 1.0, 0.0, 100.0) == 50.0 { rgb(255, 0, 0) } else { rgb(0, 0, 0) }",
        1,
    );
    assert_eq!(px[0], Rgb::new(255, 0, 0));
}
