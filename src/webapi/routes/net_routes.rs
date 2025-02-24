use actix_web::web;
use crate::webapi::endpoints::net_endpoint::health;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/net")
        .service(health)
        );
}