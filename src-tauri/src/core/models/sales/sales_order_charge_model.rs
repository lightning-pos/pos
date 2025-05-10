use chrono::NaiveDateTime;
use juniper::{GraphQLInputObject, GraphQLObject};
use lightning_macros::{LibsqlFromRow, SeaQueryCrud, SeaQueryModel};

use crate::{adapters::outgoing::database::{FromLibsqlValue, FromRow}, core::{db::SeaQueryCrudTrait, types::{db_uuid::DbUuid, money::Money}}};

#[derive(Debug, Clone, GraphQLObject, SeaQueryModel, SeaQueryCrud, LibsqlFromRow)]
pub struct SalesOrderCharge {
    pub id: DbUuid,
    pub order_id: DbUuid,
    pub charge_type_id: DbUuid,
    pub charge_type_name: String,
    pub amount: Money,
    pub tax_amount: Money,
    pub tax_group_id: Option<DbUuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct SalesOrderChargeNewInput {
    pub charge_type_id: DbUuid,
    pub charge_type_name: String,
    pub amount: Money,
    pub tax_amount: Money,
    pub tax_group_id: Option<DbUuid>,
}
