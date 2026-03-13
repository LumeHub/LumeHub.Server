use std::sync::Arc;

use domain::{BlendMode, Rgb};
use effects::bus::ParameterBus;
use effects::composite::CompositeEffect;
use effects::layer::{CompositeLayer, LayerEffect, OpacityGradient};
use engine::Effect;

struct Solid(Rgb);
impl LayerEffect for Solid {
    fn render(&self, len: usize) -> Vec<Rgb> {
        vec![self.0; len]
    }
}

fn bus(brightness: f32) -> Arc<ParameterBus> {
    Arc::new(ParameterBus::new(brightness, 255.0))
}

fn single_layer(color: Rgb, mode: BlendMode) -> CompositeEffect {
    CompositeEffect {
        layers: vec![CompositeLayer {
            effect: Arc::new(Solid(color)),
            mode,
            opacity_gradient: None,
        }],
        bus: bus(255.0),
    }
}

#[test]
fn override_mode_fills_frame_with_layer_color() {
    let effect = single_layer(Rgb::new(255, 0, 0), BlendMode::Override);
    let frame = effect.frames(&[Rgb::BLACK; 4]).next().unwrap();
    assert!(frame.iter().all(|p| *p == Rgb::new(255, 0, 0)));
}

#[test]
fn add_mode_saturates_channels() {
    let effect = CompositeEffect {
        layers: vec![
            CompositeLayer {
                effect: Arc::new(Solid(Rgb::new(200, 0, 0))),
                mode: BlendMode::Override,
                opacity_gradient: None,
            },
            CompositeLayer {
                effect: Arc::new(Solid(Rgb::new(100, 0, 0))),
                mode: BlendMode::Add,
                opacity_gradient: None,
            },
        ],
        bus: bus(255.0),
    };
    let frame = effect.frames(&[Rgb::BLACK; 4]).next().unwrap();
    assert!(frame.iter().all(|p| p.r == 255 && p.g == 0 && p.b == 0));
}

#[test]
fn zero_brightness_yields_black_frame() {
    let effect = single_layer(Rgb::new(255, 255, 255), BlendMode::Override);
    let effect = CompositeEffect {
        bus: bus(0.0),
        ..effect
    };
    let frame = effect.frames(&[Rgb::BLACK; 4]).next().unwrap();
    assert!(frame.iter().all(|p| *p == Rgb::BLACK));
}

#[test]
fn opacity_gradient_fades_in_layer_across_pixels() {
    let effect = CompositeEffect {
        layers: vec![CompositeLayer {
            effect: Arc::new(Solid(Rgb::new(255, 0, 0))),
            mode: BlendMode::Override,
            opacity_gradient: Some(OpacityGradient {
                start_pixel: 0,
                end_pixel: 4,
            }),
        }],
        bus: bus(255.0),
    };
    let frame = effect.frames(&[Rgb::BLACK; 4]).next().unwrap();
    assert!(frame[3].r > frame[0].r);
}

#[test]
fn frames_iterator_is_infinite() {
    let effect = single_layer(Rgb::new(0, 255, 0), BlendMode::Override);
    assert_eq!(effect.frames(&[Rgb::BLACK; 2]).take(1000).count(), 1000);
}
