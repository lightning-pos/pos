use crate::core::common::interface::sql::SQLEntity;

use modql::{
    field::Fields,
    filter::{FilterNodes, OpValsString},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Fields)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub nature: ItemNature,
    pub category_id: String,
    pub state: ItemState,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Default, Deserialize, FilterNodes)]
pub struct ItemFilter {
    pub id: Option<OpValsString>,
    pub name: Option<OpValsString>,
    pub nature: Option<OpValsString>,
    pub category_id: Option<OpValsString>,
    pub state: Option<OpValsString>,
}

impl SQLEntity for Item {
    const TABLE_NAME: &'static str = "items";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemNature {
    Goods,
    Service,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ItemState {
    Active,
    Inactive,
    Deleted,
}

impl std::fmt::Display for ItemState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemState::Active => write!(f, "Active"),
            ItemState::Inactive => write!(f, "Inactive"),
            ItemState::Deleted => write!(f, "Deleted"),
        }
    }
}
