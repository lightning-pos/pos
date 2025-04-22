use chrono::NaiveDateTime;
use juniper::GraphQLInputObject;
use sea_query::Iden;

use crate::core::types::db_uuid::DbUuid;

#[derive(Debug)]
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