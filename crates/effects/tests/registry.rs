use std::sync::Arc;

use domain::Rgb;
use effects::EffectError;
use effects::layer::LayerEffect;
use effects::registry::EffectRegistry;
use serde_json::Value;

struct Solid;

impl LayerEffect for Solid {
    fn render(&self, len: usize) -> Vec<Rgb> {
        vec![Rgb::BLACK; len]
    }
}

fn solid_builder(
    _params: Value,
    _strip_len: usize,
    _zone_start: usize,
) -> Result<Arc<dyn LayerEffect>, EffectError> {
    Ok(Arc::new(Solid))
}

#[test]
fn registered_layer_builds_successfully() {
    let mut registry = EffectRegistry::default();
    registry.register_layer("solid", solid_builder);

    let result = registry.build_layer("solid", Value::Null, 10, 0);
    assert!(result.is_ok());
}

#[test]
fn unknown_layer_returns_error() {
    let registry = EffectRegistry::default();
    match registry.build_layer("does_not_exist", Value::Null, 10, 0) {
        Err(EffectError::UnknownEffect(name)) => assert!(name.contains("does_not_exist")),
        Err(e) => panic!("expected UnknownEffect, got: {:?}", e),
        Ok(_) => panic!("expected error for unknown layer"),
    }
}
