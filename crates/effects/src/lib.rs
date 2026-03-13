pub(crate) mod compositor;
pub mod preset;
pub(crate) mod rhai;

pub use compositor::{builder, bus, composite, layer, live_param, registry};
pub use preset::config;

pub fn build_registry() -> registry::EffectRegistry {
    let mut registry = registry::EffectRegistry::default();
    rhai::script::register(&mut registry);
    registry
}
