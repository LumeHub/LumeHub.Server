use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;

use super::bus::ParameterBus;
use super::layer::LayerEffect;
use crate::error::EffectError;

pub type LayerBuilder =
    fn(Value, Arc<ParameterBus>, &str) -> Result<Arc<dyn LayerEffect>, EffectError>;

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
        prelude: &str,
    ) -> Result<Arc<dyn LayerEffect>, EffectError> {
        let builder = self
            .layer_builders
            .get(name)
            .ok_or_else(|| EffectError::UnknownEffect(name.to_string()))?;
        builder(params, bus, prelude)
    }
}
