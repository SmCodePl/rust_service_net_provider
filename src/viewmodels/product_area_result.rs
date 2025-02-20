
use crate::schema::*;
use diesel::prelude::*;
use serde::{de, Serialize};

#[derive(Debug, Serialize, Queryable)]
pub struct ProductAreaViewModel{
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
    pub area_id: i32,
    pub country_code: String, 
    pub zip_code: String,
    pub place_name: String,
    pub admin_name1: String,
    pub admin_code1: String,
    pub admin_name2: String,
    pub admin_code2: String,
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy: f64,  
}

