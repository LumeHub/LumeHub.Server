use std::collections::HashMap;

use domain::BlendMode;
use store::NewLayer;

use crate::ctx::Ctx;
use crate::publish::publish_device_state;
use crate::types::DeviceCommand;

pub async fn handle_set(ctx: &Ctx, payload: &[u8]) {
    let Ok(cmd) = serde_json::from_slice::<DeviceCommand>(payload) else {
        eprintln!("[mqtt] device/set: invalid JSON");
        return;
    };

    if let Some(state) = &cmd.state {
        match state.to_uppercase().as_str() {
            "ON" => ctx.runtime.set_on_off(true),
            "OFF" => ctx.runtime.set_on_off(false),
            other => eprintln!("[mqtt] device/set: unknown state '{other}'"),
        }
    }
    if let Some(b) = cmd.brightness {
        ctx.runtime.set_brightness(b);
    }
    if let Some(c) = cmd.color {
        ctx.runtime.set_color(c.into());
    }
    if let Some(effect_name) = &cmd.effect {
        apply_effect(ctx, effect_name).await;
    }

    let effect = ctx.active_effect.lock().unwrap().clone();
    let snapshot = ctx.runtime.snapshot();
    publish_device_state(&ctx.client, &ctx.topics, &snapshot, effect.as_deref()).await;
}

async fn apply_effect(ctx: &Ctx, name: &str) {
    let Some(builtin) = ctx
        .builtins
        .iter()
        .find(|b| b.name == name || b.slug == name)
    else {
        eprintln!("[mqtt] device/set: unknown effect '{name}'");
        return;
    };
    let layer = NewLayer {
        effect_id: builtin.id(),
        zone_id: "all".to_string(),
        blend_mode: BlendMode::default(),
        params: HashMap::new(),
    };
    match ctx.store.replace_active_layers(&[layer]).await {
        Ok(()) => {
            ctx.runtime.reload_active();
            *ctx.active_effect.lock().unwrap() = Some(name.to_owned());
        }
        Err(e) => eprintln!("[mqtt] device/set effect '{name}': {e}"),
    }
}
