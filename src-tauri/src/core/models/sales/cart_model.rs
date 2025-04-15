use chrono::NaiveDateTime;
use diesel::{
    prelude::{AsChangeset, Insertable, Queryable},
    Selectable,
};
use juniper::GraphQLInputObject;
use sea_query::Iden;

use crate::core::types::db_uuid::DbUuid;
use crate::schema::carts;

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = carts)]
pub struct Cart {
    pub id: DbUuid,
    pub cart_data: String,
    pub customer_id: Option<DbUuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct CartNewInput {
    pub customer_id: Option<DbUuid>,
    pub cart_data: String,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct CartUpdateInput {
    pub id: DbUuid,
    pub cart_data: Option<String>,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = carts)]
pub struct CartUpdateChangeset {
    pub id: DbUuid,
    pub cart_data: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
}

// Define table and column identifiers for SeaQuery
#[derive(Iden)]
pub enum Carts {
    Table,
    Id,
    CartData,
    CustomerId,
    CreatedAt,
    UpdatedAt,
}