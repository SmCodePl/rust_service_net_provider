use crate::schema::*;
use diesel::prelude::*;
use serde::{de, Serialize};
#[derive(Debug, Serialize, Queryable,Identifiable,PartialEq)]
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