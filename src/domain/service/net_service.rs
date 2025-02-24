use crate::domain::interfaces::repository::NetRepository;

#[derive(Clone)]
pub struct NetService<T: NetRepository>{
    net_repo: T,
}
     
impl<T:NetRepository> NetService<T> {
    pub fn new(net_repo: T) -> Self {
        NetService {
            net_repo
        }
    }

    pub async fn get_all_product_types(&self) -> Option<Vec<crate::domain::entities::product_type::ProductType>> {
        self.net_repo.get_all_product_types().await
    }
    
    pub async fn get_all_products(&self, active: bool) -> Option<Vec<crate::domain::entities::product::Product>> {
        self.net_repo.get_all_products(active).await
    }
    
    pub async fn get_zip_code(&self, zip_code: &str) -> Option<Vec<crate::domain::entities::area::Area>> {
        self.net_repo.get_zip_code(zip_code).await
    }
    
    pub async fn get_products_result(&self ,active: bool,page: i64, page_size: i64 ) -> Option<Vec<crate::viewmodels::product_result::ProductViewModel>> {
        self.net_repo.get_products_result(active, page, page_size).await
    }
    
    pub async fn get_product_area(&self, zip_code: &str, place_name: &str) -> Option<Vec<crate::viewmodels::product_area_result::ProductAreaViewModel>> {
        self.net_repo.get_product_area(zip_code, place_name).await
    }
}