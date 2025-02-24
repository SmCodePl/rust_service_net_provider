use std::sync::Arc;
use async_trait::async_trait;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, TextExpressionMethods};

use crate::{
      domain::interfaces::repository::NetRepository,
      domain::entities::area::Area,
      domain::entities::product::Product,
      domain::entities::product_type::ProductType,
      
      viewmodels::product_result::ProductViewModel,
      viewmodels::product_area_result::ProductAreaViewModel,
      
      infrastructure::db::connection::{DbPool,establish_connection}
};

#[derive(Debug,Clone)]
pub struct DbNetRepository {
   pool: DbPool
}

impl DbNetRepository {
   pub fn new() -> Self{
      let db_url = "postgres://admin:admTest2015@localhost:5432/fiberprovider";
      DbNetRepository{
         pool: establish_connection(&db_url)
      }
   }    
}

#[async_trait]
impl NetRepository for Arc<DbNetRepository>{
   // Function to get all product types
   async fn get_all_product_types(&self) -> Option<Vec<ProductType>>  {
      use crate::schema::product_types::dsl::*;
       product_types
            .load::<ProductType>(& mut self.pool.get().unwrap())
            .optional()
            .expect("Error loading product_types")      
   }

   // Function to get all products parameter active: bool
   async fn get_all_products(&self, active: bool ) -> Option<Vec<Product>>{
      use crate::schema::products::dsl::*;
      // Load all products from the database
      products
         .filter(is_active.eq(active))
         .select((id, name, description, tdc_otc, price, speed, mrc, is_unlimited, is_discounted, is_promo, is_active, product_type_id, currency, created_at, updated_at)) 
         .load::<Product>(& mut self.pool.get().unwrap())
         .optional()
         .expect("Error loading products")
   }
   
   // Function get_products_result returning ProductViewModel
   // function parameters active: bool, page: i64, page_size: i64
   async fn get_products_result(&self ,active: bool,page: i64, page_size: i64 ) -> Option<Vec<ProductViewModel>> {
      use crate::schema::products;
      use crate::schema::product_types;
      
      // Load all products from the database
      products::table
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
         .load::<ProductViewModel>(& mut self.pool.get().unwrap())            
         .optional()
         .expect("Error loading products")
   }


   //Function to get zip_code by zip_code (text)
   async fn get_zip_code(&self, _zip_code: &str) -> Option<Vec<Area>> {    
      use crate::schema::areas::dsl::*;
      // Load all products from the database
       areas
         .filter(zip_code.like(_zip_code))
         .load::<Area>(& mut self.pool.get().unwrap())
         .optional()
         .expect("Error loading areas")     
   }


   // //Function get product_area by zip_code (text) and place_name (text)
   async fn get_product_area(&self, zip_code: &str, place_name: &str) -> Option<Vec<ProductAreaViewModel>> {
      use crate::schema::{product_areas, products, areas,product_types};
      
      // Load all products from the database
      product_areas::table
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
         .load::<ProductAreaViewModel>(& mut self.pool.get().unwrap())
         .optional()
         .expect("Error loading product_areas")      
   }
}

