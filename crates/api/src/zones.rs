use std::sync::Arc;

use actix_web::{HttpResponse, Responder, delete, get, post, put, web};
use serde::{Deserialize, Serialize};
use store::{Store, ZoneRecord};

use crate::error::ApiError;

const ALL_ZONE_ID: &str = "all";

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

fn all_zone(strip_len: usize) -> ZoneResponse {
    ZoneResponse {
        id: ALL_ZONE_ID.to_string(),
        name: "Full Strip".to_string(),
        start_pixel: 0,
        end_pixel: strip_len.saturating_sub(1) as u32,
        transition_length: 0,
    }
}

#[get("/zones")]
pub async fn list_zones(
    store: web::Data<Arc<Store>>,
    strip_len: web::Data<usize>,
) -> Result<impl Responder, ApiError> {
    let mut zones = vec![all_zone(**strip_len)];
    zones.extend(store.get_zones().await?.into_iter().map(ZoneResponse::from));
    Ok(HttpResponse::Ok().json(zones))
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
    strip_len: web::Data<usize>,
    path: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let id = path.into_inner();
    if id == ALL_ZONE_ID {
        return Ok(HttpResponse::Ok().json(all_zone(**strip_len)));
    }
    let zone = store.get_zone(&id).await?;
    Ok(HttpResponse::Ok().json(ZoneResponse::from(zone)))
}

#[put("/zones/{id}")]
pub async fn update_zone(
    store: web::Data<Arc<Store>>,
    path: web::Path<String>,
    body: web::Json<ZoneBody>,
) -> Result<impl Responder, ApiError> {
    let id = path.into_inner();
    if id == ALL_ZONE_ID {
        return Err(ApiError::Forbidden("cannot modify the full-strip zone"));
    }
    let zone = store
        .update_zone(
            &id,
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
    let id = path.into_inner();
    if id == ALL_ZONE_ID {
        return Err(ApiError::Forbidden("cannot delete the full-strip zone"));
    }
    store.delete_zone(&id).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(list_zones)
        .service(create_zone)
        .service(get_zone)
        .service(update_zone)
        .service(delete_zone);
}
