use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;

use crate::effects::Effect;
use crate::effects::composite::LayerEffect;
use crate::effects::composite::ParameterBus;
use crate::settings::BindingConfig;

pub type EffectBuilder = fn(Value) -> Result<Box<dyn Effect>, EffectBuildError>;

pub type LayerBuilder = fn(
    Value,
    &HashMap<String, BindingConfig>,
    &ParameterBus,
) -> Result<Arc<dyn LayerEffect>, EffectBuildError>;

#[derive(Debug)]
pub struct EffectBuildError(pub String);

#[derive(Clone, Default)]
pub struct EffectRegistry {
    builders: HashMap<&'static str, EffectBuilder>,
    layer_builders: HashMap<&'static str, LayerBuilder>,
}

impl EffectRegistry {
    pub fn register(&mut self, name: &'static str, builder: EffectBuilder) {
        self.builders.insert(name, builder);
    }

    pub fn register_layer(&mut self, name: &'static str, builder: LayerBuilder) {
        self.layer_builders.insert(name, builder);
    }

    pub fn build(&self, name: &str, params: Value) -> Result<Box<dyn Effect>, EffectBuildError> {
        let builder = self
            .builders
            .get(name)
            .ok_or_else(|| EffectBuildError(format!("unknown effect: {}", name)))?;
        builder(params)
    }

    pub fn build_layer(
        &self,
        name: &str,
        params: Value,
        bindings: &HashMap<String, BindingConfig>,
        bus: &ParameterBus,
    ) -> Result<Arc<dyn LayerEffect>, EffectBuildError> {
        let builder = self
            .layer_builders
            .get(name)
            .ok_or_else(|| EffectBuildError(format!("unknown layer effect: {}", name)))?;
        builder(params, bindings, bus)
    }
}
