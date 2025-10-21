pub mod legacy;
pub mod google;
pub mod oauth;

use actix_web::web;

pub fn google_config(cfg: &mut web::ServiceConfig) {
    cfg.service(google::endpoint::smarthome)
        .service(oauth::authorize::authorize)
        .service(oauth::token::token);
}

