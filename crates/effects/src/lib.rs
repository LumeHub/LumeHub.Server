pub(crate) mod compositor;
pub mod preset;
pub(crate) mod rhai;
pub(crate) mod script;

pub use compositor::{builder, bus, composite, layer, live_param, registry};
pub use preset::config;

pub fn build_registry() -> registry::EffectRegistry {
    let mut registry = registry::EffectRegistry::default();
    script::register(&mut registry);
    registry
}
