use actix_web::{get, web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

use crate::{
    domain::interfaces::repository::NetRepository, 
    infrastructure::repository::net_repo::DbNetRepository    
};

#[derive(Debug,Deserialize)]
pub struct OffersRequest
{
     pub zip_code: String,
     pub place_name: String   
}
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

#[get("/products")]
async fn get_all_products( repo: web::Data<DbNetRepository>) -> HttpResponse {
    match repo.get_all_products(true).await {
        Some(result) => {
            HttpResponse::Ok().json(result)
        },
        None => {
            HttpResponse::Ok().json(Response {
                status: "error".to_string(),
                message: "No products found".to_string(),
            })
        }       
    }
}
#[get("/{zip_code}")]
async fn get_zip_codes(repo: web::Data<DbNetRepository>,zip_code: web::Path<String>) -> HttpResponse {
    match repo.get_zip_code(&zip_code).await {
        Some(result) => {
            HttpResponse::Ok().json(result)
        },
        None => {
            HttpResponse::Ok().json(Response {
                status: "error".to_string(),
                message: "No zip codes found".to_string(),
            })
        }       
    }
}
#[get("/types")]
async fn get_all_product_types( repo: web::Data<DbNetRepository>) -> HttpResponse {
    
    match repo.get_all_product_types().await {
        Some(result) => {
            HttpResponse::Ok().json(result)
        },
        None => {
            HttpResponse::Ok().json(Response {
                status: "error".to_string(),
                message: "No product types found".to_string(),
            })
        }       
    } 
}
   
#[get("offers")]
async  fn get_offers(repo: web::Data<DbNetRepository>,req: web::Json<OffersRequest>) -> HttpResponse {
             
    match repo.get_product_area(&req.zip_code,&req.place_name).await {
        Some(result) => {
            HttpResponse::Ok().json(result)
        },
        None => {
            HttpResponse::Ok().json(Response {
                status: "error".to_string(),
                message: "No offers found".to_string(),
            })
        }       
    } 
}
    
