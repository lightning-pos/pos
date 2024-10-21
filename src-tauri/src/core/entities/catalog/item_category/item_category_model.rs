use crate::core::entities::catalog::item::item_model::Item;
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

#[cfg_attr(test, mockall::automock)]
pub trait ItemCategoryRepository {
    fn is_name_taken(&self, name: &str) -> Result<bool, Error>;
    fn has_items(&self, id: &str) -> Result<bool, Error>;
    fn get_one_by_id(&self, id: &str) -> Result<ItemCategory, Error>;
    fn insert(&self, entity: &ItemCategory) -> Result<ItemCategory, Error>;
    fn update(&self, entity: &ItemCategory) -> Result<ItemCategory, Error>;
    fn delete(&self, id: &str) -> Result<bool, Error>;
}
