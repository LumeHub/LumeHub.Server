use mqtt::config::MqttConfig;
use serde_json::json;

#[test]
fn default_config_has_expected_values() {
    let c = MqttConfig::default();
    assert!(!c.enabled);
    assert_eq!(c.host, "localhost");
    assert_eq!(c.port, 1883);
    assert_eq!(c.device_id, "lumehub");
    assert_eq!(c.topic_prefix, "lumehub");
    assert!(c.ha_discovery);
    assert_eq!(c.ha_discovery_prefix, "homeassistant");
}

#[test]
fn config_deserializes_partial() {
    let json = json!({"enabled": true, "host": "192.168.1.5", "port": 1884});
    let c: MqttConfig = serde_json::from_value(json).unwrap();
    assert!(c.enabled);
    assert_eq!(c.host, "192.168.1.5");
    assert_eq!(c.port, 1884);
    assert_eq!(c.topic_prefix, "lumehub");
}
