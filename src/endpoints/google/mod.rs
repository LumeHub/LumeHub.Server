pub mod device;
pub mod endpoint;
pub mod request;
pub mod response;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(endpoint::smarthome);
}
