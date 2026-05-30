use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MqttConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    #[serde(default = "default_device_id")]
    pub device_id: String,
    #[serde(default = "default_topic_prefix")]
    pub topic_prefix: String,
    #[serde(default = "default_true")]
    pub ha_discovery: bool,
    #[serde(default = "default_ha_prefix")]
    pub ha_discovery_prefix: String,
}

impl Default for MqttConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            host: default_host(),
            port: default_port(),
            username: None,
            password: None,
            device_id: default_device_id(),
            topic_prefix: default_topic_prefix(),
            ha_discovery: true,
            ha_discovery_prefix: default_ha_prefix(),
        }
    }
}

fn default_host() -> String {
    "localhost".into()
}
fn default_port() -> u16 {
    1883
}
fn default_device_id() -> String {
    "lumehub".into()
}
fn default_topic_prefix() -> String {
    "lumehub".into()
}
fn default_ha_prefix() -> String {
    "homeassistant".into()
}
fn default_true() -> bool {
    true
}
