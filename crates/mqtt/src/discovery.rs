use effects::BuiltinEffect;
use rumqttc::{AsyncClient, QoS};
use serde_json::json;
use store::Store;

use crate::config::MqttConfig;
use crate::topics::Topics;

pub async fn publish_ha_discovery(
    client: &AsyncClient,
    config: &MqttConfig,
    store: &Store,
    builtins: &[BuiltinEffect],
) {
    let ha_prefix = &config.ha_discovery_prefix;
    let device_id = &config.device_id;
    let topics = Topics::new(&config.topic_prefix);

    let device_info = json!({
        "identifiers": [device_id],
        "name": "LumeHub",
        "manufacturer": "LumeHub",
        "model": "LED Controller"
    });

    let effect_list: Vec<&str> = builtins.iter().map(|b| b.name.as_str()).collect();

    let light_config = json!({
        "name": "LumeHub",
        "unique_id": format!("{device_id}_light"),
        "state_topic": topics.device_state(),
        "command_topic": topics.device_set(),
        "schema": "json",
        "brightness": true,
        "color_mode": true,
        "supported_color_modes": ["rgb"],
        "effect": true,
        "effect_list": effect_list,
        "device": device_info
    });

    publish_discovery_config(
        client,
        &format!("{ha_prefix}/light/{device_id}/config"),
        &light_config,
    )
    .await;

    // Separate select for saved scenes.
    let mut scene_options = vec![""];
    let scenes = store.get_scenes().await.unwrap_or_default();
    let scene_names: Vec<String> = scenes.into_iter().map(|s| s.name).collect();
    scene_options.extend(scene_names.iter().map(String::as_str));

    let select_config = json!({
        "name": "Scene",
        "unique_id": format!("{device_id}_scene"),
        "state_topic": format!("{}/active_scene/state", config.topic_prefix),
        "command_topic": format!("{}/active_scene/set", config.topic_prefix),
        "options": scene_options,
        "device": device_info
    });

    publish_discovery_config(
        client,
        &format!("{ha_prefix}/select/{device_id}_scene/config"),
        &select_config,
    )
    .await;
}

async fn publish_discovery_config(client: &AsyncClient, topic: &str, payload: &serde_json::Value) {
    match serde_json::to_vec(payload) {
        Ok(bytes) => {
            if let Err(e) = client.publish(topic, QoS::AtLeastOnce, true, bytes).await {
                eprintln!("[mqtt] discovery publish error on {topic}: {e}");
            }
        }
        Err(e) => eprintln!("[mqtt] discovery serialize error: {e}"),
    }
}
