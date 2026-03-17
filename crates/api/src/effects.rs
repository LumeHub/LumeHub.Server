use std::sync::Arc;

use actix_web::{HttpResponse, Responder, delete, get, post, put, web};
use domain::ParamDef;
use effects::BuiltinEffect;
use serde::{Deserialize, Serialize};
use store::{EffectRecord, Store};

use crate::error::ApiError;

#[derive(Serialize)]
struct EffectResponse {
    id: String,
    name: String,
    script: String,
    params: Vec<ParamDef>,
    builtin: bool,
}

impl From<EffectRecord> for EffectResponse {
    fn from(r: EffectRecord) -> Self {
        Self {
            id: r.id,
            name: r.name,
            script: r.script,
            params: r.params,
            builtin: false,
        }
    }
}

impl From<&BuiltinEffect> for EffectResponse {
    fn from(b: &BuiltinEffect) -> Self {
        Self {
            id: b.id(),
            name: b.name.clone(),
            script: b.script.clone(),
            params: b.params.clone(),
            builtin: true,
        }
    }
}

#[get("/effects")]
pub async fn list_effects(
    store: web::Data<Arc<Store>>,
    builtins: web::Data<Vec<BuiltinEffect>>,
) -> Result<impl Responder, ApiError> {
    let user_effects = store.get_effects().await?;
    let mut effects: Vec<EffectResponse> = builtins.iter().map(EffectResponse::from).collect();
    effects.extend(user_effects.into_iter().map(EffectResponse::from));
    Ok(HttpResponse::Ok().json(effects))
}

#[derive(Deserialize)]
struct EffectBody {
    name: String,
    script: String,
    #[serde(default)]
    params: Vec<ParamDef>,
}

#[post("/effects")]
pub async fn create_effect(
    store: web::Data<Arc<Store>>,
    body: web::Json<EffectBody>,
) -> Result<impl Responder, ApiError> {
    let record = store
        .create_effect(&body.name, &body.script, &body.params)
        .await?;
    Ok(HttpResponse::Created().json(EffectResponse::from(record)))
}

#[get("/effects/{id}")]
pub async fn get_effect(
    store: web::Data<Arc<Store>>,
    builtins: web::Data<Vec<BuiltinEffect>>,
    path: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let id = path.into_inner();
    if let Some(b) = find_builtin(&builtins, &id) {
        return Ok(HttpResponse::Ok().json(EffectResponse::from(b)));
    }
    let record = store.get_effect(&id).await?;
    Ok(HttpResponse::Ok().json(EffectResponse::from(record)))
}

#[put("/effects/{id}")]
pub async fn update_effect(
    store: web::Data<Arc<Store>>,
    builtins: web::Data<Vec<BuiltinEffect>>,
    path: web::Path<String>,
    body: web::Json<EffectBody>,
) -> Result<impl Responder, ApiError> {
    let id = path.into_inner();
    if find_builtin(&builtins, &id).is_some() {
        return Err(ApiError::Forbidden("cannot modify a built-in effect"));
    }
    let record = store
        .update_effect(&id, &body.name, &body.script, &body.params)
        .await?;
    Ok(HttpResponse::Ok().json(EffectResponse::from(record)))
}

#[delete("/effects/{id}")]
pub async fn delete_effect(
    store: web::Data<Arc<Store>>,
    builtins: web::Data<Vec<BuiltinEffect>>,
    path: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let id = path.into_inner();
    if find_builtin(&builtins, &id).is_some() {
        return Err(ApiError::Forbidden("cannot delete a built-in effect"));
    }
    store.delete_effect(&id).await?;
    Ok(HttpResponse::NoContent().finish())
}

fn find_builtin<'a>(builtins: &'a [BuiltinEffect], id: &str) -> Option<&'a BuiltinEffect> {
    builtins.iter().find(|b| b.id() == id)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(list_effects)
        .service(create_effect)
        .service(get_effect)
        .service(update_effect)
        .service(delete_effect);
}
