use std::sync::Arc;

use domain::{BlendMode, Rgb};
use engine::Effect;

use super::bus::ParameterBus;
use super::layer::{CompositeLayer, OpacityGradient};

pub struct CompositeEffect {
    pub layers: Vec<CompositeLayer>,
    pub bus: Arc<ParameterBus>,
}

impl Effect for CompositeEffect {
    fn frames(&self, pixels: &[Rgb]) -> Box<dyn Iterator<Item = Vec<Rgb>> + Send + 'static> {
        let len = pixels.len();
        let layers = self.layers.clone();
        let bus = Arc::clone(&self.bus);

        Box::new(std::iter::from_fn(move || {
            bus.tick();
            let frame = layers.iter().fold(vec![Rgb::BLACK; len], |base, layer| {
                blend_layer(
                    base,
                    &layer.effect.render(len),
                    layer.mode,
                    layer.opacity_gradient,
                )
            });
            let brightness = bus.brightness.get() as u8;
            Some(
                frame
                    .into_iter()
                    .map(|c| c.with_brightness(brightness))
                    .collect(),
            )
        }))
    }
}

fn blend_layer(
    base: Vec<Rgb>,
    overlay: &[Rgb],
    mode: BlendMode,
    gradient: Option<OpacityGradient>,
) -> Vec<Rgb> {
    base.into_iter()
        .zip(overlay)
        .enumerate()
        .map(|(i, (b, &o))| blend_pixel(b, o, mode, gradient.map_or(1.0, |g| pixel_opacity(g, i))))
        .collect()
}

fn blend_pixel(base: Rgb, overlay: Rgb, mode: BlendMode, opacity: f32) -> Rgb {
    let o = overlay.dim(opacity);
    match mode {
        BlendMode::Override => base.lerp(o, opacity),
        BlendMode::Add => base + o,
        BlendMode::Screen => Rgb {
            r: 255 - ((255 - base.r as u16) * (255 - o.r as u16) / 255) as u8,
            g: 255 - ((255 - base.g as u16) * (255 - o.g as u16) / 255) as u8,
            b: 255 - ((255 - base.b as u16) * (255 - o.b as u16) / 255) as u8,
        },
        BlendMode::Multiply => Rgb {
            r: (base.r as u16 * o.r as u16 / 255) as u8,
            g: (base.g as u16 * o.g as u16 / 255) as u8,
            b: (base.b as u16 * o.b as u16 / 255) as u8,
        },
    }
}

fn pixel_opacity(gradient: OpacityGradient, i: usize) -> f32 {
    if i < gradient.start_pixel {
        return 0.0;
    }
    if gradient.end_pixel <= gradient.start_pixel || i >= gradient.end_pixel {
        return 1.0;
    }
    (i - gradient.start_pixel) as f32 / (gradient.end_pixel - gradient.start_pixel) as f32
}
