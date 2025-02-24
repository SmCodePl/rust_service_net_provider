use log::info;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web::Data};
use crate::webapi::routes;
use crate::infrastructure::repository::net_repo::DbNetRepository;
use crate::infrastructure::web;

pub async fn run() -> std::io::Result<()>{
    let repo = DbNetRepository::new();
    let app_data = Data::new(repo);

    info!("Starting server at http://");
    HttpServer::new(move ||{
        App::new()
            .app_data(app_data.clone())
            .wrap(Logger::default())
            .configure(routes::net_routes::routes)
    })
    .bind("0.0.0.0:4000")
    .unwrap()
    .run()
    .await
}