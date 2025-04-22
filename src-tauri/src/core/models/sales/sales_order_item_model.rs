use chrono::NaiveDateTime;
use juniper::GraphQLInputObject;
use sea_query::Iden;

use crate::core::types::{db_uuid::DbUuid, money::Money};

#[derive(Debug)]
pub struct SalesOrderItem {
    pub id: DbUuid,
    pub order_id: DbUuid,
    pub item_id: Option<DbUuid>,
    pub item_name: String,
    pub quantity: i32,
    pub sku: Option<String>,
    pub price_amount: Money,
    pub disc_amount: Money,
    pub taxable_amount: Money,
    pub tax_amount: Money,
    pub total_amount: Money,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct SalesOrderItemInput {
    pub item_id: Option<DbUuid>,
    pub item_name: String,
    pub quantity: i32,
    pub sku: Option<String>,
    pub price_amount: Money,
    pub disc_amount: Money,
    pub taxable_amount: Money,
    pub tax_amount: Money,
    pub total_amount: Money,
}

// Define table and column identifiers for SeaQuery
#[derive(Iden)]
pub enum SalesOrderItems {
    Table,
    Id,
    OrderId,
    ItemId,
    ItemName,
    Quantity,
    Sku,
    PriceAmount,
    DiscAmount,
    TaxableAmount,
    TaxAmount,
    TotalAmount,
    CreatedAt,
    UpdatedAt,
}