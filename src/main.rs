use repository::net_repo::*;

extern crate diesel;
extern crate dotenv;

mod schema;
mod models;
mod db;
mod repository;

fn main() {
    println!("Hello, world!");
   
    let result = get_all_product_types();
    println!("Displaying {} posts", result.len());
    
    let presult = get_all_products(false);
    let presult1 = get_all_products(true);
    
    println!("Displaying {} posts", presult.len());
    println!("Displaying {} posts", presult1.len());

    let prod_view_model_result = get_products_result(true, 2, 5);
    println!("Displaying {} posts", prod_view_model_result.len());
   
    for pr in prod_view_model_result {
        println!("product id: {}, name: {}, description: {}, tdc_otc: {}, price: {}, speed: {}, mrc: {}, is_unlimited: {}, is_discounted: {}, is_promo: {}, prod_type_id: {}, 
                prod_type_name: {}, prod_type_desc: {}", 
                pr.prod_id, pr.name, pr.description, pr.tdc_otc, pr.price, pr.speed, pr.mrc, pr.is_unlimited, pr.is_discounted, pr.is_promo, pr.prod_type_id, 
                pr.prod_type_name, pr.prod_type_desc);
    }
}