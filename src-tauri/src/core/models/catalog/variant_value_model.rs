use chrono::NaiveDateTime;
use juniper::GraphQLInputObject;
use lightning_macros::{LibsqlFromRow, SeaQueryCrud, SeaQueryModel};

use crate::{adapters::outgoing::database::{FromLibsqlValue, FromRow}, core::{db::SeaQueryCrudTrait, types::db_uuid::DbUuid}};

#[derive(Debug, Clone, SeaQueryModel, SeaQueryCrud, LibsqlFromRow)]
pub struct VariantValue {
    pub id: DbUuid,
    pub variant_type_id: DbUuid,
    pub value: String,
    pub display_order: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct VariantValueNewInput {
    pub variant_type_id: DbUuid,
    pub value: String,
    pub display_order: Option<i32>,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct VariantValueUpdateInput {
    pub id: DbUuid,
    pub value: Option<String>,
    pub display_order: Option<i32>,
    pub updated_at: Option<NaiveDateTime>,
}
