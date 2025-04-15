use chrono::NaiveDateTime;
use diesel::{
    prelude::{AsChangeset, Insertable, Queryable},
    Selectable,
};
use juniper::GraphQLInputObject;
use sea_query::Iden;

use crate::core::types::db_uuid::DbUuid;
use crate::schema::suppliers;

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = suppliers)]
pub struct Supplier {
    pub id: DbUuid,
    pub name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct SupplierNewInput {
    pub name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct SupplierUpdateInput {
    pub id: DbUuid,
    pub name: Option<String>,
    pub address: Option<Option<String>>,
    pub phone: Option<Option<String>>,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = suppliers)]
pub struct SupplierUpdateChangeset {
    pub id: DbUuid,
    pub name: Option<String>,
    pub address: Option<Option<String>>,
    pub phone: Option<Option<String>>,
    pub updated_at: NaiveDateTime,
}

// Define table and column identifiers for SeaQuery
#[derive(Iden)]
pub enum Suppliers {
    Table,
    Id,
    Name,
    Address,
    Phone,
    CreatedAt,
    UpdatedAt,
}