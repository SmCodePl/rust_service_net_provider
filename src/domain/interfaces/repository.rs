use async_trait::async_trait;
use crate::domain::entities::product::Product;
use crate::domain::entities::area::Area;
use crate::domain::entities::product_type::ProductType;
use crate::viewmodels::product_result::ProductViewModel;
use crate::viewmodels::product_area_result::ProductAreaViewModel;

#[async_trait]
pub trait NetRepository{
    async fn get_all_product_types(&self) -> Option<Vec<ProductType>>;
    async fn get_all_products(&self, active: bool) -> Option<Vec<Product>>;
    async fn get_zip_code(&self, zip_code: &str) -> Option<Vec<Area>>;
    async fn get_products_result(&self ,active: bool,page: i64, page_size: i64 ) -> Option<Vec<ProductViewModel>> ;
    async fn get_product_area(&self, zip_code: &str, place_name: &str)-> Option<Vec<ProductAreaViewModel>>;
   
}