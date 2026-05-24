use std::sync::Arc;

use actix_web::{HttpResponse, Responder, delete, get, post, web};
use application::SceneRuntime;
use serde::Serialize;
use store::{StackDevice, StackEntry, StackLayer, Store};

use crate::error::ApiError;
use crate::transition::TransitionQuery;

#[derive(Serialize)]
struct StackSummary {
    position: i64,
    pushed_at: i64,
    layer_count: usize,
}

impl From<&StackEntry> for StackSummary {
    fn from(e: &StackEntry) -> Self {
        Self {
            position: e.position,
            pushed_at: e.pushed_at,
            layer_count: e.layers.len(),
        }
    }
}

#[post("/scenes/active/push")]
pub async fn push_state(
    store: web::Data<Arc<Store>>,
    runtime: web::Data<dyn SceneRuntime>,
) -> Result<impl Responder, ApiError> {
    let layers: Vec<StackLayer> = store
        .get_active_layers()
        .await?
        .into_iter()
        .map(|l| StackLayer {
            effect_id: l.effect_id,
            zone_id: l.zone_id,
            blend_mode: l.blend_mode,
            enabled: l.enabled,
            params: l.params,
        })
        .collect();

    let snapshot = runtime.snapshot();
    let device = StackDevice {
        on: snapshot.on,
        brightness: snapshot.brightness,
        color: snapshot.color,
    };
    store.push_state(&layers, &device).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[post("/scenes/active/pop")]
pub async fn pop_state(
    store: web::Data<Arc<Store>>,
    runtime: web::Data<dyn SceneRuntime>,
    query: web::Query<TransitionQuery>,
) -> Result<impl Responder, ApiError> {
    let entry = store.pop_state().await?.ok_or(ApiError::NotFound)?;
    store.restore_active_from_stack(&entry.layers).await?;
    let spec = query.into_inner().resolve(runtime.as_ref());
    runtime.set_on_off_with(entry.device.on, spec);
    runtime.set_brightness_with(entry.device.brightness, spec);
    runtime.set_color_with(entry.device.color, spec);
    runtime.reload_active_with(spec);
    Ok(HttpResponse::NoContent().finish())
}

#[get("/scenes/active/stack")]
pub async fn get_stack(store: web::Data<Arc<Store>>) -> Result<impl Responder, ApiError> {
    let entries = store.list_state_stack().await?;
    let summaries: Vec<StackSummary> = entries.iter().map(StackSummary::from).collect();
    Ok(HttpResponse::Ok().json(summaries))
}

#[delete("/scenes/active/stack")]
pub async fn clear_stack(store: web::Data<Arc<Store>>) -> Result<impl Responder, ApiError> {
    store.clear_state_stack().await?;
    Ok(HttpResponse::NoContent().finish())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(push_state)
        .service(pop_state)
        .service(get_stack)
        .service(clear_stack);
}
