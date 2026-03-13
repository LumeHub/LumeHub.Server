pub mod authorize;
pub mod token;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(authorize::authorize).service(token::token);
}
