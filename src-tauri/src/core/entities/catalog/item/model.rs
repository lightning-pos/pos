use crate::core::entities::catalog::item_category::model::{ItemCategory, ItemCategoryRelation};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub nature: ItemNature,
    pub category_id: String,
    pub created_at: i64,
    pub updated_at: i64,

    // Relations
    pub category: Option<ItemCategory>,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum ItemRelation {
    Category(ItemCategoryRelation),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemNature {
    Goods,
    Service,
}
