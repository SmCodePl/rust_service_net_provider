use crate::schema::*;
use diesel::prelude::*;
use serde::{de, Serialize};

#[derive(Debug, Serialize, Queryable)]
#[diesel(belongs_to(product))]
#[diesel(belongs_to(area))]
pub struct ProductArea {
    pub id: i32,
    pub product_id : i32,
    pub area_id: i32,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

