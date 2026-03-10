use std::collections::HashMap;
use std::path::Path;

use include_dir::{Dir, include_dir};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::effects::composite::OpacityGradient;

static STOCK_FUNCTIONS: Dir = include_dir!("$CARGO_MANIFEST_DIR/config/functions");
static STOCK_EFFECTS: Dir = include_dir!("$CARGO_MANIFEST_DIR/config/effects");

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

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct EffectsConfig {
    #[serde(default)]
    pub presets: HashMap<String, EffectPreset>,
    #[serde(default)]
    pub functions: HashMap<String, String>,
}

impl EffectsConfig {
    pub fn from_dir(dir: &Path) -> Self {
        let mut functions = HashMap::new();
        let mut presets = HashMap::new();

        for file in STOCK_FUNCTIONS.files() {
            if file.path().extension().and_then(|e| e.to_str()) == Some("rhai")
                && let (Some(name), Some(code)) = (
                    file.path().file_stem().and_then(|s| s.to_str()),
                    file.contents_utf8(),
                )
            {
                functions.insert(name.to_string(), code.to_string());
            }
        }
        for file in STOCK_EFFECTS.files() {
            if let (Some(name), Some(content)) = (
                file.path().file_stem().and_then(|s| s.to_str()),
                file.contents_utf8(),
            ) {
                let ext = file.path().extension().and_then(|e| e.to_str());
                let result = match ext {
                    Some("toml") => Self::parse_preset(content),
                    Some("rhai") => Ok(Self::script_preset(content)),
                    _ => continue,
                };
                match result {
                    Ok(preset) => {
                        presets.insert(name.to_string(), preset);
                    }
                    Err(e) => eprintln!("warning: stock effect '{}' failed to parse: {}", name, e),
                }
            }
        }

        let fn_dir = dir.join("functions");
        if let Ok(entries) = std::fs::read_dir(&fn_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("rhai")
                    && let Some(name) = path.file_stem().and_then(|s| s.to_str())
                {
                    match std::fs::read_to_string(&path) {
                        Ok(code) => {
                            functions.insert(name.to_string(), code);
                        }
                        Err(e) => eprintln!("warning: could not read {:?}: {}", path, e),
                    }
                }
            }
        }
        let fx_dir = dir.join("effects");
        if let Ok(entries) = std::fs::read_dir(&fx_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let ext = path.extension().and_then(|e| e.to_str());
                if !matches!(ext, Some("toml") | Some("rhai")) {
                    continue;
                }
                let Some(name) = path.file_stem().and_then(|s| s.to_str()) else {
                    continue;
                };
                let result = std::fs::read_to_string(&path)
                    .map_err(|e| e.to_string())
                    .and_then(|content| match ext {
                        Some("toml") => Self::parse_preset(&content),
                        Some("rhai") => Ok(Self::script_preset(&content)),
                        _ => unreachable!(),
                    });
                match result {
                    Ok(preset) => {
                        presets.insert(name.to_string(), preset);
                    }
                    Err(e) => eprintln!("warning: could not load effect {:?}: {}", path, e),
                }
            }
        }

        EffectsConfig { presets, functions }
    }

    fn script_preset(code: &str) -> EffectPreset {
        let mut params = HashMap::new();
        params.insert("code".to_string(), Value::String(code.to_string()));
        EffectPreset {
            signals: HashMap::new(),
            layers: vec![LayerPreset {
                effect: "script".to_string(),
                mode: None,
                opacity_gradient: None,
                params,
            }],
        }
    }

    fn parse_preset(content: &str) -> Result<EffectPreset, String> {
        config::Config::builder()
            .add_source(config::File::from_str(content, config::FileFormat::Toml))
            .build()
            .and_then(|c| c.try_deserialize::<EffectPreset>())
            .map_err(|e| e.to_string())
    }
}
