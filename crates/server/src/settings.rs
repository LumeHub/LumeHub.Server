use std::path::PathBuf;

use clap::Parser;
use config::{Config, ConfigError, Environment, File};
use drivers::DriverConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LedControllerConfig {
    #[serde(flatten)]
    pub driver: DriverConfig,
    pub crossfade_ms: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    pub ip_address: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MdnsConfig {
    pub enable: bool,
    pub port: u16,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Settings {
    pub led_controller: LedControllerConfig,
    pub server: ServerConfig,
    pub mdns: MdnsConfig,
    #[serde(skip)]
    pub config_dir: PathBuf,
    #[serde(skip)]
    pub state_dir: PathBuf,
}

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, env = "LUMEHUB_CONFIG")]
    config: Option<String>,

    #[arg(long, env = "LUMEHUB_CONFIG_DIR", default_value = "config")]
    config_dir: String,

    #[arg(long, env = "LUMEHUB_STATE_DIR", default_value = ".")]
    state_dir: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let args = Args::parse();

        let s = Config::builder()
            .add_source(config::File::from_str(
                include_str!("../../../config/default.toml"),
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
        settings.state_dir = PathBuf::from(args.state_dir);
        Ok(settings)
    }
}
