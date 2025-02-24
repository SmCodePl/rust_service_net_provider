use actix_web::rt::net;

use crate::domain::interfaces::repository::NetRepository;
use crate::domain::service::net_service::NetService;

pub struct GetAllProductTypeEvent<T: NetRepository> {
     net_repository: NetService<T>,
}

impl<T: NetRepository> GetAllProductTypeEvent<T> {
    pub fn new(net_repository: NetService<T>) -> Self {
        GetAllProductTypeEvent {
            net_repository
        }
    }

    pub async fn execute(&self) -> Option<Vec<crate::domain::entities::product_type::ProductType>> {
        self.net_repository.get_all_product_types().await
    }
}