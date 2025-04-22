use chrono::NaiveDateTime;
use juniper::GraphQLInputObject;
use sea_query::Iden;

use crate::core::types::db_uuid::DbUuid;

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
