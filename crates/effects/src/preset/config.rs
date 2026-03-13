use std::collections::HashMap;
use std::path::Path;

use include_dir::{Dir, include_dir};
use serde::{Deserialize, Serialize};

use super::EffectPreset;

static STOCK_FUNCTIONS: Dir = include_dir!("$CARGO_MANIFEST_DIR/config/functions");
static STOCK_EFFECTS: Dir = include_dir!("$CARGO_MANIFEST_DIR/config/effects");

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct EffectsConfig {
    #[serde(default)]
    pub presets: HashMap<String, EffectPreset>,
    #[serde(default)]
    pub functions: HashMap<String, String>,
}

impl EffectsConfig {
    pub fn from_dir(dir: &Path) -> Self {
        let mut config = load_embedded();
        let fs = load_from_fs(dir);
        config.functions.extend(fs.functions);
        config.presets.extend(fs.presets);
        config
    }
}

fn load_embedded() -> EffectsConfig {
    let functions = STOCK_FUNCTIONS
        .files()
        .filter(|f| f.path().extension().and_then(|e| e.to_str()) == Some("rhai"))
        .filter_map(|f| {
            Some((
                f.path().file_stem()?.to_str()?.to_string(),
                f.contents_utf8()?.to_string(),
            ))
        })
        .collect();

    let presets = STOCK_EFFECTS
        .files()
        .filter_map(|f| {
            let name = f.path().file_stem()?.to_str()?.to_string();
            let content = f.contents_utf8()?;
            let preset = match f.path().extension()?.to_str()? {
                "toml" => parse_preset(content).ok()?,
                "rhai" => EffectPreset::single_script(content),
                _ => return None,
            };
            Some((name, preset))
        })
        .collect();

    EffectsConfig { presets, functions }
}

fn load_from_fs(dir: &Path) -> EffectsConfig {
    let functions = std::fs::read_dir(dir.join("functions"))
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| e.path().extension().and_then(|e| e.to_str()) == Some("rhai"))
        .filter_map(|e| {
            let path = e.path();
            let name = path.file_stem()?.to_str()?.to_string();
            Some((name, std::fs::read_to_string(&path).ok()?))
        })
        .collect();

    let presets = std::fs::read_dir(dir.join("effects"))
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| {
            matches!(
                e.path().extension().and_then(|e| e.to_str()),
                Some("toml" | "rhai")
            )
        })
        .filter_map(|e| {
            let path = e.path();
            let name = path.file_stem()?.to_str()?.to_string();
            let content = std::fs::read_to_string(&path).ok()?;
            let preset = match path.extension().and_then(|e| e.to_str())? {
                "toml" => match parse_preset(&content) {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("warning: skipping {:?}: {}", path, e);
                        return None;
                    }
                },
                _ => EffectPreset::single_script(&content),
            };
            Some((name, preset))
        })
        .collect();

    EffectsConfig { presets, functions }
}

fn parse_preset(content: &str) -> Result<EffectPreset, config::ConfigError> {
    config::Config::builder()
        .add_source(config::File::from_str(content, config::FileFormat::Toml))
        .build()
        .and_then(|c| c.try_deserialize::<EffectPreset>())
}
