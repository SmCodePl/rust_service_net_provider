use env_logger;
use crate::infrastructure::repository::net_repo::DbNetRepository;
use crate::infrastructure::web::run;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use env_logger::Env;

use serde::Serialize;
mod schema;
mod infrastructure;
mod viewmodels;
mod domain;
mod application;
mod webapi;

#[derive(Serialize)]
pub struct Response {
    status: String,
    message: String,
}
async fn not_found_error() -> Result<HttpResponse> {
    Ok(HttpResponse::NotFound().json(Response {
        status: "error".to_string(),
        message: "Not Found".to_string(),
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let events_db = DbNetRepository::new();
    let app_data = actix_web::web::Data::new(events_db);
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(webapi::routes::net_routes::routes)
            .service(webapi::endpoints::net_endpoint::health)
            .default_service(web::route().to(not_found_error))
            .wrap(actix_web::middleware::Logger::default())
            
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await



    
    // let msg = msg_result::MsgResult::new("Hello".to_string(), true);

    //  println!("msg: {}, success: {}", msg.message, msg.success); 
    // let result = get_all_product_types();
    // println!("Displaying {} posts", result.len());
    
    // let presult = get_all_products(false);
    // let presult1 = get_all_products(true);
    
    // println!("Displaying {} posts", presult.len());
    // println!("Displaying {} posts", presult1.len());

    
    // let prod_view_model_result = get_products_result(true, 2, 5);
    // println!("Displaying {} posts", prod_view_model_result.len());
   
    // for pr in prod_view_model_result {
    //     println!("product id: {}, name: {}, description: {}, tdc_otc: {}, price: {}, speed: {}, mrc: {}, is_unlimited: {}, is_discounted: {}, is_promo: {}, prod_type_id: {}, 
    //             prod_type_name: {}, prod_type_desc: {}", 
    //             pr.prod_id, pr.name, pr.description, pr.tdc_otc, pr.price, pr.speed, pr.mrc, pr.is_unlimited, pr.is_discounted, pr.is_promo, pr.prod_type_id, 
    //             pr.prod_type_name, pr.prod_type_desc);
    // }
}

