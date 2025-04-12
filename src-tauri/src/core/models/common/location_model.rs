use chrono::NaiveDateTime;
use diesel::{
    prelude::{AsChangeset, Insertable, Queryable},
    Selectable,
};
use juniper::GraphQLInputObject;

use crate::core::types::db_uuid::DbUuid;
use crate::schema::locations;

#[derive(Debug, Clone, Queryable, Selectable, Insertable)]
#[diesel(table_name = locations)]
pub struct Location {
    pub id: DbUuid,
    pub name: String,
    pub description: Option<String>,
    pub address: Option<String>,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct LocationNewInput {
    pub name: String,
    pub description: Option<String>,
    pub address: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct LocationUpdateInput {
    pub id: DbUuid,
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub address: Option<Option<String>>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = locations)]
pub struct LocationUpdateChangeset {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub address: Option<Option<String>>,
    pub is_active: Option<bool>,
    pub updated_at: NaiveDateTime,
}

// Helper to create changeset from input
impl LocationUpdateInput {
    pub fn into_changeset(self, now: NaiveDateTime) -> LocationUpdateChangeset {
        LocationUpdateChangeset {
            name: self.name,
            description: self.description,
            address: self.address,
            is_active: self.is_active,
            updated_at: now,
        }
    }
}