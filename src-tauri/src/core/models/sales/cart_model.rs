use chrono::NaiveDateTime;
use juniper::GraphQLInputObject;
use lightning_macros::{LibsqlFromRow, SeaQueryCrud, SeaQueryModel};

use crate::{adapters::outgoing::database::{FromLibsqlValue, FromRow}, core::{db::SeaQueryCrudTrait, types::db_uuid::DbUuid}};

#[derive(Debug, Clone, SeaQueryModel, SeaQueryCrud, LibsqlFromRow)]
pub struct Cart {
    pub id: DbUuid,
    pub cart_data: String,
    pub customer_id: Option<DbUuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct CartNewInput {
    pub customer_id: Option<DbUuid>,
    pub cart_data: String,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct CartUpdateInput {
    pub id: DbUuid,
    pub cart_data: Option<String>,
}
