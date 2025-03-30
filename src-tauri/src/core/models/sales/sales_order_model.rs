use chrono::NaiveDateTime;
use diesel::{
    expression::AsExpression,
    prelude::{AsChangeset, Insertable, Queryable},
    sql_types::Text,
    Selectable,
};
use diesel_derive_enum::DbEnum;
use juniper::{GraphQLEnum, GraphQLInputObject};

use crate::core::types::{db_uuid::DbUuid, money::Money};
use crate::schema::sales_orders;

use super::sales_order_item_model::SalesOrderItemInput;

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = sales_orders)]
pub struct SalesOrder {
    pub id: DbUuid,
    pub customer_id: DbUuid,
    pub customer_name: String,
    pub customer_phone_number: String,
    pub order_date: NaiveDateTime,
    pub net_amount: Money,
    pub disc_amount: Money,
    pub taxable_amount: Money,
    pub tax_amount: Money,
    pub total_amount: Money,
    pub state: SalesOrderState,
    pub cost_center_id: DbUuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct SalesOrderNewInput {
    pub customer_id: DbUuid,
    pub customer_name: String,
    pub customer_phone_number: String,
    pub order_date: NaiveDateTime,
    pub net_amount: Money,
    pub disc_amount: Money,
    pub taxable_amount: Money,
    pub tax_amount: Money,
    pub total_amount: Money,
    pub state: SalesOrderState,
    pub cost_center_id: DbUuid,
    pub items: Vec<SalesOrderItemInput>,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct SalesOrderUpdateInput {
    pub id: DbUuid,
    pub customer_name: Option<String>,
    pub customer_phone_number: Option<String>,
    pub order_date: Option<NaiveDateTime>,
    pub net_amount: Option<Money>,
    pub disc_amount: Option<Money>,
    pub taxable_amount: Option<Money>,
    pub tax_amount: Option<Money>,
    pub total_amount: Option<Money>,
    pub state: Option<SalesOrderState>,
    pub cost_center_id: Option<DbUuid>,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = sales_orders)]
pub struct SalesOrderUpdateChangeset {
    pub id: DbUuid,
    pub customer_name: Option<String>,
    pub customer_phone_number: Option<String>,
    pub order_date: Option<NaiveDateTime>,
    pub net_amount: Option<Money>,
    pub disc_amount: Option<Money>,
    pub taxable_amount: Option<Money>,
    pub tax_amount: Option<Money>,
    pub total_amount: Option<Money>,
    pub state: Option<SalesOrderState>,
    pub cost_center_id: Option<DbUuid>,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum, GraphQLEnum, AsExpression)]
#[diesel(sql_type = Text)]
pub enum SalesOrderState {
    Draft,
    Completed,
    Cancelled,
}
