use chrono::NaiveDateTime;
use juniper::{GraphQLInputObject, GraphQLObject};
use sea_query::Iden;

use crate::core::types::{db_uuid::DbUuid, money::Money};

#[derive(Debug, GraphQLObject)]
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

#[derive(Iden)]
pub enum SalesOrderCharges {
    Table,
    Id,
    OrderId,
    ChargeTypeId,
    ChargeTypeName,
    Amount,
    TaxAmount,
    TaxGroupId,
    CreatedAt,
    UpdatedAt,
}