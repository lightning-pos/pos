use chrono::NaiveDateTime;
use juniper::GraphQLInputObject;
use sea_query::Iden;

use crate::{adapters::outgoing::database::FromRow, core::types::db_uuid::DbUuid, error::Result};

#[derive(Debug)]
pub struct Customer {
    pub id: DbUuid,
    pub full_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct CustomerNewInput {
    pub full_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct CustomerUpdateInput {
    pub id: DbUuid,
    pub full_name: Option<String>,
    pub email: Option<Option<String>>,
    pub phone: Option<Option<String>>,
    pub address: Option<Option<String>>,
}

// Define table and column identifiers for SeaQuery
#[derive(Iden)]
pub enum Customers {
    Table,
    Id,
    FullName,
    Email,
    Phone,
    Address,
    CreatedAt,
    UpdatedAt,
}

impl FromRow<libsql::Row> for Customer {
    fn from_row(row: &libsql::Row) -> Result<Self> {
        todo!()
    }
}