use chrono::NaiveDateTime;
use juniper::GraphQLInputObject;
use lightning_macros::{LibsqlFromRow, SeaQueryCrud, SeaQueryModel};

use crate::core::{db::SeaQueryCrudTrait, types::db_uuid::DbUuid};

#[derive(Debug, Clone, SeaQueryModel, SeaQueryCrud, LibsqlFromRow)]
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

#[derive(Debug, Clone, SeaQueryModel, SeaQueryCrud, LibsqlFromRow)]
pub struct TaxGroupTax {
    #[sea_query(primary_key)]
    pub tax_group_id: DbUuid,
    #[sea_query(primary_key)]
    pub tax_id: DbUuid,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct TaxGroupTaxNewInput {
    pub tax_group_id: DbUuid,
    pub tax_id: DbUuid,
}
