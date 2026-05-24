use std::collections::HashMap;
use std::sync::Arc;

use actix_web::{HttpResponse, Responder, delete, get, patch, post, put, web};
use application::SceneRuntime;
use domain::{BlendMode, ParamValue};
use serde::Deserialize;
use store::{NewLayer, Store};

use crate::error::ApiError;
use crate::transition::TransitionQuery;
use crate::types::{LayerResponse, SceneResponse};

#[get("/scenes/active")]
pub async fn get_active_scene(store: web::Data<Arc<Store>>) -> Result<impl Responder, ApiError> {
    let layers = store.get_active_layers().await?;
    Ok(HttpResponse::Ok().json(
        layers
            .into_iter()
            .map(LayerResponse::from)
            .collect::<Vec<_>>(),
    ))
}

#[delete("/scenes/active")]
pub async fn clear_active_scene(
    store: web::Data<Arc<Store>>,
    runtime: web::Data<dyn SceneRuntime>,
) -> Result<impl Responder, ApiError> {
    store.clear_active_scene().await?;
    runtime.halt();
    Ok(HttpResponse::NoContent().finish())
}

fn default_opacity() -> f32 {
    1.0
}

#[derive(Deserialize)]
struct AddLayerRequest {
    effect_id: String,
    zone_id: String,
    #[serde(default)]
    blend_mode: BlendMode,
    #[serde(default)]
    params: HashMap<String, ParamValue>,
    #[serde(default = "default_opacity")]
    opacity: f32,
}

impl From<AddLayerRequest> for NewLayer {
    fn from(r: AddLayerRequest) -> Self {
        Self {
            effect_id: r.effect_id,
            zone_id: r.zone_id,
            blend_mode: r.blend_mode,
            params: r.params,
            opacity: r.opacity,
        }
    }
}

#[put("/scenes/active")]
pub async fn set_active_scene(
    store: web::Data<Arc<Store>>,
    runtime: web::Data<dyn SceneRuntime>,
    query: web::Query<TransitionQuery>,
    body: web::Json<Vec<AddLayerRequest>>,
) -> Result<impl Responder, ApiError> {
    let layers: Vec<NewLayer> = body.into_inner().into_iter().map(Into::into).collect();
    store.replace_active_layers(&layers).await?;
    runtime.reload_active_with(query.into_inner().resolve(runtime.as_ref()));
    let layers = store.get_active_layers().await?;
    Ok(HttpResponse::Ok().json(
        layers
            .into_iter()
            .map(LayerResponse::from)
            .collect::<Vec<_>>(),
    ))
}

#[post("/scenes/active/layers")]
pub async fn add_layer(
    store: web::Data<Arc<Store>>,
    runtime: web::Data<dyn SceneRuntime>,
    query: web::Query<TransitionQuery>,
    body: web::Json<AddLayerRequest>,
) -> Result<impl Responder, ApiError> {
    let layer = store
        .add_active_layer(
            &body.effect_id,
            &body.zone_id,
            body.blend_mode,
            &body.params,
        )
        .await?;
    runtime.reload_active_with(query.into_inner().resolve(runtime.as_ref()));
    Ok(HttpResponse::Created().json(LayerResponse::from(layer)))
}

#[derive(Deserialize)]
struct PatchLayerRequest {
    zone_id: Option<String>,
    enabled: Option<bool>,
    blend_mode: Option<BlendMode>,
    params: Option<HashMap<String, ParamValue>>,
}

#[patch("/scenes/active/layers/{id}")]
pub async fn patch_layer(
    store: web::Data<Arc<Store>>,
    runtime: web::Data<dyn SceneRuntime>,
    query: web::Query<TransitionQuery>,
    path: web::Path<String>,
    body: web::Json<PatchLayerRequest>,
) -> Result<impl Responder, ApiError> {
    let id = path.into_inner();
    let current = store.get_active_layer(&id).await?;
    let zone_id = body.zone_id.as_deref().unwrap_or(&current.zone_id);
    let enabled = body.enabled.unwrap_or(current.enabled);
    let blend_mode = body.blend_mode.unwrap_or(current.blend_mode);
    let params = body.params.clone().unwrap_or(current.params);
    let layer = store
        .update_active_layer(&id, zone_id, enabled, blend_mode, &params)
        .await?;
    runtime.reload_active_with(query.into_inner().resolve(runtime.as_ref()));
    Ok(HttpResponse::Ok().json(LayerResponse::from(layer)))
}

#[delete("/scenes/active/layers/{id}")]
pub async fn remove_layer(
    store: web::Data<Arc<Store>>,
    runtime: web::Data<dyn SceneRuntime>,
    query: web::Query<TransitionQuery>,
    path: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    store.remove_active_layer(&path.into_inner()).await?;
    runtime.reload_active_with(query.into_inner().resolve(runtime.as_ref()));
    Ok(HttpResponse::NoContent().finish())
}

#[derive(Deserialize)]
struct ReorderRequest {
    ordered_ids: Vec<String>,
}

#[put("/scenes/active/layers/reorder")]
pub async fn reorder_layers(
    store: web::Data<Arc<Store>>,
    runtime: web::Data<dyn SceneRuntime>,
    query: web::Query<TransitionQuery>,
    body: web::Json<ReorderRequest>,
) -> Result<impl Responder, ApiError> {
    store.reorder_active_layers(&body.ordered_ids).await?;
    runtime.reload_active_with(query.into_inner().resolve(runtime.as_ref()));
    Ok(HttpResponse::NoContent().finish())
}

#[derive(Deserialize)]
struct SaveSceneRequest {
    name: String,
}

#[post("/scenes/active/save")]
pub async fn save_active_scene(
    store: web::Data<Arc<Store>>,
    body: web::Json<SaveSceneRequest>,
) -> Result<impl Responder, ApiError> {
    let scene = store.save_active_as_scene(&body.name).await?;
    Ok(HttpResponse::Created().json(SceneResponse::from(scene)))
}

#[post("/scenes/active/load/{scene_id}")]
pub async fn load_scene_into_active(
    store: web::Data<Arc<Store>>,
    runtime: web::Data<dyn SceneRuntime>,
    query: web::Query<TransitionQuery>,
    path: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    store.load_scene_into_active(&path.into_inner()).await?;
    runtime.reload_active_with(query.into_inner().resolve(runtime.as_ref()));
    Ok(HttpResponse::NoContent().finish())
}

#[put("/scenes/active/overwrite/{scene_id}")]
pub async fn overwrite_scene_from_active(
    store: web::Data<Arc<Store>>,
    path: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    store
        .overwrite_scene_from_active(&path.into_inner())
        .await?;
    Ok(HttpResponse::NoContent().finish())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_active_scene)
        .service(set_active_scene)
        .service(clear_active_scene)
        .service(add_layer)
        .service(reorder_layers)
        .service(patch_layer)
        .service(remove_layer)
        .service(save_active_scene)
        .service(load_scene_into_active)
        .service(overwrite_scene_from_active);
}
