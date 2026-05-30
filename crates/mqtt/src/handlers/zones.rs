use crate::ctx::Ctx;
use crate::publish::{publish_retained, publish_zones};
use crate::types::{CreateZoneCommand, UpdateZoneCommand, ZonePayload};

pub async fn handle_create(ctx: &Ctx, payload: &[u8]) {
    let Ok(cmd) = serde_json::from_slice::<CreateZoneCommand>(payload) else {
        eprintln!("[mqtt] zones/create: invalid JSON");
        return;
    };
    match ctx
        .store
        .create_zone(
            &cmd.name,
            cmd.start_pixel,
            cmd.end_pixel,
            cmd.transition_length,
        )
        .await
    {
        Ok(zone) => {
            let topic = ctx.topics.zone(&zone.id);
            publish_retained(&ctx.client, &topic, &ZonePayload::from(zone)).await;
            publish_zones(&ctx.client, &ctx.topics, &ctx.store).await;
        }
        Err(e) => eprintln!("[mqtt] zones/create error: {e}"),
    }
}

pub async fn handle_update(ctx: &Ctx, id: &str, payload: &[u8]) {
    let Ok(cmd) = serde_json::from_slice::<UpdateZoneCommand>(payload) else {
        eprintln!("[mqtt] zones/{id}/update: invalid JSON");
        return;
    };
    match ctx
        .store
        .update_zone(
            id,
            &cmd.name,
            cmd.start_pixel,
            cmd.end_pixel,
            cmd.transition_length,
        )
        .await
    {
        Ok(zone) => {
            let topic = ctx.topics.zone(&zone.id);
            publish_retained(&ctx.client, &topic, &ZonePayload::from(zone)).await;
            publish_zones(&ctx.client, &ctx.topics, &ctx.store).await;
        }
        Err(e) => eprintln!("[mqtt] zones/{id}/update error: {e}"),
    }
}

pub async fn handle_delete(ctx: &Ctx, id: &str) {
    match ctx.store.delete_zone(id).await {
        Ok(()) => {
            // Clear retained message for this zone
            publish_retained(&ctx.client, &ctx.topics.zone(id), &serde_json::Value::Null).await;
            publish_zones(&ctx.client, &ctx.topics, &ctx.store).await;
        }
        Err(e) => eprintln!("[mqtt] zones/{id}/delete error: {e}"),
    }
}
