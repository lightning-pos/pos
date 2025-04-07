use chrono::NaiveDateTime;
use diesel::{
    prelude::{AsChangeset, Insertable, Queryable},
    Selectable,
};
use juniper::GraphQLInputObject;

use crate::core::types::db_uuid::DbUuid;
use crate::schema::variant_types;

#[derive(Debug, Clone, Queryable, Selectable, Insertable)]
#[diesel(table_name = variant_types)]
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

#[derive(Debug, Clone, AsChangeset, GraphQLInputObject)]
#[diesel(table_name = variant_types)]
pub struct VariantTypeUpdateInput {
    pub id: DbUuid,
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub updated_at: Option<NaiveDateTime>,
}
