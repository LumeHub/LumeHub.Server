use crate::color::Rgb;

pub mod fade_color;
pub mod queue;
pub mod zone_interpolation;

pub use queue::EffectQueue;

pub trait Effect: Send {
    fn frames(&self, pixels: &[Rgb]) -> Box<dyn Iterator<Item = Vec<Rgb>>>;
}
