use chrono::NaiveDateTime;
use diesel::prelude::{AsChangeset, Insertable, Queryable, Selectable};
use diesel_derive_enum::DbEnum;
use juniper::{GraphQLEnum, GraphQLInputObject};

use crate::core::types::{db_uuid::DbUuid, money::Money};
use crate::schema::sales_order_payments;

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = sales_order_payments)]
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

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = sales_order_payments)]
pub struct SalesOrderPaymentUpdateChangeset {
    pub id: DbUuid,
    pub payment_method_id: Option<DbUuid>,
    pub payment_date: Option<NaiveDateTime>,
    pub amount: Option<Money>,
    pub reference_number: Option<Option<String>>,
    pub notes: Option<Option<String>>,
    pub state: Option<SalesOrderPaymentState>,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Copy, DbEnum, GraphQLEnum, PartialEq, Eq)]
pub enum SalesOrderPaymentState {
    Completed,
    Voided,
}
