use chrono::NaiveDateTime;
use juniper::GraphQLInputObject;
use lightning_macros::{LibsqlFromRow, SeaQueryCrud, SeaQueryModel};

use crate::{adapters::outgoing::database::{FromLibsqlValue, FromRow}, core::{db::SeaQueryCrudTrait, types::db_uuid::DbUuid}};

#[derive(Debug, Clone, SeaQueryModel, SeaQueryCrud, LibsqlFromRow)]
pub struct Location {
    pub id: DbUuid,
    pub name: String,
    pub description: Option<String>,
    pub address: Option<String>,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct LocationNewInput {
    pub name: String,
    pub description: Option<String>,
    pub address: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct LocationUpdateInput {
    pub id: DbUuid,
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub address: Option<Option<String>>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct LocationUpdateChangeset {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub address: Option<Option<String>>,
    pub is_active: Option<bool>,
    pub updated_at: NaiveDateTime,
}
