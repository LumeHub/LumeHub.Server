pub mod live_param;
pub mod signal_script;

pub use live_param::{PRIMARY_COLOR, ParameterBus, SECONDARY_COLOR};

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::effects::Effect;
use domain::{BlendMode, Rgb};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct OpacityGradient {
    pub start_pixel: usize,
    pub end_pixel: usize,
}

pub trait LayerEffect: Send + Sync + 'static {
    fn render(&self, len: usize) -> Vec<Rgb>;
}

pub struct CompositeLayer {
    pub effect: Arc<dyn LayerEffect>,
    pub mode: BlendMode,
    pub opacity_gradient: Option<OpacityGradient>,
}

pub struct CompositeEffect {
    pub layers: Vec<CompositeLayer>,
    pub bus: Arc<ParameterBus>,
}

impl Effect for CompositeEffect {
    fn frames(&self, pixels: &[Rgb]) -> Box<dyn Iterator<Item = Vec<Rgb>> + Send + 'static> {
        let len = pixels.len();
        let layers: Vec<(Arc<dyn LayerEffect>, BlendMode, Option<OpacityGradient>)> = self
            .layers
            .iter()
            .map(|l| (Arc::clone(&l.effect), l.mode.clone(), l.opacity_gradient))
            .collect();
        let bus = Arc::clone(&self.bus);

        Box::new(std::iter::from_fn(move || {
            bus.tick();

            let composite =
                layers
                    .iter()
                    .fold(vec![Rgb::BLACK; len], |base, (layer, mode, gradient)| {
                        let overlay = layer.render(len);
                        base.into_iter()
                            .zip(overlay)
                            .enumerate()
                            .map(|(i, (b, o))| {
                                let opacity = gradient.map_or(1.0, |g| pixel_opacity(g, i));
                                match mode {
                                    BlendMode::Override => b.lerp(o, opacity),
                                    BlendMode::Add => Rgb {
                                        r: b.r.saturating_add((o.r as f32 * opacity) as u8),
                                        g: b.g.saturating_add((o.g as f32 * opacity) as u8),
                                        b: b.b.saturating_add((o.b as f32 * opacity) as u8),
                                    },
                                }
                            })
                            .collect()
                    });

            let brightness = bus.brightness.get() as u8;
            let composite = composite
                .into_iter()
                .map(|c| c.with_brightness(brightness))
                .collect();

            Some(composite)
        }))
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
