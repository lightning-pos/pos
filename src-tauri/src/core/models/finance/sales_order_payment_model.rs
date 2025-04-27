use chrono::NaiveDateTime;
use derive_more::Display;
use juniper::{GraphQLEnum, GraphQLInputObject};
use lightning_macros::{LibsqlEnum, LibsqlFromRow, SeaQueryCrud, SeaQueryEnum, SeaQueryModel};

use crate::{adapters::outgoing::database::FromLibsqlValue, core::{db::SeaQueryCrudTrait, types::{db_uuid::DbUuid, money::Money}}};

#[derive(Debug, Clone, SeaQueryModel, SeaQueryCrud, LibsqlFromRow)]
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

#[derive(Debug, Clone, Copy, GraphQLEnum, PartialEq, Eq, Display, SeaQueryEnum, LibsqlEnum)]
pub enum SalesOrderPaymentState {
    Completed,
    Failed,
    Voided,
}

