use chrono::NaiveDateTime;
use juniper::GraphQLInputObject;
use sea_query::Iden;

use crate::{adapters::outgoing::database::FromRow, core::types::db_uuid::DbUuid, error::Result};

#[derive(Debug, Clone)]
pub struct Brand {
    pub id: DbUuid,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct BrandNewInput {
    pub name: String,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct BrandUpdateInput {
    pub id: DbUuid,
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub is_active: Option<bool>,
}

#[derive(Iden)]
pub enum Brands {
    Table,
    Id,
    Name,
    Description,
    IsActive,
    CreatedAt,
    UpdatedAt,
}


impl FromRow<libsql::Row> for Brand {
    fn from_row(row: &libsql::Row) -> Result<Self> {
        todo!()
    }
}
