use std::sync::Arc;

use domain::{BlendMode, Rgb};

use super::live_param::LiveParam;

pub trait LayerEffect: Send + Sync + 'static {
    fn render(&self, len: usize) -> Vec<Rgb>;
}

#[derive(Clone, Copy, Debug)]
pub struct ZoneGradient {
    pub start_pixel: usize,
    pub end_pixel: usize,
    pub transition_length: usize,
}

impl ZoneGradient {
    pub fn full_strip(len: usize) -> Self {
        Self {
            start_pixel: 0,
            end_pixel: len,
            transition_length: 0,
        }
    }

    pub fn zone_len(&self) -> usize {
        self.end_pixel.saturating_sub(self.start_pixel)
    }

    pub fn render_start(&self) -> usize {
        self.start_pixel.saturating_sub(self.transition_length)
    }

    pub fn render_end(&self, strip_len: usize) -> usize {
        (self.end_pixel + self.transition_length).min(strip_len)
    }
}

#[derive(Clone)]
pub struct CompositeLayer {
    pub effect: Arc<dyn LayerEffect>,
    pub mode: BlendMode,
    pub zone: ZoneGradient,
    pub opacity: Arc<LiveParam<f32>>,
}
