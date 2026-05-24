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

fn primary() -> Arc<LiveParam<Rgb>> {
    Arc::new(LiveParam::new(Rgb::BLACK, f32::MAX))
}

fn full_layer(color: Rgb, mode: BlendMode, strip_len: usize) -> CompositeLayer {
    CompositeLayer {
        effect: Arc::new(Solid(color)),
        mode,
        zone: ZoneGradient::full_strip(strip_len),
        opacity: Arc::new(LiveParam::new(1.0, f32::MAX)),
    }
}

fn full_layer_with_opacity(
    color: Rgb,
    mode: BlendMode,
    strip_len: usize,
    opacity: f32,
) -> CompositeLayer {
    CompositeLayer {
        effect: Arc::new(Solid(color)),
        mode,
        zone: ZoneGradient::full_strip(strip_len),
        opacity: Arc::new(LiveParam::new(opacity, f32::MAX)),
    }
}

#[test]
fn override_mode_fills_frame_with_layer_color() {
    let effect = CompositeEffect {
        layers: vec![full_layer(Rgb::new(255, 0, 0), BlendMode::Override, 4)],
        brightness: brightness(255.0),
        primary_color: primary(),
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
        primary_color: primary(),
    };
    let frame = effect.frames(&[Rgb::BLACK; 4]).next().unwrap();
    assert!(frame.iter().all(|p| p.r == 255 && p.g == 0 && p.b == 0));
}

#[test]
fn zero_brightness_yields_black_frame() {
    let effect = CompositeEffect {
        layers: vec![full_layer(Rgb::new(255, 255, 255), BlendMode::Override, 4)],
        brightness: brightness(0.0),
        primary_color: primary(),
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
            opacity: Arc::new(LiveParam::new(1.0, f32::MAX)),
        }],
        brightness: brightness(255.0),
        primary_color: primary(),
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
            opacity: Arc::new(LiveParam::new(1.0, f32::MAX)),
        }],
        brightness: brightness(255.0),
        primary_color: primary(),
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
                opacity: Arc::new(LiveParam::new(1.0, f32::MAX)),
            },
            CompositeLayer {
                effect: Arc::new(Solid(Rgb::new(0, 0, 255))),
                mode: BlendMode::Override,
                zone: zone_b,
                opacity: Arc::new(LiveParam::new(1.0, f32::MAX)),
            },
        ],
        brightness: brightness(255.0),
        primary_color: primary(),
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
fn layer_opacity_zero_passes_through_base() {
    let effect = CompositeEffect {
        layers: vec![
            full_layer(Rgb::new(255, 0, 0), BlendMode::Override, 4),
            full_layer_with_opacity(Rgb::new(0, 0, 255), BlendMode::Override, 4, 0.0),
        ],
        brightness: brightness(255.0),
        primary_color: primary(),
    };
    let frame = effect.frames(&[Rgb::BLACK; 4]).next().unwrap();
    assert!(frame.iter().all(|p| *p == Rgb::new(255, 0, 0)));
}

#[test]
fn layer_opacity_half_blends_with_base() {
    let effect = CompositeEffect {
        layers: vec![
            full_layer(Rgb::new(0, 0, 0), BlendMode::Override, 4),
            full_layer_with_opacity(Rgb::new(200, 0, 0), BlendMode::Override, 4, 0.5),
        ],
        brightness: brightness(255.0),
        primary_color: primary(),
    };
    let frame = effect.frames(&[Rgb::BLACK; 4]).next().unwrap();
    // Override at opacity 0.5 lerps base toward overlay halfway.
    assert!(
        frame
            .iter()
            .all(|p| p.r > 40 && p.r < 120 && p.g == 0 && p.b == 0)
    );
}

#[test]
fn layer_opacity_one_equals_full_strength() {
    let with = CompositeEffect {
        layers: vec![full_layer_with_opacity(
            Rgb::new(123, 45, 67),
            BlendMode::Override,
            4,
            1.0,
        )],
        brightness: brightness(255.0),
        primary_color: primary(),
    };
    let without = CompositeEffect {
        layers: vec![full_layer(Rgb::new(123, 45, 67), BlendMode::Override, 4)],
        brightness: brightness(255.0),
        primary_color: primary(),
    };
    assert_eq!(
        with.frames(&[Rgb::BLACK; 4]).next().unwrap(),
        without.frames(&[Rgb::BLACK; 4]).next().unwrap()
    );
}

#[test]
fn layer_opacity_scales_add_mode_contribution() {
    let full = CompositeEffect {
        layers: vec![
            full_layer(Rgb::new(50, 0, 0), BlendMode::Override, 4),
            full_layer(Rgb::new(100, 0, 0), BlendMode::Add, 4),
        ],
        brightness: brightness(255.0),
        primary_color: primary(),
    };
    let dimmed = CompositeEffect {
        layers: vec![
            full_layer(Rgb::new(50, 0, 0), BlendMode::Override, 4),
            full_layer_with_opacity(Rgb::new(100, 0, 0), BlendMode::Add, 4, 0.5),
        ],
        brightness: brightness(255.0),
        primary_color: primary(),
    };
    let full_px = full.frames(&[Rgb::BLACK; 4]).next().unwrap()[0];
    let dim_px = dimmed.frames(&[Rgb::BLACK; 4]).next().unwrap()[0];
    assert!(
        dim_px.r < full_px.r,
        "opacity 0.5 should shrink Add contribution: full={} dim={}",
        full_px.r,
        dim_px.r
    );
}

#[test]
fn opacity_live_param_tween_ramps_overlay_across_frames() {
    // speed=0.25 means 4 ticks to reach target 1.0 starting from 0.0.
    let opacity = Arc::new(LiveParam::new(0.0, 0.25));
    opacity.set(1.0);
    let layer = CompositeLayer {
        effect: Arc::new(Solid(Rgb::new(200, 0, 0))),
        mode: BlendMode::Override,
        zone: ZoneGradient::full_strip(1),
        opacity: Arc::clone(&opacity),
    };
    let effect = CompositeEffect {
        layers: vec![layer],
        brightness: brightness(255.0),
        primary_color: primary(),
    };
    let mut frames = effect.frames(&[Rgb::BLACK; 1]);

    let r1 = frames.next().unwrap()[0].r;
    let r2 = frames.next().unwrap()[0].r;
    let r3 = frames.next().unwrap()[0].r;
    let r4 = frames.next().unwrap()[0].r;
    assert!(r1 < r2, "frame1 ({r1}) should be dimmer than frame2 ({r2})");
    assert!(r2 < r3, "frame2 ({r2}) should be dimmer than frame3 ({r3})");
    assert!(r3 < r4, "frame3 ({r3}) should be dimmer than frame4 ({r4})");
    assert_eq!(r4, 200, "frame4 should reach full target");
}

#[test]
fn opacity_live_param_set_speed_changes_tween_rate() {
    let opacity = Arc::new(LiveParam::new(0.0, 0.1));
    opacity.set(1.0);
    // Tick once at speed 0.1: current = 0.1
    opacity.tick();
    assert!((opacity.get() - 0.1).abs() < 1e-5);
    // Bump speed to 1.0; next tick snaps to target
    opacity.set_speed(1.0);
    opacity.tick();
    assert_eq!(opacity.get(), 1.0);
}

#[test]
fn frames_iterator_is_infinite() {
    let effect = CompositeEffect {
        layers: vec![full_layer(Rgb::new(0, 255, 0), BlendMode::Override, 2)],
        brightness: brightness(255.0),
        primary_color: primary(),
    };
    assert_eq!(effect.frames(&[Rgb::BLACK; 2]).take(1000).count(), 1000);
}
