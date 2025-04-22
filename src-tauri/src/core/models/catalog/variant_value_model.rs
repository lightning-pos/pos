use chrono::NaiveDateTime;
use juniper::GraphQLInputObject;
use sea_query::Iden;

use crate::core::types::db_uuid::DbUuid;

#[derive(Debug, Clone)]
pub struct VariantValue {
    pub id: DbUuid,
    pub variant_type_id: DbUuid,
    pub value: String,
    pub display_order: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct VariantValueNewInput {
    pub variant_type_id: DbUuid,
    pub value: String,
    pub display_order: Option<i32>,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct VariantValueUpdateInput {
    pub id: DbUuid,
    pub value: Option<String>,
    pub display_order: Option<i32>,
    pub updated_at: Option<NaiveDateTime>,
}

// Define table and column identifiers for SeaQuery
#[derive(Iden)]
pub enum VariantValues {
    Table,
    Id,
    VariantTypeId,
    Value,
    DisplayOrder,
    CreatedAt,
    UpdatedAt,
}
