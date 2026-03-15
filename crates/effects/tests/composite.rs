use std::sync::Arc;

use domain::{BlendMode, Rgb};
use effects::LiveParam;
use effects::composite::CompositeEffect;
use effects::layer::{CompositeLayer, LayerEffect, ZoneGradient};
use engine::Effect;

struct Solid(Rgb);

impl LayerEffect for Solid {
    fn render(&self, len: usize) -> Vec<Rgb> {
        vec![self.0; len]
    }
}

fn brightness(val: f32) -> Arc<LiveParam<f32>> {
    Arc::new(LiveParam::new(val, f32::MAX))
}

fn full_layer(color: Rgb, mode: BlendMode, strip_len: usize) -> CompositeLayer {
    CompositeLayer {
        effect: Arc::new(Solid(color)),
        mode,
        zone: ZoneGradient::full_strip(strip_len),
    }
}

#[test]
fn override_mode_fills_frame_with_layer_color() {
    let effect = CompositeEffect {
        layers: vec![full_layer(Rgb::new(255, 0, 0), BlendMode::Override, 4)],
        brightness: brightness(255.0),
    };
    let frame = effect.frames(&[Rgb::BLACK; 4]).next().unwrap();
    assert!(frame.iter().all(|p| *p == Rgb::new(255, 0, 0)));
}

#[test]
fn add_mode_saturates_channels() {
    let effect = CompositeEffect {
        layers: vec![
            full_layer(Rgb::new(200, 0, 0), BlendMode::Override, 4),
            full_layer(Rgb::new(100, 0, 0), BlendMode::Add, 4),
        ],
        brightness: brightness(255.0),
    };
    let frame = effect.frames(&[Rgb::BLACK; 4]).next().unwrap();
    assert!(frame.iter().all(|p| p.r == 255 && p.g == 0 && p.b == 0));
}

#[test]
fn zero_brightness_yields_black_frame() {
    let effect = CompositeEffect {
        layers: vec![full_layer(Rgb::new(255, 255, 255), BlendMode::Override, 4)],
        brightness: brightness(0.0),
    };
    let frame = effect.frames(&[Rgb::BLACK; 4]).next().unwrap();
    assert!(frame.iter().all(|p| *p == Rgb::BLACK));
}

#[test]
fn zone_restricts_rendering_to_pixel_range() {
    let zone = ZoneGradient {
        start_pixel: 2,
        end_pixel: 4,
        transition_length: 0,
    };
    let effect = CompositeEffect {
        layers: vec![CompositeLayer {
            effect: Arc::new(Solid(Rgb::new(255, 0, 0))),
            mode: BlendMode::Override,
            zone,
        }],
        brightness: brightness(255.0),
    };
    let frame = effect.frames(&[Rgb::BLACK; 4]).next().unwrap();
    assert_eq!(frame[0], Rgb::BLACK);
    assert_eq!(frame[1], Rgb::BLACK);
    assert_eq!(frame[2], Rgb::new(255, 0, 0));
    assert_eq!(frame[3], Rgb::new(255, 0, 0));
}

#[test]
fn transition_extends_outside_logical_zone() {
    // Zone covers pixels 3-7 on a 10-pixel strip with tl=2.
    // Render range: 1-9. Interior 3-6 is full opacity.
    // Leading extension 1-2 fades in; trailing extension 7-8 fades out.
    let zone = ZoneGradient {
        start_pixel: 3,
        end_pixel: 7,
        transition_length: 2,
    };
    let effect = CompositeEffect {
        layers: vec![CompositeLayer {
            effect: Arc::new(Solid(Rgb::new(255, 0, 0))),
            mode: BlendMode::Override,
            zone,
        }],
        brightness: brightness(255.0),
    };
    let frame = effect.frames(&[Rgb::BLACK; 10]).next().unwrap();
    // Logical interior: full red
    assert_eq!(frame[3], Rgb::new(255, 0, 0));
    assert_eq!(frame[6], Rgb::new(255, 0, 0));
    // Leading extension fades in (dimmer than logical interior)
    assert!(
        frame[1].r < frame[3].r,
        "fade-in: pixel 1 dimmer than interior"
    );
    assert!(
        frame[2].r < frame[3].r,
        "fade-in: pixel 2 dimmer than interior"
    );
    // Trailing extension fades out (render_end=9, fade over 7-8; pixel 7 is first extended pixel)
    assert!(
        frame[8].r < frame[6].r,
        "fade-out: pixel 8 dimmer than interior"
    );
    // Outside render range: untouched black
    assert_eq!(frame[0], Rgb::BLACK);
    assert_eq!(frame[9], Rgb::BLACK);
}

#[test]
fn adjacent_zones_blend_without_black() {
    // Zone A: 0-5 (red), Zone B: 5-10 (blue), tl=2.
    // In blend region 3-6 (A's trailing ext 5-6, B's leading ext 3-4):
    // Zone A's last pixel extends into B's territory and fades out.
    // Zone B's first pixel extends into A's territory and fades in over fully-opaque A.
    // No pixel in 3-6 should be black.
    let zone_a = ZoneGradient {
        start_pixel: 0,
        end_pixel: 5,
        transition_length: 2,
    };
    let zone_b = ZoneGradient {
        start_pixel: 5,
        end_pixel: 10,
        transition_length: 2,
    };
    let effect = CompositeEffect {
        layers: vec![
            CompositeLayer {
                effect: Arc::new(Solid(Rgb::new(255, 0, 0))),
                mode: BlendMode::Override,
                zone: zone_a,
            },
            CompositeLayer {
                effect: Arc::new(Solid(Rgb::new(0, 0, 255))),
                mode: BlendMode::Override,
                zone: zone_b,
            },
        ],
        brightness: brightness(255.0),
    };
    let frame = effect.frames(&[Rgb::BLACK; 10]).next().unwrap();
    // Full red in A's interior
    assert_eq!(frame[0], Rgb::new(255, 0, 0));
    assert_eq!(frame[2], Rgb::new(255, 0, 0));
    // Full blue in B's interior
    assert_eq!(frame[7], Rgb::new(0, 0, 255));
    assert_eq!(frame[9], Rgb::new(0, 0, 255));
    // Blend region: no black (both channels together > 0 at all blend pixels)
    for (px, p) in frame.iter().enumerate().take(7).skip(3) {
        assert!(
            p.r > 0 || p.b > 0,
            "pixel {px} should not be black in blend region, got {:?}",
            p
        );
    }
}

#[test]
fn frames_iterator_is_infinite() {
    let effect = CompositeEffect {
        layers: vec![full_layer(Rgb::new(0, 255, 0), BlendMode::Override, 2)],
        brightness: brightness(255.0),
    };
    assert_eq!(effect.frames(&[Rgb::BLACK; 2]).take(1000).count(), 1000);
}
