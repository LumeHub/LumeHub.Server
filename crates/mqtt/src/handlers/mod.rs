pub mod active_layers;
pub mod device;
pub mod effects;
pub mod scenes;
pub mod zones;

use rumqttc::Publish;

use crate::ctx::Ctx;

pub(crate) async fn dispatch(ctx: &Ctx, p: Publish) {
    route(ctx, p.topic.as_str(), &p.payload).await;
}

pub async fn route(ctx: &Ctx, topic: &str, payload: &[u8]) {
    let t = &ctx.topics;

    // Device
    if topic == t.device_set() {
        device::handle_set(ctx, payload).await;
        return;
    }

    // Zones
    if topic == t.zones_create() {
        zones::handle_create(ctx, payload).await;
        return;
    }
    if let Some(id) = t.extract_id(topic, "zones", "update") {
        zones::handle_update(ctx, id, payload).await;
        return;
    }
    if let Some(id) = t.extract_id(topic, "zones", "delete") {
        zones::handle_delete(ctx, id).await;
        return;
    }

    // Scenes
    if topic == t.scenes_create() {
        scenes::handle_create(ctx, payload).await;
        return;
    }
    if let Some(id) = t.extract_id(topic, "scenes", "update") {
        scenes::handle_update(ctx, id, payload).await;
        return;
    }
    if let Some(id) = t.extract_id(topic, "scenes", "delete") {
        scenes::handle_delete(ctx, id).await;
        return;
    }
    if let Some(id) = t.extract_id(topic, "scenes", "load") {
        scenes::handle_load(ctx, id).await;
        return;
    }
    if let Some(id) = t.extract_id(topic, "scenes", "save") {
        scenes::handle_save(ctx, id).await;
        return;
    }

    // Active layers
    if topic == t.active_layers_set() {
        active_layers::handle_set(ctx, payload).await;
        return;
    }
    if topic == t.active_layers_clear() {
        active_layers::handle_clear(ctx).await;
        return;
    }
    if topic == t.active_layers_add() {
        active_layers::handle_add(ctx, payload).await;
        return;
    }
    if topic == t.active_layers_reorder() {
        active_layers::handle_reorder(ctx, payload).await;
        return;
    }
    if let Some(id) = t.extract_id(topic, "active_layers", "update") {
        active_layers::handle_update(ctx, id, payload).await;
        return;
    }
    if let Some(id) = t.extract_id(topic, "active_layers", "delete") {
        active_layers::handle_delete(ctx, id).await;
        return;
    }

    // Effects
    if topic == t.effects_create() {
        effects::handle_create(ctx, payload).await;
        return;
    }
    if let Some(id) = t.extract_id(topic, "effects", "update") {
        effects::handle_update(ctx, id, payload).await;
        return;
    }
    if let Some(id) = t.extract_id(topic, "effects", "delete") {
        effects::handle_delete(ctx, id).await;
        return;
    }

    // Active scene select (HA)
    if topic == format!("{}/active_scene/set", t.prefix) {
        handle_active_scene_set(ctx, payload).await;
    }
}

async fn handle_active_scene_set(ctx: &Ctx, payload: &[u8]) {
    let name = match std::str::from_utf8(payload) {
        Ok(s) => s.trim(),
        Err(_) => return,
    };
    let state_topic = format!("{}/active_scene/state", ctx.topics.prefix);
    if name.is_empty() {
        active_layers::handle_clear(ctx).await;
        crate::publish::publish_retained(&ctx.client, &state_topic, &"").await;
        return;
    }
    let scenes = match ctx.store.get_scenes().await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[mqtt] active_scene/set: store error: {e}");
            return;
        }
    };
    match scenes.into_iter().find(|s| s.name == name) {
        Some(scene) => {
            scenes::handle_load(ctx, &scene.id.clone()).await;
            crate::publish::publish_retained(&ctx.client, &state_topic, &name).await;
        }
        None => eprintln!("[mqtt] active_scene/set: no scene named '{name}'"),
    }
}
