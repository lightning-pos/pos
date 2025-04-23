use chrono::NaiveDateTime;
use juniper::GraphQLInputObject;
use sea_query::Iden;

use crate::{adapters::outgoing::database::FromRow, core::types::db_uuid::DbUuid, error::Result};

#[derive(Debug)]
pub struct Cart {
    pub id: DbUuid,
    pub cart_data: String,
    pub customer_id: Option<DbUuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct CartNewInput {
    pub customer_id: Option<DbUuid>,
    pub cart_data: String,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct CartUpdateInput {
    pub id: DbUuid,
    pub cart_data: Option<String>,
}

// Define table and column identifiers for SeaQuery
#[derive(Iden)]
pub enum Carts {
    Table,
    Id,
    CartData,
    CustomerId,
    CreatedAt,
    UpdatedAt,
}

impl FromRow<libsql::Row> for Cart {
    fn from_row(row: &libsql::Row) -> Result<Self> {
        todo!()
    }
}
