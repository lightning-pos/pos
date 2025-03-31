use chrono::NaiveDateTime;
use diesel::{
    query_dsl::methods::{FilterDsl, FindDsl, SelectDsl},
    ExpressionMethods, RunQueryDsl, SelectableHelper,
};
use juniper::{graphql_object, FieldResult};

use crate::{
    core::{
        commands::{finance::sales_order_payment_commands::GetSalesOrderPaymentsCommand, Command},
        models::{
            finance::{
                cost_center_model::CostCenter, sales_order_payment_model::SalesOrderPayment,
            },
            sales::{
                customer_model::Customer,
                sales_order_item_model::SalesOrderItem,
                sales_order_model::{SalesOrder, SalesOrderState},
            },
        },
        types::{db_uuid::DbUuid, money::Money},
    },
    schema::{cost_centers, customers, sales_order_items},
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

    pub fn cost_center_id(&self) -> DbUuid {
        self.cost_center_id
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

    pub fn cost_center(&self, context: &AppState) -> FieldResult<Option<CostCenter>> {
        let mut service = context.service.lock().unwrap();
        let cost_center = cost_centers::table
            .find(self.cost_center_id)
            .select(CostCenter::as_select())
            .first::<CostCenter>(&mut service.conn)?;
        Ok(Some(cost_center))
    }

    pub fn items(&self, context: &AppState) -> FieldResult<Vec<SalesOrderItem>> {
        let mut service = context.service.lock().unwrap();
        let items = sales_order_items::table
            .filter(sales_order_items::order_id.eq(&self.id))
            .load::<SalesOrderItem>(&mut service.conn)?;
        Ok(items)
    }

    pub fn payments(&self, context: &AppState) -> FieldResult<Vec<SalesOrderPayment>> {
        let mut service = context.service.lock().unwrap();
        let cmd = GetSalesOrderPaymentsCommand { order_id: self.id };
        let payments = cmd.exec(&mut service)?;
        Ok(payments)
    }

    pub fn total_paid_amount(&self, context: &AppState) -> FieldResult<Money> {
        let mut service = context.service.lock().unwrap();
        let cmd = GetSalesOrderPaymentsCommand { order_id: self.id };
        let payments = cmd.exec(&mut service)?;

        let total: Money = payments
            .iter()
            .filter(|p| p.state == crate::core::models::finance::sales_order_payment_model::SalesOrderPaymentState::Completed)
            .map(|p| p.amount)
            .sum();

        Ok(total)
    }
}
