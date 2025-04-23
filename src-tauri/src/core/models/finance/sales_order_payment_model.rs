use chrono::NaiveDateTime;
use derive_more::Display;
use juniper::{GraphQLEnum, GraphQLInputObject};
use sea_query::Iden;

use crate::{adapters::outgoing::database::FromRow, core::types::{db_uuid::DbUuid, money::Money}, error::Result};

#[derive(Debug, Clone)]
pub struct SalesOrderPayment {
    pub id: DbUuid,
    pub order_id: DbUuid,
    pub payment_method_id: DbUuid,
    pub payment_date: NaiveDateTime,
    pub amount: Money,
    pub reference_number: Option<String>,
    pub notes: Option<String>,
    pub state: SalesOrderPaymentState,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct SalesOrderPaymentNewInput {
    pub order_id: DbUuid,
    pub payment_method_id: DbUuid,
    pub payment_date: NaiveDateTime,
    pub amount: Money,
    pub reference_number: Option<String>,
    pub notes: Option<String>,
    pub state: Option<SalesOrderPaymentState>,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct SalesOrderPaymentUpdateInput {
    pub id: DbUuid,
    pub payment_method_id: Option<DbUuid>,
    pub payment_date: Option<NaiveDateTime>,
    pub amount: Option<Money>,
    pub reference_number: Option<Option<String>>,
    pub notes: Option<Option<String>>,
    pub state: Option<SalesOrderPaymentState>,
}

#[derive(Debug, Clone, Copy, GraphQLEnum, PartialEq, Eq, Display)]
pub enum SalesOrderPaymentState {
    Completed,
    Failed,
    Voided,
}

// Define table and column identifiers for SeaQuery
#[derive(Iden)]
pub enum SalesOrderPayments {
    Table,
    Id,
    OrderId,
    PaymentMethodId,
    PaymentDate,
    Amount,
    ReferenceNumber,
    Notes,
    State,
    CreatedAt,
    UpdatedAt,
}

impl FromRow<libsql::Row> for SalesOrderPayment {
    fn from_row(row: &libsql::Row) -> Result<Self> {
        todo!()
    }
}

