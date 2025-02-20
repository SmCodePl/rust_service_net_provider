
use crate::schema::*;
use diesel::prelude::*;
use serde::{de, Serialize};

#[derive(Debug, Serialize, Queryable)]
pub struct ProductViewModel{
    pub prod_id: i32,
    pub name: String,
    pub description: String,
    pub tdc_otc: i32,
    pub price: f64,
    pub speed: i32,
    pub mrc: i32,
    pub is_unlimited: bool,
    pub is_discounted: bool,
    pub is_promo: bool,
    pub prod_type_id: i32,
    pub prod_type_name: String,
    pub prod_type_desc: String,
}
