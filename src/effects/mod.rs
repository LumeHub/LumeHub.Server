use crate::color::Rgb;

pub mod color_loop;
pub mod fade_color;
pub mod queue;
pub mod registry;
pub mod sleep;
pub mod wake;
pub mod zone_interpolation;

pub use queue::EffectQueue;

pub trait Effect: Send {
    fn frames(&self, pixels: &[Rgb]) -> Box<dyn Iterator<Item = Vec<Rgb>> + Send + 'static>;
}

pub fn build_registry() -> registry::EffectRegistry {
    let mut registry = registry::EffectRegistry::new();
    zone_interpolation::register(&mut registry);
    registry
}
