mod color;
mod state;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(color::led_get_color)
        .service(color::led_set_color)
        .service(state::led_state_get)
        .service(state::led_state_on)
        .service(state::led_state_off);
}
