use crate::core::common::interface::sql::SQLEntity;

use derive_more::derive::Display;
use modql::{
    field::Fields,
    filter::{FilterNodes, OpValsString},
};
use serde::Deserialize;
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Fields, FromRow)]
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

#[derive(Debug, Clone, Display, sqlx::Type)]
pub enum ItemNature {
    Goods,
    Service,
}

#[derive(Debug, Clone, Display, sqlx::Type)]
pub enum ItemState {
    Active,
    Inactive,
    Deleted,
}

impl SQLEntity for Item {
    const TABLE_NAME: &'static str = "items";

    fn id(&self) -> String {
        self.id.clone()
    }
}

impl From<ItemNature> for sea_query::Value {
    fn from(value: ItemNature) -> Self {
        value.to_string().into()
    }
}

impl From<ItemState> for sea_query::Value {
    fn from(value: ItemState) -> Self {
        value.to_string().into()
    }
}
