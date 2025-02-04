use chrono::NaiveDateTime;
use diesel::{
    prelude::{AsChangeset, Insertable, Queryable},
    Selectable,
};
use juniper::GraphQLInputObject;

use crate::core::types::db_uuid::DbUuid;
use crate::schema::customers;

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = customers)]
pub struct Customer {
    pub id: DbUuid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub country_code: Option<String>,
    pub phone_number: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct CustomerNewInput {
    pub name: Option<String>,
    pub email: Option<String>,
    pub country_code: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct CustomerUpdateInput {
    pub id: DbUuid,
    pub name: Option<Option<String>>,
    pub email: Option<Option<String>>,
    pub country_code: Option<Option<String>>,
    pub phone_number: Option<Option<String>>,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = customers)]
pub struct CustomerUpdateChangeset {
    pub id: DbUuid,
    pub name: Option<Option<String>>,
    pub email: Option<Option<String>>,
    pub country_code: Option<Option<String>>,
    pub phone_number: Option<Option<String>>,
    pub updated_at: NaiveDateTime,
}
