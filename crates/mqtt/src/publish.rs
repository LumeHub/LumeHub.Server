use std::sync::Arc;

use application::{SceneRuntime, SceneSnapshot};
use effects::BuiltinEffect;
use rumqttc::{AsyncClient, QoS};
use store::Store;

use crate::topics::Topics;
use crate::types::{DeviceStatePayload, EffectPayload, LayerPayload, ScenePayload, ZonePayload};

pub async fn publish_device_state(
    client: &AsyncClient,
    topics: &Topics,
    snapshot: &SceneSnapshot,
    effect: Option<&str>,
) {
    let payload = DeviceStatePayload::from_snapshot(snapshot, effect);
    publish_retained(client, &topics.device_state(), &payload).await;
}

pub async fn publish_zones(client: &AsyncClient, topics: &Topics, store: &Store) {
    let Ok(zones) = store.get_zones().await else {
        return;
    };
    let payloads: Vec<ZonePayload> = zones.into_iter().map(Into::into).collect();
    publish_retained(client, &topics.zones(), &payloads).await;

    // Publish individual zone topics
    let Ok(zones) = store.get_zones().await else {
        return;
    };
    for z in zones {
        let topic = topics.zone(&z.id);
        let payload = ZonePayload::from(z);
        publish_retained(client, &topic, &payload).await;
    }
}

pub async fn publish_scenes(client: &AsyncClient, topics: &Topics, store: &Store) {
    let Ok(scenes) = store.get_scenes().await else {
        return;
    };
    let payloads: Vec<ScenePayload> = scenes.into_iter().map(Into::into).collect();
    publish_retained(client, &topics.scenes(), &payloads).await;
}

pub async fn publish_active_layers(client: &AsyncClient, topics: &Topics, store: &Store) {
    let Ok(layers) = store.get_active_layers().await else {
        return;
    };
    let payloads: Vec<LayerPayload> = layers.into_iter().map(Into::into).collect();
    publish_retained(client, &topics.active_layers(), &payloads).await;
}

pub async fn publish_effects(
    client: &AsyncClient,
    topics: &Topics,
    store: &Store,
    builtins: &[BuiltinEffect],
) {
    let mut payloads: Vec<EffectPayload> = builtins.iter().map(Into::into).collect();
    if let Ok(user_effects) = store.get_effects().await {
        payloads.extend(user_effects.into_iter().map(Into::into));
    }
    publish_retained(client, &topics.effects(), &payloads).await;
}

pub async fn publish_all_initial(
    client: &AsyncClient,
    topics: &Topics,
    runtime: &Arc<dyn SceneRuntime>,
    store: &Store,
    builtins: &[BuiltinEffect],
) {
    let snapshot = runtime.snapshot();
    publish_device_state(client, topics, &snapshot, None).await;
    publish_zones(client, topics, store).await;
    publish_scenes(client, topics, store).await;
    publish_active_layers(client, topics, store).await;
    publish_effects(client, topics, store, builtins).await;
}

pub async fn publish_retained<T: serde::Serialize>(client: &AsyncClient, topic: &str, value: &T) {
    match serde_json::to_vec(value) {
        Ok(payload) => {
            if let Err(e) = client.publish(topic, QoS::AtLeastOnce, true, payload).await {
                eprintln!("[mqtt] publish error on {topic}: {e}");
            }
        }
        Err(e) => eprintln!("[mqtt] serialize error on {topic}: {e}"),
    }
}
