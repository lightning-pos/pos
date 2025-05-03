use chrono::NaiveDateTime;
use juniper::GraphQLInputObject;
use lightning_macros::{LibsqlFromRow, SeaQueryCrud, SeaQueryModel};

use crate::{adapters::outgoing::database::{FromLibsqlValue, FromRow}, core::{db::SeaQueryCrudTrait, types::db_uuid::DbUuid}};

#[derive(Debug, Clone, SeaQueryModel, SeaQueryCrud, LibsqlFromRow)]
pub struct SalesChargeType {
    pub id: DbUuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct SalesChargeTypeNewInput {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct SalesChargeTypeUpdateInput {
    pub id: DbUuid,
    pub name: Option<String>,
    pub description: Option<Option<String>>, // Double optional for nullable field
}
