use chrono::NaiveDateTime;
use diesel::{
    prelude::{Insertable, Queryable},
    Selectable,
};
use juniper::GraphQLInputObject;

use crate::{
    core::types::{db_uuid::DbUuid, money::Money},
    schema::sales_order_items,
};

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = sales_order_items)]
pub struct SalesOrderItem {
    pub id: DbUuid,
    pub order_id: DbUuid,
    pub item_id: DbUuid,
    pub item_name: String,
    pub quantity: i32,
    pub price_amount: Money,
    pub tax_amount: Money,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct SalesOrderItemInput {
    pub item_id: DbUuid,
    pub item_name: String,
    pub quantity: i32,
    pub price_amount: Money,
    pub tax_amount: Money,
}
