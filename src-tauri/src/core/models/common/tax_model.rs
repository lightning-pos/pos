use chrono::NaiveDateTime;
use diesel::{
    prelude::{AsChangeset, Insertable, Queryable},
    Associations, Selectable,
};
use juniper::GraphQLInputObject;

use crate::core::types::db_uuid::DbUuid;
use crate::schema::{item_taxes, taxes};

#[derive(Debug, Clone, Queryable, Selectable, Insertable)]
#[diesel(table_name = taxes)]
pub struct Tax {
    pub id: DbUuid,
    pub name: String,
    pub rate: i32, // Stored as basis points (e.g., 1000 = 10%)
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct TaxNewInput {
    pub name: String,
    pub rate: i32,
    pub description: Option<String>,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct TaxUpdateInput {
    pub id: DbUuid,
    pub name: Option<String>,
    pub rate: Option<i32>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = taxes)]
pub struct TaxUpdateChangeset {
    pub name: Option<String>,
    pub rate: Option<i32>,
    pub description: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Queryable, Insertable, Associations)]
#[diesel(belongs_to(Tax, foreign_key = tax_id))]
#[diesel(belongs_to(crate::core::models::catalog::item_model::Item, foreign_key = item_id))]
#[diesel(table_name = item_taxes)]
pub struct ItemTax {
    pub item_id: DbUuid,
    pub tax_id: DbUuid,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ItemTaxNewInput {
    pub item_id: DbUuid,
    pub tax_id: DbUuid,
}
