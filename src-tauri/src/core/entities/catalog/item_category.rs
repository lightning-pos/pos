use crate::core::common::interface::sql::SQLEntity;
use derive_more::derive::Display;
use modql::{
    field::Fields,
    filter::{FilterNodes, OpValsString},
};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, Fields, FromRow, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, Clone, Display, PartialEq, sqlx::Type)]
pub enum ItemCategoryState {
    Active,
    Inactive,
    Deleted,
}

impl SQLEntity for ItemCategory {
    const TABLE_NAME: &'static str = "item_categories";

    fn id(&self) -> String {
        self.id.clone()
    }
}

impl From<ItemCategoryState> for sea_query::Value {
    fn from(state: ItemCategoryState) -> Self {
        sea_query::Value::from(state.to_string())
    }
}
