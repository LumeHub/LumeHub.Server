use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;

use crate::effects::composite::LayerEffect;
use crate::effects::composite::ParameterBus;

pub type LayerBuilder =
    fn(Value, Arc<ParameterBus>) -> Result<Arc<dyn LayerEffect>, EffectBuildError>;

#[derive(Debug)]
pub struct EffectBuildError(pub String);

#[derive(Clone, Default)]
pub struct EffectRegistry {
    layer_builders: HashMap<&'static str, LayerBuilder>,
}

impl EffectRegistry {
    pub fn register_layer(&mut self, name: &'static str, builder: LayerBuilder) {
        self.layer_builders.insert(name, builder);
    }

    pub fn build_layer(
        &self,
        name: &str,
        params: Value,
        bus: Arc<ParameterBus>,
    ) -> Result<Arc<dyn LayerEffect>, EffectBuildError> {
        let builder = self
            .layer_builders
            .get(name)
            .ok_or_else(|| EffectBuildError(format!("unknown layer effect: {}", name)))?;
        builder(params, bus)
    }
}
