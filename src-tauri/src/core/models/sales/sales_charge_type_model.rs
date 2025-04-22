use chrono::NaiveDateTime;
use juniper::GraphQLInputObject;
use sea_query::Iden;

use crate::core::types::db_uuid::DbUuid;

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