use crate::core::common::interface::sql::SQLEntity;
use modql::{
    field::Fields,
    filter::{FilterNodes, OpValsString},
};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, Fields)]
pub struct ItemCategory {
    pub id: String,
    pub name: String,
    pub state: ItemCategoryState,
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Default, Deserialize, FilterNodes)]
pub struct ItemCategoryFilter {
    pub id: Option<OpValsString>,
    pub name: Option<OpValsString>,
    pub state: Option<OpValsString>,
}

impl SQLEntity for ItemCategory {
    const TABLE_NAME: &'static str = "item_categories";
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ItemCategoryState {
    Active,
    Inactive,
    Deleted,
}

impl fmt::Display for ItemCategoryState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ItemCategoryState::Active => write!(f, "Active"),
            ItemCategoryState::Inactive => write!(f, "Inactive"),
            ItemCategoryState::Deleted => write!(f, "Deleted"),
        }
    }
}
