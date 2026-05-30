use store::NewLayer;

use crate::ctx::Ctx;
use crate::publish::{publish_active_layers, publish_retained};
use crate::types::{AddLayerCommand, LayerPayload, ReorderLayersCommand, UpdateLayerCommand};

pub async fn handle_set(ctx: &Ctx, payload: &[u8]) {
    let Ok(cmds) = serde_json::from_slice::<Vec<AddLayerCommand>>(payload) else {
        eprintln!("[mqtt] active_layers/set: invalid JSON (expected array)");
        return;
    };
    let layers: Vec<NewLayer> = cmds
        .into_iter()
        .map(|c| NewLayer {
            effect_id: c.effect_id,
            zone_id: c.zone_id,
            blend_mode: c.blend_mode,
            params: c.params,
        })
        .collect();
    match ctx.store.replace_active_layers(&layers).await {
        Ok(()) => {
            ctx.runtime.reload_active();
            publish_active_layers(&ctx.client, &ctx.topics, &ctx.store).await;
        }
        Err(e) => eprintln!("[mqtt] active_layers/set error: {e}"),
    }
}

pub async fn handle_clear(ctx: &Ctx) {
    match ctx.store.clear_active_scene().await {
        Ok(()) => {
            ctx.runtime.halt();
            publish_active_layers(&ctx.client, &ctx.topics, &ctx.store).await;
        }
        Err(e) => eprintln!("[mqtt] active_layers/clear error: {e}"),
    }
}

pub async fn handle_add(ctx: &Ctx, payload: &[u8]) {
    let Ok(cmd) = serde_json::from_slice::<AddLayerCommand>(payload) else {
        eprintln!("[mqtt] active_layers/add: invalid JSON");
        return;
    };
    match ctx
        .store
        .add_active_layer(&cmd.effect_id, &cmd.zone_id, cmd.blend_mode, &cmd.params)
        .await
    {
        Ok(layer) => {
            ctx.runtime.reload_active();
            let topic = format!("{}/active_layers/{}", ctx.topics.prefix, layer.id);
            publish_retained(&ctx.client, &topic, &LayerPayload::from(layer)).await;
            publish_active_layers(&ctx.client, &ctx.topics, &ctx.store).await;
        }
        Err(e) => eprintln!("[mqtt] active_layers/add error: {e}"),
    }
}

pub async fn handle_reorder(ctx: &Ctx, payload: &[u8]) {
    let Ok(cmd) = serde_json::from_slice::<ReorderLayersCommand>(payload) else {
        eprintln!("[mqtt] active_layers/reorder: invalid JSON");
        return;
    };
    match ctx.store.reorder_active_layers(&cmd.ordered_ids).await {
        Ok(()) => {
            ctx.runtime.reload_active();
            publish_active_layers(&ctx.client, &ctx.topics, &ctx.store).await;
        }
        Err(e) => eprintln!("[mqtt] active_layers/reorder error: {e}"),
    }
}

pub async fn handle_update(ctx: &Ctx, id: &str, payload: &[u8]) {
    let Ok(cmd) = serde_json::from_slice::<UpdateLayerCommand>(payload) else {
        eprintln!("[mqtt] active_layers/{id}/update: invalid JSON");
        return;
    };
    let current = match ctx.store.get_active_layer(id).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("[mqtt] active_layers/{id}/update: layer not found: {e}");
            return;
        }
    };
    let zone_id = cmd.zone_id.as_deref().unwrap_or(&current.zone_id);
    let enabled = cmd.enabled.unwrap_or(current.enabled);
    let blend_mode = cmd.blend_mode.unwrap_or(current.blend_mode);
    let params = cmd.params.unwrap_or(current.params);
    match ctx
        .store
        .update_active_layer(id, zone_id, enabled, blend_mode, &params)
        .await
    {
        Ok(_) => {
            ctx.runtime.reload_active();
            publish_active_layers(&ctx.client, &ctx.topics, &ctx.store).await;
        }
        Err(e) => eprintln!("[mqtt] active_layers/{id}/update error: {e}"),
    }
}

pub async fn handle_delete(ctx: &Ctx, id: &str) {
    match ctx.store.remove_active_layer(id).await {
        Ok(()) => {
            ctx.runtime.reload_active();
            publish_retained(
                &ctx.client,
                &format!("{}/active_layers/{}", ctx.topics.prefix, id),
                &serde_json::Value::Null,
            )
            .await;
            publish_active_layers(&ctx.client, &ctx.topics, &ctx.store).await;
        }
        Err(e) => eprintln!("[mqtt] active_layers/{id}/delete error: {e}"),
    }
}
