use chrono::NaiveDateTime;
use diesel::{
    prelude::{AsChangeset, Insertable, Queryable},
    Selectable,
};
use juniper::GraphQLInputObject;

use crate::core::types::db_uuid::DbUuid;
use crate::schema::carts;

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = carts)]
pub struct Cart {
    pub id: DbUuid,
    pub customer_id: DbUuid,
    pub cart_data: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct CartNewInput {
    pub customer_id: DbUuid,
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
