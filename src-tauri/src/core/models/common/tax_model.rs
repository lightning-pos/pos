use chrono::NaiveDateTime;
use juniper::GraphQLInputObject;
use sea_query::Iden;

use crate::core::types::db_uuid::DbUuid;
use crate::core::types::percentage::Percentage;

#[derive(Debug, Clone)]
pub struct Tax {
    pub id: DbUuid,
    pub name: String,
    pub rate: Percentage,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct TaxNewInput {
    pub name: String,
    pub rate: Percentage,
    pub description: Option<String>,
    pub item_ids: Option<Vec<DbUuid>>, // Optional list of items to initially assign this tax to
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct TaxUpdateInput {
    pub id: DbUuid,
    pub name: Option<String>,
    pub rate: Option<Percentage>,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ItemTax {
    pub item_id: DbUuid,
    pub tax_id: DbUuid,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ItemTaxNewInput {
    pub item_id: DbUuid,
    pub tax_id: DbUuid,
}

#[derive(Iden)]
pub enum Taxes {
    Table,
    Id,
    Name,
    Rate,
    Description,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum ItemTaxes {
    Table,
    ItemId,
    TaxId,
}
