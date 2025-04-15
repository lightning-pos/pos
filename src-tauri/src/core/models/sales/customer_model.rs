use chrono::NaiveDateTime;
use diesel::{
    prelude::{AsChangeset, Insertable, Queryable},
    Selectable,
};
use juniper::GraphQLInputObject;
use sea_query::Iden;

use crate::core::types::db_uuid::DbUuid;
use crate::schema::customers;

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = customers)]
pub struct Customer {
    pub id: DbUuid,
    pub full_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct CustomerNewInput {
    pub full_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct CustomerUpdateInput {
    pub id: DbUuid,
    pub full_name: Option<String>,
    pub email: Option<Option<String>>,
    pub phone: Option<Option<String>>,
    pub address: Option<Option<String>>,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = customers)]
pub struct CustomerUpdateChangeset {
    pub id: DbUuid,
    pub full_name: Option<String>,
    pub email: Option<Option<String>>,
    pub phone: Option<Option<String>>,
    pub address: Option<Option<String>>,
    pub updated_at: NaiveDateTime,
}

// Define table and column identifiers for SeaQuery
#[derive(Iden)]
pub enum Customers {
    Table,
    Id,
    FullName,
    Email,
    Phone,
    Address,
    CreatedAt,
    UpdatedAt,
}