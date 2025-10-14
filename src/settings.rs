use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};

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
pub struct ServerConfig {
    pub ip_address: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Settings {
    pub led_controller: LedControllerConfig,
    pub server: ServerConfig,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(config::File::from_str(
                include_str!("../config/default.toml"),
                config::FileFormat::Toml,
            ))
            .add_source(File::with_name("config.toml").required(false))
            .add_source(Environment::with_prefix("LUMEHUB"))
            .build()?;

        s.try_deserialize()
    }
}
