pub mod commands;
pub mod device;
pub mod effects;
pub mod endpoint;
pub mod error;
pub mod oauth;
pub mod request;
pub mod response;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(endpoint::smarthome);
}
