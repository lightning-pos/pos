use chrono::NaiveDateTime;
use juniper::GraphQLInputObject;
use lightning_macros::{LibsqlFromRow, SeaQueryCrud, SeaQueryModel};

use crate::{adapters::outgoing::database::{FromLibsqlValue, FromRow}, core::{db::SeaQueryCrudTrait, types::{db_uuid::DbUuid, money::Money}}};

#[derive(Debug, Clone, SeaQueryModel, SeaQueryCrud, LibsqlFromRow)]
pub struct SalesOrderItem {
    pub id: DbUuid,
    pub order_id: DbUuid,
    pub item_id: Option<DbUuid>,
    pub item_name: String,
    pub quantity: i32,
    pub sku: Option<String>,
    pub price_amount: Money,
    pub disc_amount: Money,
    pub taxable_amount: Money,
    pub tax_amount: Money,
    pub total_amount: Money,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct SalesOrderItemInput {
    pub item_id: Option<DbUuid>,
    pub item_name: String,
    pub quantity: i32,
    pub sku: Option<String>,
    pub price_amount: Money,
    pub disc_amount: Money,
    pub taxable_amount: Money,
    pub tax_amount: Money,
    pub total_amount: Money,
}
