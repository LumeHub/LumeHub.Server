use std::collections::HashMap;
use std::path::{Path, PathBuf};

use include_dir::{Dir, include_dir};

static STOCK_FUNCTIONS: Dir = include_dir!("$CARGO_MANIFEST_DIR/config/functions");
static STOCK_EFFECTS: Dir = include_dir!("$CARGO_MANIFEST_DIR/config/effects");

use clap::Parser;
use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::effects::composite::OpacityGradient;

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ControllerType {
    Console,
    Ws2801,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LedControllerConfig {
    pub controller_type: ControllerType,
    pub pixel_count: usize,
    pub spi_path: Option<String>,
    pub freq_hz: Option<u32>,
    pub latch_time_micros: Option<u64>,
}

/// Default value for a named color signal on the parameter bus.
/// `Color` sets a static initial color (optionally with interpolation speed).
/// `Script` sets an animated signal driven by a Rhai expression evaluated each
/// frame; scope variables: `time` (f64), `frame` (i64), `pi`.
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
    /// Interpolation speed (units/frame). Defaults to 5.0.
    pub speed: Option<f32>,
}

/// A single layer in a composite effect preset.
///
/// All layer-specific parameters (e.g. `code`, `speed`, `tail_length`) are
/// written flat alongside `effect`/`mode`/`opacity_gradient` — no nested
/// `[params]` sub-table required.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LayerPreset {
    pub effect: String,
    pub mode: Option<String>,
    pub opacity_gradient: Option<OpacityGradient>,
    #[serde(flatten)]
    pub params: HashMap<String, Value>,
}

/// An effect preset consisting of one or more composited layers.
///
/// The optional `signals` table declares default values for named color
/// signals on the parameter bus.  Use this to pre-register custom signal
/// names (so scripts can reference them before any `set_signal` call) or to
/// set preset-level defaults that the user can override at runtime.
///
/// Signal priority (highest wins): runtime `set_signal` > preset `signals` default.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EffectPreset {
    /// Named signal defaults.  Keys can be any signal name; well-known names
    /// are `primary_color` and `secondary_color`.
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
    /// Build the effects config by:
    /// 1. Loading stock functions and presets embedded at compile time.
    /// 2. Overlaying any `.rhai` / `.toml` files found in `<dir>/functions/`
    ///    and `<dir>/effects/` at runtime — user files override stock ones with
    ///    the same name, and new files are added alongside them.
    pub fn from_dir(dir: &Path) -> Self {
        let mut functions = HashMap::new();
        let mut presets = HashMap::new();

        // --- stock (embedded at compile time) ---
        for file in STOCK_FUNCTIONS.files() {
            if file.path().extension().and_then(|e| e.to_str()) == Some("rhai") {
                if let (Some(name), Some(code)) = (
                    file.path().file_stem().and_then(|s| s.to_str()),
                    file.contents_utf8(),
                ) {
                    functions.insert(name.to_string(), code.to_string());
                }
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
                    Err(e) => eprintln!("Warning: stock effect '{}' failed to parse: {}", name, e),
                }
            }
        }

        // --- user overrides / additions (runtime) ---
        let fn_dir = dir.join("functions");
        if let Ok(entries) = std::fs::read_dir(&fn_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("rhai") {
                    let name = match path.file_stem().and_then(|s| s.to_str()) {
                        Some(n) => n.to_string(),
                        None => continue,
                    };
                    match std::fs::read_to_string(&path) {
                        Ok(code) => {
                            functions.insert(name, code);
                        }
                        Err(e) => eprintln!("Warning: could not read {:?}: {}", path, e),
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
                let name = match path.file_stem().and_then(|s| s.to_str()) {
                    Some(n) => n.to_string(),
                    None => continue,
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
                        presets.insert(name, preset);
                    }
                    Err(e) => eprintln!("Warning: could not load effect {:?}: {}", path, e),
                }
            }
        }

        EffectsConfig { presets, functions }
    }

    /// Wrap a bare Rhai script as a single-layer script preset.
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
        Config::builder()
            .add_source(config::File::from_str(content, config::FileFormat::Toml))
            .build()
            .and_then(|c| c.try_deserialize::<EffectPreset>())
            .map_err(|e| e.to_string())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    pub ip_address: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Settings {
    pub led_controller: LedControllerConfig,
    pub server: ServerConfig,
    /// Resolved at startup from `--config-dir`; not read from TOML.
    #[serde(skip)]
    pub config_dir: PathBuf,
}

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, env = "LUMEHUB_CONFIG")]
    config: Option<String>,

    /// Directory containing `functions/` and `effects/` sub-folders.
    #[arg(long, env = "LUMEHUB_CONFIG_DIR", default_value = "config")]
    config_dir: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let args = Args::parse();

        let s = Config::builder()
            .add_source(config::File::from_str(
                include_str!("../config/default.toml"),
                config::FileFormat::Toml,
            ))
            .add_source(
                File::with_name(&args.config.unwrap_or_else(|| "config.toml".to_string()))
                    .required(false),
            )
            .add_source(Environment::with_prefix("LUMEHUB"))
            .build()?;

        let mut settings: Self = s.try_deserialize()?;
        settings.config_dir = PathBuf::from(args.config_dir);
        Ok(settings)
    }
}
