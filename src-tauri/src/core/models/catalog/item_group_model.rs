use crate::{adapters::outgoing::database::FromRow, core::types::db_uuid::DbUuid, error::Result};
use chrono::NaiveDateTime;
use derive_more::Display;
use juniper::{GraphQLEnum, GraphQLInputObject};
use sea_query::Iden;

#[derive(Debug)]
pub struct ItemCategory {
    pub id: DbUuid,
    pub name: String,
    pub description: Option<String>,
    pub state: ItemCategoryState,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ItemCategoryNew {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ItemCategoryUpdate {
    pub id: DbUuid,
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub state: Option<ItemCategoryState>,
}

#[derive(Debug, Clone, Copy, GraphQLEnum, Display)]
pub enum ItemCategoryState {
    Active,
    Inactive,
    Deleted,
}

// Define table and column identifiers for SeaQuery
#[derive(Iden)]
pub enum ItemCategories {
    Table,
    Id,
    Name,
    Description,
    State,
    CreatedAt,
    UpdatedAt,
}

impl FromRow<libsql::Row> for ItemCategory {
    fn from_row(row: &libsql::Row) -> Result<Self> {
        todo!()
    }
}
