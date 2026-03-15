use std::sync::Arc;

use actix_web::{HttpResponse, Responder, delete, get, post, put, web};
use serde::{Deserialize, Serialize};
use store::{Store, ZoneRecord};

use crate::error::ApiError;

#[derive(Serialize)]
struct ZoneResponse {
    id: String,
    name: String,
    start_pixel: u32,
    end_pixel: u32,
    transition_length: u32,
}

impl From<ZoneRecord> for ZoneResponse {
    fn from(r: ZoneRecord) -> Self {
        Self {
            id: r.id,
            name: r.name,
            start_pixel: r.start_pixel,
            end_pixel: r.end_pixel,
            transition_length: r.transition_length,
        }
    }
}

#[get("/zones")]
pub async fn list_zones(store: web::Data<Arc<Store>>) -> Result<impl Responder, ApiError> {
    let zones = store.get_zones().await?;
    Ok(HttpResponse::Ok().json(
        zones
            .into_iter()
            .map(ZoneResponse::from)
            .collect::<Vec<_>>(),
    ))
}

#[derive(Deserialize)]
struct ZoneBody {
    name: String,
    start_pixel: u32,
    end_pixel: u32,
    #[serde(default)]
    transition_length: u32,
}

#[post("/zones")]
pub async fn create_zone(
    store: web::Data<Arc<Store>>,
    body: web::Json<ZoneBody>,
) -> Result<impl Responder, ApiError> {
    let zone = store
        .create_zone(
            &body.name,
            body.start_pixel,
            body.end_pixel,
            body.transition_length,
        )
        .await?;
    Ok(HttpResponse::Created().json(ZoneResponse::from(zone)))
}

#[get("/zones/{id}")]
pub async fn get_zone(
    store: web::Data<Arc<Store>>,
    path: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let zone = store.get_zone(&path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ZoneResponse::from(zone)))
}

#[put("/zones/{id}")]
pub async fn update_zone(
    store: web::Data<Arc<Store>>,
    path: web::Path<String>,
    body: web::Json<ZoneBody>,
) -> Result<impl Responder, ApiError> {
    let zone = store
        .update_zone(
            &path.into_inner(),
            &body.name,
            body.start_pixel,
            body.end_pixel,
            body.transition_length,
        )
        .await?;
    Ok(HttpResponse::Ok().json(ZoneResponse::from(zone)))
}

#[delete("/zones/{id}")]
pub async fn delete_zone(
    store: web::Data<Arc<Store>>,
    path: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    store.delete_zone(&path.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(list_zones)
        .service(create_zone)
        .service(get_zone)
        .service(update_zone)
        .service(delete_zone);
}
