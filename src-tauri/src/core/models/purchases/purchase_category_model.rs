use chrono::NaiveDateTime;
use derive_more::Display;
use juniper::{GraphQLEnum, GraphQLInputObject};
use lightning_macros::{LibsqlEnum, LibsqlFromRow, SeaQueryCrud, SeaQueryEnum, SeaQueryModel};

use crate::{adapters::outgoing::database::{FromLibsqlValue, FromRow}, core::{db::SeaQueryCrudTrait, types::db_uuid::DbUuid}};

#[derive(Debug, Clone, SeaQueryModel, SeaQueryCrud, LibsqlFromRow)]
pub struct PurchaseCategory {
    pub id: DbUuid,
    pub name: String,
    pub description: Option<String>,
    pub state: PurchaseCategoryState,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct PurchaseCategoryNew {
    pub name: String,
    pub description: Option<String>,
    pub state: Option<PurchaseCategoryState>,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct PurchaseCategoryUpdate {
    pub id: DbUuid,
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub state: Option<PurchaseCategoryState>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Copy, GraphQLEnum, PartialEq, Eq, Display, SeaQueryEnum, LibsqlEnum)]
pub enum PurchaseCategoryState {
    Active,
    Inactive,
    Deleted,
}
