use crate::ctx::Ctx;
use crate::discovery::publish_ha_discovery;
use crate::publish::{publish_retained, publish_scenes};
use crate::types::{CreateSceneCommand, ScenePayload, UpdateSceneCommand};

pub async fn handle_create(ctx: &Ctx, payload: &[u8]) {
    let Ok(cmd) = serde_json::from_slice::<CreateSceneCommand>(payload) else {
        eprintln!("[mqtt] scenes/create: invalid JSON");
        return;
    };
    match ctx.store.create_scene(&cmd.name).await {
        Ok(scene) => {
            let topic = ctx.topics.scene(&scene.id);
            publish_retained(&ctx.client, &topic, &ScenePayload::from(scene)).await;
            publish_scenes(&ctx.client, &ctx.topics, &ctx.store).await;
            republish_discovery_if_enabled(ctx).await;
        }
        Err(e) => eprintln!("[mqtt] scenes/create error: {e}"),
    }
}

pub async fn handle_update(ctx: &Ctx, id: &str, payload: &[u8]) {
    let Ok(cmd) = serde_json::from_slice::<UpdateSceneCommand>(payload) else {
        eprintln!("[mqtt] scenes/{id}/update: invalid JSON");
        return;
    };
    match ctx.store.update_scene(id, &cmd.name).await {
        Ok(scene) => {
            let topic = ctx.topics.scene(&scene.id);
            publish_retained(&ctx.client, &topic, &ScenePayload::from(scene)).await;
            publish_scenes(&ctx.client, &ctx.topics, &ctx.store).await;
            republish_discovery_if_enabled(ctx).await;
        }
        Err(e) => eprintln!("[mqtt] scenes/{id}/update error: {e}"),
    }
}

pub async fn handle_delete(ctx: &Ctx, id: &str) {
    match ctx.store.delete_scene(id).await {
        Ok(()) => {
            publish_retained(&ctx.client, &ctx.topics.scene(id), &serde_json::Value::Null).await;
            publish_scenes(&ctx.client, &ctx.topics, &ctx.store).await;
            republish_discovery_if_enabled(ctx).await;
        }
        Err(e) => eprintln!("[mqtt] scenes/{id}/delete error: {e}"),
    }
}

pub async fn handle_load(ctx: &Ctx, id: &str) {
    match ctx.store.load_scene_into_active(id).await {
        Ok(()) => {
            ctx.runtime.reload_active();
            crate::publish::publish_active_layers(&ctx.client, &ctx.topics, &ctx.store).await;
        }
        Err(e) => eprintln!("[mqtt] scenes/{id}/load error: {e}"),
    }
}

pub async fn handle_save(ctx: &Ctx, id: &str) {
    match ctx.store.overwrite_scene_from_active(id).await {
        Ok(()) => {}
        Err(e) => eprintln!("[mqtt] scenes/{id}/save error: {e}"),
    }
}

async fn republish_discovery_if_enabled(ctx: &Ctx) {
    if ctx.config.ha_discovery {
        publish_ha_discovery(&ctx.client, &ctx.config, &ctx.store, &ctx.builtins).await;
    }
}
