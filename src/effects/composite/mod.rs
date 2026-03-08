pub mod live_param;

pub use live_param::{BRIGHTNESS, LiveParam, PRIMARY_COLOR, ParameterBus, SECONDARY_COLOR};

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::color::Rgb;
use crate::effects::Effect;

pub enum ColorSource {
    Static(Rgb),
    Live(LiveParam<Rgb>),
}

impl ColorSource {
    pub fn get(&self) -> Rgb {
        match self {
            ColorSource::Static(c) => *c,
            ColorSource::Live(p) => p.get(),
        }
    }
}

#[derive(Clone)]
pub enum CompositeMode {
    Override,
    Add,
}

impl CompositeMode {
    pub fn from_str(s: &str) -> Self {
        match s {
            "add" => CompositeMode::Add,
            _ => CompositeMode::Override,
        }
    }
}

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
    pub mode: CompositeMode,
    pub opacity_gradient: Option<OpacityGradient>,
}

pub struct CompositeEffect {
    pub layers: Vec<CompositeLayer>,
    pub bus: Arc<ParameterBus>,
}

impl Effect for CompositeEffect {
    fn frames(&self, pixels: &[Rgb]) -> Box<dyn Iterator<Item = Vec<Rgb>> + Send + 'static> {
        let len = pixels.len();
        let layers: Vec<(Arc<dyn LayerEffect>, CompositeMode, Option<OpacityGradient>)> = self
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
                    .fold(vec![Rgb::BLACK; len], |acc, (layer, mode, opacity)| {
                        let frame = match opacity {
                            Some(g) => apply_opacity(layer.render(len), *g),
                            None => layer.render(len),
                        };
                        apply_mode(acc, frame, mode)
                    });

            let composite = match bus.get_scalar(live_param::BRIGHTNESS) {
                Some(b) => {
                    let brightness = b.get().clamp(0.0, 100.0) as u8;
                    composite
                        .into_iter()
                        .map(|c| c.with_brightness(brightness))
                        .collect()
                }
                None => composite,
            };

            Some(composite)
        }))
    }
}

fn apply_opacity(frame: Vec<Rgb>, gradient: OpacityGradient) -> Vec<Rgb> {
    if gradient.end_pixel <= gradient.start_pixel {
        return frame;
    }
    let range = (gradient.end_pixel - gradient.start_pixel) as f32;
    frame
        .into_iter()
        .enumerate()
        .map(|(i, c)| {
            if i < gradient.start_pixel {
                Rgb::BLACK
            } else if i >= gradient.end_pixel {
                c
            } else {
                let t = (i - gradient.start_pixel) as f32 / range;
                Rgb::BLACK.lerp(c, t)
            }
        })
        .collect()
}

fn apply_mode(base: Vec<Rgb>, overlay: Vec<Rgb>, mode: &CompositeMode) -> Vec<Rgb> {
    match mode {
        CompositeMode::Override => overlay,
        CompositeMode::Add => base
            .into_iter()
            .zip(overlay)
            .map(|(b, o)| Rgb {
                r: b.r.saturating_add(o.r),
                g: b.g.saturating_add(o.g),
                b: b.b.saturating_add(o.b),
            })
            .collect(),
    }
}
