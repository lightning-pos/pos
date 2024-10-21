use std::io::Error;

use crate::core::entities::catalog::item_category::item_category_model::ItemCategory;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub nature: ItemNature,
    pub category_id: String,
    pub category: Option<ItemCategory>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemNature {
    Goods,
    Service,
}

#[cfg_attr(test, mockall::automock)]
pub trait ItemRepository {
    fn insert(&self, item: &Item) -> Result<Item, Error>;
    fn update(&self, item: &Item) -> Result<Item, Error>;
    fn delete(&self, id: &str) -> Result<bool, Error>;
}
