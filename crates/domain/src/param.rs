use serde::{Deserialize, Serialize};

use crate::Rgb;

/// What UI control the app should render for a parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ParamControl {
    Slider {
        min: f32,
        max: f32,
        step: Option<f32>,
    },
    Color,
    Toggle,
    Select {
        options: Vec<String>,
    },
}

/// A typed parameter value — used both for effect defaults and layer overrides.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ParamValue {
    Number(f32),
    Color(Rgb),
    Bool(bool),
    Select(String),
}

/// Definition of a single parameter on an effect.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamDef {
    /// Variable name injected into the Rhai script scope.
    pub name: String,
    /// Human-readable label shown in the app.
    pub label: String,
    pub control: ParamControl,
    pub default: ParamValue,
}
