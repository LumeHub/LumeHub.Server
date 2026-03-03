use std::collections::HashMap;

use serde_json::Value;

use crate::effects::Effect;

pub type EffectBuilder = fn(Value) -> Result<Box<dyn Effect>, EffectBuildError>;

#[derive(Debug)]
pub struct EffectBuildError(pub String);

#[derive(Clone)]
pub struct EffectRegistry {
    builders: HashMap<&'static str, EffectBuilder>,
}

impl EffectRegistry {
    pub fn new() -> Self {
        Self {
            builders: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &'static str, builder: EffectBuilder) {
        self.builders.insert(name, builder);
    }

    pub fn build(&self, name: &str, params: Value) -> Result<Box<dyn Effect>, EffectBuildError> {
        let builder = self
            .builders
            .get(name)
            .ok_or_else(|| EffectBuildError(format!("unknown effect: {}", name)))?;
        builder(params)
    }
}
