use std::collections::HashMap;

use clap::Parser;
use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EffectPreset {
    pub effect: String,
    pub params: Value,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct EffectsConfig {
    #[serde(default)]
    pub presets: HashMap<String, EffectPreset>,
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
    #[serde(default)]
    pub effects: EffectsConfig,
}

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, env = "LUMEHUB_CONFIG")]
    config: Option<String>,
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

        s.try_deserialize()
    }
}
