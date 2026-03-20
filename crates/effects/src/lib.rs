pub mod builtin;
pub(crate) mod compositor;
pub mod error;
pub(crate) mod rhai;
pub mod scene_builder;

pub use builtin::{BuiltinEffect, load_builtins};
pub use compositor::live_param::LiveParam;
pub use compositor::{composite, layer, live_param};
pub use error::EffectError;
pub use scene_builder::build_composite;

pub fn validate_builtin_scripts() -> Vec<String> {
    let engine = rhai::make_script_engine();
    let mut errors = Vec::new();
    for effect in load_builtins() {
        if let Err(e) = engine.compile(&effect.script) {
            errors.push(format!("builtin '{}': {}", effect.slug, e));
        }
    }
    errors
}
