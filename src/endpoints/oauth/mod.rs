use actix_web::web;

pub mod authorize;
pub mod token;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(authorize::authorize).service(token::token);
}
