

use crate::entities::area::Area;
use crate::entities::product::Product;
use crate::entities::product_type::ProductType;
use crate::viewmodels::product_result::ProductViewModel;
use crate::viewmodels::product_area_result::ProductAreaViewModel;
use crate::db::establish_connection; 

use diesel::prelude::*;

// Function to get all product types
 pub fn get_all_product_types() -> Vec<ProductType>  {
    use crate::schema::product_types::dsl::*;

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
pub fn get_products_result(active: bool,page: i64, page_size: i64 ) -> Vec<ProductViewModel> {
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
      .limit(page_size )
      .offset((page - 1) * page_size )
      .load::<ProductViewModel>(connection)            
      .expect("Error loading products");

   return results;
}

// Function to get zip_code by zip_code (text)
pub fn get_zip_code(zip_code: &str) -> Vec<Area> {    
   use crate::schema::areas::dsl::*;
   println!("get_zip_code");

   // Establish a connection to the database
   let connection = &mut establish_connection();

   // Load all products from the database
   let results = areas
      .filter(zip_code.like(zip_code))
      .load::<Area>(connection)
      .expect("Error loading areas");

   return results;
}

// //Function get product_area by zip_code (text) and place_name (text)
pub fn get_product_area(zip_code: &str, place_name: &str) -> Vec<ProductAreaViewModel> {
   use crate::schema::{product_areas, products, areas,product_types};
     
   println!("get_product_area");

   // Establish a connection to the database
   let connection = &mut establish_connection();

   // Load all products from the database
   let results = product_areas::table
      .inner_join(products::table).filter(products::is_active.eq(true))
      .inner_join(areas::table)
      .filter(areas::zip_code.eq(zip_code))
      .filter(areas::place_name.eq(place_name))
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
               areas::id,
               areas::country_code,
               areas::zip_code,
               areas::place_name,
               areas::admin_name1,
               areas::admin_code1,
               areas::admin_name2,
               areas::admin_code2,
               areas::latitude,
               areas::longitude,
               areas::accuracy)
            )            
      .load::<ProductAreaViewModel>(connection)
      .expect("Error loading product_areas");

   return  results;
}
