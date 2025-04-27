use chrono::NaiveDateTime;
use juniper::GraphQLInputObject;
use lightning_macros::{LibsqlFromRow, SeaQueryCrud, SeaQueryModel};

use crate::core::{db::SeaQueryCrudTrait, types::db_uuid::DbUuid};

#[derive(Debug, Clone, SeaQueryModel, SeaQueryCrud, LibsqlFromRow)]
pub struct Supplier {
    pub id: DbUuid,
    pub name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct SupplierNewInput {
    pub name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct SupplierUpdateInput {
    pub id: DbUuid,
    pub name: Option<String>,
    pub address: Option<Option<String>>,
    pub phone: Option<Option<String>>,
}
