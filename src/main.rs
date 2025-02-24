use crate::infrastructure::repository::net_repo::DbNetRepository;
use actix_web::{ web, App, HttpResponse, HttpServer,  Result};


use serde::Serialize;
mod schema;
mod infrastructure;
mod viewmodels;
mod domain;
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
            .default_service(web::route().to(not_found_error))
            .wrap(actix_web::middleware::Logger::default())
            
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await

}

