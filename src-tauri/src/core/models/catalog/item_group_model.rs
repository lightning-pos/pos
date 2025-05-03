use crate::{adapters::outgoing::database::{FromLibsqlValue, FromRow}, core::{db::SeaQueryCrudTrait, types::db_uuid::DbUuid}};
use chrono::NaiveDateTime;
use derive_more::Display;
use juniper::{GraphQLEnum, GraphQLInputObject};
use lightning_macros::{LibsqlEnum, LibsqlFromRow, SeaQueryCrud, SeaQueryEnum, SeaQueryModel};

#[derive(Debug, SeaQueryModel, LibsqlFromRow, SeaQueryCrud)]
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

#[derive(Debug, Clone, Copy, GraphQLEnum, Display, SeaQueryEnum, LibsqlEnum)]
pub enum ItemCategoryState {
    Active,
    Inactive,
    Deleted,
}
