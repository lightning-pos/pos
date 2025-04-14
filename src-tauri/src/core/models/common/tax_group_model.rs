use chrono::NaiveDateTime;
use diesel::{
    prelude::{AsChangeset, Identifiable, Insertable, Queryable},
    Associations, Selectable,
};
use juniper::GraphQLInputObject;
use sea_query::Iden;

use crate::core::types::db_uuid::DbUuid;
use crate::schema::{tax_group_taxes, tax_groups};

#[derive(Debug, Clone, Queryable, Selectable, Insertable, Identifiable)]
#[diesel(table_name = tax_groups)]
#[diesel(primary_key(id))]
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

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = tax_groups)]
pub struct TaxGroupUpdateChangeset {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub updated_at: NaiveDateTime,
}

// Helper to create changeset from input
impl TaxGroupUpdateInput {
    pub fn into_changeset(self, now: NaiveDateTime) -> TaxGroupUpdateChangeset {
        TaxGroupUpdateChangeset {
            name: self.name,
            description: self.description,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Queryable, Insertable, Associations, Identifiable)]
#[diesel(belongs_to(TaxGroup, foreign_key = tax_group_id))]
#[diesel(table_name = tax_group_taxes)]
#[diesel(primary_key(tax_group_id, tax_id))]
pub struct TaxGroupTax {
    pub tax_group_id: DbUuid,
    pub tax_id: DbUuid,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct TaxGroupTaxNewInput {
    pub tax_group_id: DbUuid,
    pub tax_id: DbUuid,
}

#[derive(Debug, Iden)]
#[iden = "tax_groups"]
pub enum TaxGroups {
    Table,
    Id,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
}

#[derive(Debug, Iden)]
#[iden = "tax_group_taxes"]
pub enum TaxGroupTaxes {
    Table,
    TaxGroupId,
    TaxId,
}
