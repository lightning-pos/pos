use chrono::NaiveDateTime;
use juniper::GraphQLInputObject;
use sea_query::Iden;

use crate::{adapters::outgoing::database::FromRow, core::types::db_uuid::DbUuid, error::Result};

#[derive(Debug, Clone)]
pub struct VariantType {
    pub id: DbUuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct VariantTypeNewInput {
    pub name: String,
    pub description: Option<String>,
}

// Define table and column identifiers for SeaQuery
#[derive(Iden)]
pub enum VariantTypes {
    Table,
    Id,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct VariantTypeUpdateInput {
    pub id: DbUuid,
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub updated_at: Option<NaiveDateTime>,
}

impl FromRow<libsql::Row> for VariantType {
    fn from_row(row: &libsql::Row) -> Result<Self> {
        todo!()
    }
}

