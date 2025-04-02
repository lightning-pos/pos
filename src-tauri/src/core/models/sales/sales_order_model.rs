use chrono::NaiveDateTime;
use diesel::{
    expression::AsExpression,
    prelude::{AsChangeset, Identifiable, Insertable, Queryable},
    sql_types::Text,
    Selectable,
};
use diesel_derive_enum::DbEnum;
use juniper::{GraphQLEnum, GraphQLInputObject};
use uuid::Uuid;

use crate::core::types::{db_uuid::DbUuid, money::Money};
use crate::schema::sales_orders;

use super::sales_order_charge_model::SalesOrderChargeNewInput;
use super::sales_order_item_model::SalesOrderItemInput;

#[derive(Debug, Queryable, Selectable, Insertable, Identifiable)]
#[diesel(table_name = sales_orders)]
#[diesel(primary_key(id))]
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

#[derive(Debug, Clone, GraphQLInputObject, Identifiable)]
#[diesel(table_name = sales_orders)]
#[diesel(primary_key(id))]
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

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = sales_orders)]
pub struct SalesOrderUpdateChangeset {
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
    pub updated_by: DbUuid,

    // Optional Mappings
    pub discount_id: Option<Option<DbUuid>>,

    // Timestamp
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum, GraphQLEnum, AsExpression)]
#[DbValueStyle = "PascalCase"]
#[diesel(sql_type = Text)]
pub enum SalesOrderState {
    Draft,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum, GraphQLEnum, AsExpression)]
#[DbValueStyle = "PascalCase"]
#[diesel(sql_type = Text)]
pub enum SalesOrderPaymentState {
    Pending,
    PartiallyPaid,
    Paid,
    Refunded,
    PartiallyRefunded,
    Failed,
    Voided,
}

impl SalesOrderUpdateInput {
    pub fn into_changeset(self, updated_by: Uuid, now: NaiveDateTime) -> SalesOrderUpdateChangeset {
        SalesOrderUpdateChangeset {
            customer_id: self.customer_id,
            customer_name: self.customer_name,
            customer_phone_number: self.customer_phone_number,
            billing_address: self.billing_address,
            shipping_address: self.shipping_address,
            net_amount: self.net_amount,
            disc_amount: self.disc_amount,
            taxable_amount: self.taxable_amount,
            tax_amount: self.tax_amount,
            total_amount: self.total_amount,
            order_state: self.order_state,
            payment_state: self.payment_state,
            notes: self.notes,
            channel_id: self.channel_id,
            location_id: self.location_id,
            cost_center_id: self.cost_center_id,
            discount_id: self.discount_id,
            updated_by: updated_by.into(),
            updated_at: now,
        }
    }
}
