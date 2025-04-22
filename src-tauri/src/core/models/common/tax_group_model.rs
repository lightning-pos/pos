use chrono::NaiveDateTime;
use juniper::GraphQLInputObject;
use sea_query::Iden;

use crate::core::types::db_uuid::DbUuid;

#[derive(Debug, Clone)]
pub struct TaxGroup {
    pub id: DbUuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct TaxGroupNewInput {
    pub name: String,
    pub description: Option<String>,
    pub tax_ids: Option<Vec<DbUuid>>, // Optional list of taxes to initially assign to this group
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct TaxGroupUpdateInput {
    pub id: DbUuid,
    pub name: Option<String>,
    pub description: Option<Option<String>>, // Double optional for nullable field
}

#[derive(Debug, Clone)]
pub struct TaxGroupTax {
    pub tax_group_id: DbUuid,
    pub tax_id: DbUuid,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct TaxGroupTaxNewInput {
    pub tax_group_id: DbUuid,
    pub tax_id: DbUuid,
}

#[derive(Iden)]
pub enum TaxGroups {
    Table,
    Id,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum TaxGroupTaxes {
    Table,
    TaxGroupId,
    TaxId,
}
