pub mod config;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::layer::OpacityGradient;
use domain::Rgb;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum SignalDef {
    Color(SignalColorDef),
    Script(String),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SignalColorDef {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub speed: Option<f32>,
}

impl From<&SignalColorDef> for Rgb {
    fn from(c: &SignalColorDef) -> Self {
        Rgb::new(c.r, c.g, c.b)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LayerPreset {
    pub effect: String,
    pub mode: Option<String>,
    pub opacity_gradient: Option<OpacityGradient>,
    #[serde(flatten)]
    pub params: HashMap<String, Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EffectPreset {
    #[serde(default)]
    pub signals: HashMap<String, SignalDef>,
    pub layers: Vec<LayerPreset>,
}

impl EffectPreset {
    pub(crate) fn single_script(code: &str) -> Self {
        EffectPreset {
            signals: HashMap::new(),
            layers: vec![LayerPreset {
                effect: "script".to_string(),
                mode: None,
                opacity_gradient: None,
                params: [("code".to_string(), Value::String(code.to_string()))].into(),
            }],
        }
    }
}
