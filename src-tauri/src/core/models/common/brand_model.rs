use chrono::NaiveDateTime;
use diesel::{
    prelude::{AsChangeset, Insertable, Queryable},
    Selectable,
};
use juniper::GraphQLInputObject;

use crate::core::types::db_uuid::DbUuid;
use crate::schema::brands;

#[derive(Debug, Clone, Queryable, Selectable, Insertable)]
#[diesel(table_name = brands)]
pub struct Brand {
    pub id: DbUuid,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct BrandNewInput {
    pub name: String,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct BrandUpdateInput {
    pub id: DbUuid,
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = brands)]
pub struct BrandUpdateChangeset {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub is_active: Option<bool>,
    pub updated_at: NaiveDateTime,
}
