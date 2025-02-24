use actix_web::web;
use crate::{domain::service, webapi::endpoints::net_endpoint::{get_all_product_types, get_all_products, get_offers, health,get_zip_codes}};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/net")
        .service(health)
        .service(get_all_product_types)
        .service(get_all_products)
        .service(get_offers)
        .service(get_zip_codes)
        );
}