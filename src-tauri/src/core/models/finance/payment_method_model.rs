use chrono::NaiveDateTime;
use derive_more::Display;
use juniper::{GraphQLEnum, GraphQLInputObject};
use sea_query::Iden;

use crate::{adapters::outgoing::database::FromRow, core::types::db_uuid::DbUuid, error::Result};

#[derive(Debug, Clone)]
pub struct PaymentMethod {
    pub id: DbUuid,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub state: PaymentMethodState,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct PaymentMethodNewInput {
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub state: Option<PaymentMethodState>,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct PaymentMethodUpdateInput {
    pub id: DbUuid,
    pub name: Option<String>,
    pub code: Option<String>,
    pub description: Option<Option<String>>,
    pub state: Option<PaymentMethodState>,
}

#[derive(Debug, Clone, Copy, GraphQLEnum, PartialEq, Eq, Display)]
pub enum PaymentMethodState {
    Active,
    Inactive,
}

// Define table and column identifiers for SeaQuery
#[derive(Iden)]
pub enum PaymentMethods {
    Table,
    Id,
    Name,
    Code,
    Description,
    State,
    CreatedAt,
    UpdatedAt,
}

impl FromRow<libsql::Row> for PaymentMethod {
    fn from_row(row: &libsql::Row) -> Result<Self> {
        todo!()
    }
}