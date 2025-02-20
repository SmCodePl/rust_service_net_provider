
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

