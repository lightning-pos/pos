use crate::{core::types::db_uuid::DbUuid, schema::purchase_categories};
use chrono::NaiveDateTime;
use derive_more::Display;
use diesel::prelude::{AsChangeset, Insertable, Queryable, Selectable};
use diesel_derive_enum::DbEnum;
use juniper::{GraphQLEnum, GraphQLInputObject};
use sea_query::Iden;

#[derive(Debug, Queryable, Insertable, Selectable)]
#[diesel(table_name = purchase_categories)]
pub struct PurchaseCategory {
    pub id: DbUuid,
    pub name: String,
    pub description: Option<String>,
    pub state: PurchaseCategoryState,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct PurchaseCategoryNew {
    pub name: String,
    pub description: Option<String>,
    pub state: Option<PurchaseCategoryState>,
}

#[derive(Debug, Clone, AsChangeset, GraphQLInputObject)]
#[diesel(table_name = purchase_categories)]
pub struct PurchaseCategoryUpdate {
    pub id: DbUuid,
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub state: Option<PurchaseCategoryState>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Copy, DbEnum, GraphQLEnum, PartialEq, Eq, Display)]
pub enum PurchaseCategoryState {
    Active,
    Inactive,
    Deleted,
}

// Define table and column identifiers for SeaQuery
#[derive(Iden)]
pub enum PurchaseCategories {
    Table,
    Id,
    Name,
    Description,
    State,
    CreatedAt,
    UpdatedAt,
}