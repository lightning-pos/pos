use chrono::NaiveDateTime;
use juniper::{GraphQLInputObject, GraphQLObject};
use sea_query::Iden;

use crate::{adapters::outgoing::database::FromRow, core::types::{db_uuid::DbUuid, money::Money}, error::Result};

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

impl FromRow<libsql::Row> for SalesOrderCharge {
    fn from_row(row: &libsql::Row) -> Result<Self> {
        todo!()
    }
}