use chrono::NaiveDateTime;
use juniper::GraphQLInputObject;
use sea_query::Iden;

use crate::{adapters::outgoing::database::FromRow, core::types::db_uuid::DbUuid, error::Result};

#[derive(Debug)]
pub struct SalesChargeType {
    pub id: DbUuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct SalesChargeTypeNewInput {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct SalesChargeTypeUpdateInput {
    pub id: DbUuid,
    pub name: Option<String>,
    pub description: Option<Option<String>>, // Double optional for nullable field
}

// Define table and column identifiers for SeaQuery
#[derive(Iden)]
pub enum SalesChargeTypes {
    Table,
    Id,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
}

impl FromRow<libsql::Row> for SalesChargeType {
    fn from_row(row: &libsql::Row) -> Result<Self> {
        todo!()
    }
}