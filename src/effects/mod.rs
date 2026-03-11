use domain::Rgb;

pub mod builder;
pub mod builtins;
pub mod color_loop;
pub mod composite;
pub mod config;
pub mod fade_in;
pub mod halt;
pub mod queue;
pub mod registry;
pub mod rhai;
pub mod script;
pub mod sleep;
pub mod solid_color;
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
