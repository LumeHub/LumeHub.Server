use std::sync::Arc;

use domain::Rgb;
use effects::bus::ParameterBus;
use effects::layer::LayerEffect;
use effects::registry::{EffectBuildError, EffectRegistry};
use serde_json::Value;

struct Solid;
impl LayerEffect for Solid {
    fn render(&self, len: usize) -> Vec<Rgb> {
        vec![Rgb::BLACK; len]
    }
}

fn solid_builder(
    _params: Value,
    _bus: Arc<ParameterBus>,
    _prelude: &str,
) -> Result<Arc<dyn LayerEffect>, EffectBuildError> {
    Ok(Arc::new(Solid))
}

#[test]
fn registered_layer_builds_successfully() {
    let mut registry = EffectRegistry::default();
    registry.register_layer("solid", solid_builder);

    let bus = Arc::new(ParameterBus::new(255.0, 255.0));
    let result = registry.build_layer("solid", Value::Null, bus, "");
    assert!(result.is_ok());
}

#[test]
fn unknown_layer_returns_error() {
    let registry = EffectRegistry::default();
    let bus = Arc::new(ParameterBus::new(255.0, 255.0));
    match registry.build_layer("does_not_exist", Value::Null, bus, "") {
        Err(e) => assert!(e.0.contains("does_not_exist")),
        Ok(_) => panic!("expected error for unknown layer"),
    }
}
