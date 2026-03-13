use std::sync::Arc;

use serde::{Deserialize, Serialize};

use domain::{BlendMode, Rgb};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct OpacityGradient {
    pub start_pixel: usize,
    pub end_pixel: usize,
}

pub trait LayerEffect: Send + Sync + 'static {
    fn render(&self, len: usize) -> Vec<Rgb>;
}

#[derive(Clone)]
pub struct CompositeLayer {
    pub effect: Arc<dyn LayerEffect>,
    pub mode: BlendMode,
    pub opacity_gradient: Option<OpacityGradient>,
}
