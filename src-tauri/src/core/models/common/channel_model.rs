use chrono::NaiveDateTime;
use juniper::GraphQLInputObject;
use sea_query::Iden;

use crate::{adapters::outgoing::database::FromRow, core::types::db_uuid::DbUuid, error::Result};

#[derive(Debug, Clone)]
pub struct Channel {
    pub id: DbUuid,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ChannelNewInput {
    pub name: String,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ChannelUpdateInput {
    pub id: DbUuid,
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub is_active: Option<bool>,
}

#[derive(Iden)]
pub enum Channels {
    Table,
    Id,
    Name,
    Description,
    IsActive,
    CreatedAt,
    UpdatedAt,
}


impl FromRow<libsql::Row> for Channel {
    fn from_row(row: &libsql::Row) -> Result<Self> {
        todo!()
    }
}
