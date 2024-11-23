use crate::core::entities::catalog::item::model::{Item, ItemRelation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemCategory {
    pub id: String,
    pub name: String,
    pub state: ItemCategoryState,
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,

    // Relations
    pub items: Option<Vec<Item>>,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum ItemCategoryRelation {
    Items(Vec<ItemRelation>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemCategoryState {
    Active,
    Inactive,
}
