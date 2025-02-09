use chrono::NaiveDateTime;
use diesel::{
    query_dsl::methods::{FilterDsl, FindDsl, SelectDsl},
    ExpressionMethods, RunQueryDsl, SelectableHelper,
};
use juniper::{graphql_object, FieldResult};

use crate::{
    core::{
        models::sales::{
            customer_model::Customer,
            sales_order_item_model::SalesOrderItem,
            sales_order_model::{SalesOrder, SalesOrderState},
        },
        types::{db_uuid::DbUuid, money::Money},
    },
    schema::{customers, sales_order_items},
    AppState,
};

#[graphql_object(context = AppState)]
impl SalesOrder {
    pub fn id(&self) -> DbUuid {
        self.id
    }

    pub fn customer_id(&self) -> DbUuid {
        self.customer_id
    }

    pub fn customer_name(&self) -> String {
        self.customer_name.clone()
    }

    pub fn customer_phone_number(&self) -> String {
        self.customer_phone_number.clone()
    }

    pub fn order_date(&self) -> NaiveDateTime {
        self.order_date
    }

    pub fn net_amount(&self) -> Money {
        self.net_amount
    }

    pub fn disc_amount(&self) -> Money {
        self.disc_amount
    }

    pub fn taxable_amount(&self) -> Money {
        self.taxable_amount
    }

    pub fn tax_amount(&self) -> Money {
        self.tax_amount
    }

    pub fn total_amount(&self) -> Money {
        self.total_amount
    }

    pub fn state(&self) -> SalesOrderState {
        self.state
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }

    // Relationships
    pub fn customer(&self, context: &AppState) -> FieldResult<Customer> {
        let mut service = context.service.lock().unwrap();
        let customer = customers::table
            .find(self.customer_id)
            .select(Customer::as_select())
            .first::<Customer>(&mut service.conn)?;
        Ok(customer)
    }

    pub fn items(&self, context: &AppState) -> FieldResult<Vec<SalesOrderItem>> {
        let mut service = context.service.lock().unwrap();
        let items = sales_order_items::table
            .filter(sales_order_items::order_id.eq(self.id))
            .select(SalesOrderItem::as_select())
            .load::<SalesOrderItem>(&mut service.conn)?;
        Ok(items)
    }
}
