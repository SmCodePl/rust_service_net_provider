 
use crate::models::{Product, ProductType, ProductViewModel};

 use crate::db::establish_connection;
 
use diesel::dsl::select;
use diesel::prelude::*;

// Function to get all product types
 pub fn get_all_product_types() -> Vec<ProductType>  {
    use crate::schema::product_types::dsl::*;
    println!("get_all_product_types" );

    let  connection = &mut establish_connection();

    let results = product_types
         .load::<ProductType>(connection)
         .expect("Error loading product_types");

    return results;
 }

// Function to get all products parameter active: bool
pub fn get_all_products(active: bool ) -> Vec<Product> {
   use crate::schema::products::dsl::*;
   println!("get_all_products");

   // Establish a connection to the database
   let connection = &mut establish_connection();

   // Load all products from the database
   let results = products
      .filter(is_active.eq(active))
      .select((id, name, description, tdc_otc, price, speed, mrc, is_unlimited, is_discounted, is_promo, is_active, product_type_id, currency, created_at, updated_at)) 
      .load::<Product>(connection)
      .expect("Error loading products");

   return results;
}
// Function get_products_result returning ProductViewModel
// function parameters active: bool, page: u8, page_size: u8
pub fn get_products_result(active: bool,page: u8, page_size: u8 ) -> Vec<ProductViewModel> {
   use crate::schema::products;
   use crate::schema::product_types;
   println!("get_products_result");

   // Establish a connection to the database
   let connection = &mut establish_connection();

   // Load all products from the database
   let results = products::table
      .inner_join(product_types::table)
      .filter(products::is_active.eq(active))
      .select((products::id, 
               products::name, 
               products::description, 
               products::tdc_otc, 
               products::price, 
               products::speed, 
               products::mrc, 
               products::is_unlimited, 
               products::is_discounted, 
               products::is_promo,  
               products::product_type_id, 
               product_types::name, 
               product_types::description)
            )
      .limit(page_size as i64)
      .offset((page as i64 - 1) * page_size as i64)
      .load::<ProductViewModel>(connection)            
      .expect("Error loading products");

   return results;
}