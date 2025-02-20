use crate::schema::*;
use diesel::prelude::*;
use serde::{de, Serialize};

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