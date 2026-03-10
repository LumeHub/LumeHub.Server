use crate::color::Rgb;

pub mod builder;
pub mod color_loop;
pub mod composite;
pub mod config;
pub mod fade_color;
pub mod queue;
pub mod registry;
pub mod rhai;
pub mod script;
pub mod sleep;
pub mod wake;

pub use queue::EffectQueue;

pub trait Effect: Send {
    fn frames(&self, pixels: &[Rgb]) -> Box<dyn Iterator<Item = Vec<Rgb>> + Send + 'static>;
}

pub fn build_registry() -> registry::EffectRegistry {
    let mut registry = registry::EffectRegistry::default();
    script::register(&mut registry);
    registry
}
