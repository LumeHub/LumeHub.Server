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

pub fn validate_scripts(config: &config::EffectsConfig, prelude: &str) -> Vec<String> {
    let engine = rhai::make_script_engine();
    let mut errors = Vec::new();
    for (preset_name, preset) in &config.presets {
        for (i, layer) in preset.layers.iter().enumerate() {
            if layer.effect != "script" {
                continue;
            }
            let Some(code) = layer.params.get("code").and_then(|v| v.as_str()) else {
                continue;
            };
            let full_code = format!("{prelude}\n{code}");
            if let Err(e) = engine.compile(&full_code) {
                errors.push(format!("preset '{}' layer {}: {}", preset_name, i, e));
            }
        }
    }
    errors
}
