use crate::ctx::Ctx;
use crate::publish::{publish_effects, publish_retained};
use crate::types::{CreateEffectCommand, EffectPayload, UpdateEffectCommand};

pub async fn handle_create(ctx: &Ctx, payload: &[u8]) {
    let Ok(cmd) = serde_json::from_slice::<CreateEffectCommand>(payload) else {
        eprintln!("[mqtt] effects/create: invalid JSON");
        return;
    };
    match ctx
        .store
        .create_effect(&cmd.name, &cmd.script, &cmd.params)
        .await
    {
        Ok(effect) => {
            let topic = ctx.topics.effect(&effect.id);
            publish_retained(&ctx.client, &topic, &EffectPayload::from(effect)).await;
            publish_effects(&ctx.client, &ctx.topics, &ctx.store, &ctx.builtins).await;
        }
        Err(e) => eprintln!("[mqtt] effects/create error: {e}"),
    }
}

pub async fn handle_update(ctx: &Ctx, id: &str, payload: &[u8]) {
    if ctx.builtins.iter().any(|b| b.id() == id) {
        eprintln!("[mqtt] effects/{id}/update: cannot modify built-in effect");
        return;
    }
    let Ok(cmd) = serde_json::from_slice::<UpdateEffectCommand>(payload) else {
        eprintln!("[mqtt] effects/{id}/update: invalid JSON");
        return;
    };
    let current = match ctx.store.get_effect(id).await {
        Ok(e) => e,
        Err(e) => {
            eprintln!("[mqtt] effects/{id}/update: not found: {e}");
            return;
        }
    };
    let name = cmd.name.as_deref().unwrap_or(&current.name);
    let script = cmd.script.as_deref().unwrap_or(&current.script);
    let params = cmd.params.as_deref().unwrap_or(&current.params);
    match ctx.store.update_effect(id, name, script, params).await {
        Ok(effect) => {
            let topic = ctx.topics.effect(&effect.id);
            publish_retained(&ctx.client, &topic, &EffectPayload::from(effect)).await;
            publish_effects(&ctx.client, &ctx.topics, &ctx.store, &ctx.builtins).await;
        }
        Err(e) => eprintln!("[mqtt] effects/{id}/update error: {e}"),
    }
}

pub async fn handle_delete(ctx: &Ctx, id: &str) {
    if ctx.builtins.iter().any(|b| b.id() == id) {
        eprintln!("[mqtt] effects/{id}/delete: cannot delete built-in effect");
        return;
    }
    match ctx.store.delete_effect(id).await {
        Ok(()) => {
            publish_retained(
                &ctx.client,
                &ctx.topics.effect(id),
                &serde_json::Value::Null,
            )
            .await;
            publish_effects(&ctx.client, &ctx.topics, &ctx.store, &ctx.builtins).await;
        }
        Err(e) => eprintln!("[mqtt] effects/{id}/delete error: {e}"),
    }
}
