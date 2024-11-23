use crate::core::entities::catalog::item::model::Item;
use serde::{Deserialize, Serialize};
use std::io::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemCategory {
    pub id: String,
    pub name: String,
    pub state: ItemCategoryState,
    pub description: Option<String>,
    pub items: Option<Vec<Item>>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemCategoryState {
    Active,
    Inactive,
}
