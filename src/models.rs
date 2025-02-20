use crate::schema::*;
use diesel::prelude::*;
use serde::{de, Serialize};

#[derive(Debug, Serialize, Queryable,Identifiable,PartialEq)]
pub struct ProductType {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Queryable,Identifiable,PartialEq)]
pub struct Product{ 
    pub id: i32,
    pub name: String,
    pub description: String,
    pub tdc_otc: i32,
    pub price: f64,
    pub speed: i32,
    pub mrc: i32,
    pub is_unlimited: bool,
    pub is_discounted: bool,
    pub is_promo: bool,
    pub is_active: bool,
    pub product_type_id: i32,
    pub currency: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

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

#[derive(Debug, Serialize, Queryable)]
pub struct Area {
    pub id: i32,
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
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
#[derive(Debug, Serialize, Queryable)]
pub struct ProductArea {
    pub id: i32,
    pub product_id : i32,
    pub area_id: i32,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}