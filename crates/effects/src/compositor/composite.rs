use std::sync::Arc;

use domain::{BlendMode, Rgb};
use engine::Effect;

use super::layer::{CompositeLayer, ZoneGradient};
use super::live_param::LiveParam;

pub struct CompositeEffect {
    pub layers: Vec<CompositeLayer>,
    pub brightness: Arc<LiveParam<f32>>,
    pub primary_color: Arc<LiveParam<Rgb>>,
}

impl Effect for CompositeEffect {
    fn frames(&self, pixels: &[Rgb]) -> Box<dyn Iterator<Item = Vec<Rgb>> + Send + 'static> {
        let strip_len = pixels.len();
        let layers = self.layers.clone();
        let brightness = Arc::clone(&self.brightness);
        let primary_color = Arc::clone(&self.primary_color);

        Box::new(std::iter::from_fn(move || {
            brightness.tick();
            primary_color.tick();
            for layer in &layers {
                layer.opacity.tick();
            }
            let first = first_visible_layer(&layers, strip_len);
            let frame = layers[first..]
                .iter()
                .fold(vec![Rgb::BLACK; strip_len], |base, layer| {
                    let overlay = layer.effect.render(layer.zone.zone_len());
                    blend_layer(base, &overlay, layer.mode, &layer.zone, layer.opacity.get())
                });
            let b = brightness.get() as u8;
            Some(frame.into_iter().map(|c| c.with_brightness(b)).collect())
        }))
    }
}

fn first_visible_layer(layers: &[CompositeLayer], strip_len: usize) -> usize {
    layers
        .iter()
        .enumerate()
        .rev()
        .find(|(_, l)| {
            l.mode == BlendMode::Override
                && l.zone.start_pixel == 0
                && l.zone.end_pixel >= strip_len
                && l.zone.transition_length == 0
                && l.opacity.get() >= 1.0
        })
        .map(|(i, _)| i)
        .unwrap_or(0)
}

fn blend_layer(
    base: Vec<Rgb>,
    overlay: &[Rgb],
    mode: BlendMode,
    zone: &ZoneGradient,
    layer_opacity: f32,
) -> Vec<Rgb> {
    if overlay.is_empty() || layer_opacity <= 0.0 {
        return base;
    }
    let strip_len = base.len();
    let (rstart, rend) = (
        zone.render_start().min(strip_len),
        zone.render_end(strip_len),
    );
    base.into_iter()
        .enumerate()
        .map(
            |(strip_px, base_px)| match strip_px < rstart || strip_px >= rend {
                true => base_px,
                false => {
                    let idx = strip_px
                        .saturating_sub(zone.start_pixel)
                        .min(overlay.len() - 1);
                    blend_pixel(
                        base_px,
                        overlay[idx],
                        mode,
                        pixel_opacity(zone, strip_px, strip_len) * layer_opacity,
                    )
                }
            },
        )
        .collect()
}

fn blend_pixel(base: Rgb, overlay: Rgb, mode: BlendMode, opacity: f32) -> Rgb {
    // Each blend mode produces a fully-blended pixel at opacity=1.0; opacity
    // then lerps between base (no effect) and the blended result.
    let blended = match mode {
        BlendMode::Override => overlay,
        BlendMode::Add => base + overlay,
        BlendMode::Screen => Rgb {
            r: 255 - ((255 - base.r as u16) * (255 - overlay.r as u16) / 255) as u8,
            g: 255 - ((255 - base.g as u16) * (255 - overlay.g as u16) / 255) as u8,
            b: 255 - ((255 - base.b as u16) * (255 - overlay.b as u16) / 255) as u8,
        },
        BlendMode::Multiply => Rgb {
            r: (base.r as u16 * overlay.r as u16 / 255) as u8,
            g: (base.g as u16 * overlay.g as u16 / 255) as u8,
            b: (base.b as u16 * overlay.b as u16 / 255) as u8,
        },
    };
    base.lerp(blended, opacity)
}

fn pixel_opacity(zone: &ZoneGradient, strip_px: usize, strip_len: usize) -> f32 {
    let tl = zone.transition_length;
    if tl == 0 {
        return 1.0;
    }
    let render_start = zone.start_pixel.saturating_sub(tl);
    let render_end = (zone.end_pixel + tl).min(strip_len);

    let fade_in = match render_start == zone.start_pixel || strip_px >= zone.start_pixel {
        true => 1.0,
        false => (strip_px - render_start) as f32 / (zone.start_pixel - render_start) as f32,
    };
    let fade_out = match render_end == zone.end_pixel || strip_px < zone.end_pixel {
        true => 1.0,
        false => (render_end - strip_px) as f32 / (render_end - zone.end_pixel) as f32,
    };
    fade_in.min(fade_out)
}
