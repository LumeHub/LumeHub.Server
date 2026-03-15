use std::sync::Arc;

use actix_web::{HttpResponse, Responder, delete, get, post, put, web};
use serde::{Deserialize, Serialize};
use store::Store;

use crate::error::ApiError;
use crate::types::{LayerResponse, SceneResponse};

#[derive(Serialize)]
struct SceneDetailResponse {
    id: String,
    name: String,
    layers: Vec<LayerResponse>,
}

#[get("/scenes")]
pub async fn list_scenes(store: web::Data<Arc<Store>>) -> Result<impl Responder, ApiError> {
    let scenes = store.get_scenes().await?;
    Ok(HttpResponse::Ok().json(
        scenes
            .into_iter()
            .map(SceneResponse::from)
            .collect::<Vec<_>>(),
    ))
}

#[derive(Deserialize)]
struct SceneBody {
    name: String,
}

#[post("/scenes")]
pub async fn create_scene(
    store: web::Data<Arc<Store>>,
    body: web::Json<SceneBody>,
) -> Result<impl Responder, ApiError> {
    let scene = store.create_scene(&body.name).await?;
    Ok(HttpResponse::Created().json(SceneResponse::from(scene)))
}

#[get("/scenes/{id}")]
pub async fn get_scene(
    store: web::Data<Arc<Store>>,
    path: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let id = path.into_inner();
    let scene = store.get_scene(&id).await?;
    let layers = store.get_layers(&id).await?;
    Ok(HttpResponse::Ok().json(SceneDetailResponse {
        id: scene.id,
        name: scene.name,
        layers: layers.into_iter().map(LayerResponse::from).collect(),
    }))
}

#[put("/scenes/{id}")]
pub async fn update_scene(
    store: web::Data<Arc<Store>>,
    path: web::Path<String>,
    body: web::Json<SceneBody>,
) -> Result<impl Responder, ApiError> {
    let scene = store.update_scene(&path.into_inner(), &body.name).await?;
    Ok(HttpResponse::Ok().json(SceneResponse::from(scene)))
}

#[delete("/scenes/{id}")]
pub async fn delete_scene(
    store: web::Data<Arc<Store>>,
    path: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    store.delete_scene(&path.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(list_scenes)
        .service(create_scene)
        .service(get_scene)
        .service(update_scene)
        .service(delete_scene);
}
