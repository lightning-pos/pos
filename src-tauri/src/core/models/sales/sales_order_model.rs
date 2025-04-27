use chrono::NaiveDateTime;
use derive_more::Display;
use juniper::{GraphQLEnum, GraphQLInputObject};
use lightning_macros::{LibsqlEnum, LibsqlFromRow, SeaQueryCrud, SeaQueryEnum, SeaQueryModel};

use crate::adapters::outgoing::database::FromLibsqlValue;
use crate::core::db::SeaQueryCrudTrait;
use crate::core::types::{db_uuid::DbUuid, money::Money};

use super::sales_order_charge_model::SalesOrderChargeNewInput;
use super::sales_order_item_model::SalesOrderItemInput;

#[derive(Debug, Clone, SeaQueryModel, SeaQueryCrud, LibsqlFromRow)]
pub struct SalesOrder {
    pub id: DbUuid,
    pub order_readable_id: String,
    pub order_date: NaiveDateTime,

    // Customer
    pub customer_id: Option<DbUuid>,
    pub customer_name: Option<String>,
    pub customer_phone_number: Option<String>,
    pub billing_address: Option<String>,
    pub shipping_address: Option<String>,

    // Amounts
    pub net_amount: Money,
    pub disc_amount: Money,
    pub taxable_amount: Money,
    pub tax_amount: Money,
    pub total_amount: Money,

    // State
    pub order_state: SalesOrderState,
    pub payment_state: SalesOrderPaymentState,

    // Notes
    pub notes: Option<String>,

    // Mappings
    pub channel_id: DbUuid,
    pub location_id: DbUuid,
    pub cost_center_id: DbUuid,
    pub created_by: DbUuid,
    pub updated_by: DbUuid,

    // Optional Mappings
    pub discount_id: Option<DbUuid>,

    // Timestamps
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct SalesOrderNewInput {
    pub order_date: NaiveDateTime,

    // Customer
    pub customer_id: Option<DbUuid>,
    pub customer_name: Option<String>,
    pub customer_phone_number: Option<String>,
    pub billing_address: Option<String>,
    pub shipping_address: Option<String>,

    // Amounts
    pub net_amount: Money,
    pub disc_amount: Money,
    pub taxable_amount: Money,
    pub tax_amount: Money,
    pub total_amount: Money,

    // Notes
    pub notes: Option<String>,

    // Mappings
    pub channel_id: DbUuid,
    pub location_id: DbUuid,
    pub cost_center_id: DbUuid,

    // Optional Mappings
    pub discount_id: Option<DbUuid>,

    // Associated items and charges
    pub items: Vec<SalesOrderItemInput>,
    pub charges: Option<Vec<SalesOrderChargeNewInput>>,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct SalesOrderUpdateInput {
    pub id: DbUuid,

    // Customer
    pub customer_id: Option<Option<DbUuid>>,
    pub customer_name: Option<Option<String>>,
    pub customer_phone_number: Option<Option<String>>,
    pub billing_address: Option<Option<String>>,
    pub shipping_address: Option<Option<String>>,

    // Amounts
    pub net_amount: Option<Money>,
    pub disc_amount: Option<Money>,
    pub taxable_amount: Option<Money>,
    pub tax_amount: Option<Money>,
    pub total_amount: Option<Money>,

    // State
    pub order_state: Option<SalesOrderState>,
    pub payment_state: Option<SalesOrderPaymentState>,

    // Notes
    pub notes: Option<Option<String>>,

    // Mappings
    pub channel_id: Option<DbUuid>,
    pub location_id: Option<DbUuid>,
    pub cost_center_id: Option<DbUuid>,

    // Optional Mappings
    pub discount_id: Option<Option<DbUuid>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, GraphQLEnum, Display, SeaQueryEnum, LibsqlEnum)]
pub enum SalesOrderState {
    Draft,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, GraphQLEnum, Display, SeaQueryEnum, LibsqlEnum)]
pub enum SalesOrderPaymentState {
    Pending,
    PartiallyPaid,
    Paid,
    Refunded,
    PartiallyRefunded,
    Failed,
    Voided,
}
