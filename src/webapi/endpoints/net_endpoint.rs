use actix_web::{get, web, HttpResponse,Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Response {
    status: String,
    message: String,
}


#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(Response {
        status: "ok".to_string(),
        message: "Server is running".to_string(),
    })
}