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
                sales_order_charge_model::SalesOrderCharge,
                sales_order_item_model::SalesOrderItem,
                sales_order_model::{SalesOrder, SalesOrderPaymentState, SalesOrderState},
            },
        },
        types::{db_uuid::DbUuid, money::Money},
    },
    schema::{cost_centers, customers, sales_order_charges, sales_order_items},
    AppState,
};

#[graphql_object(context = AppState)]
impl SalesOrder {
    pub fn id(&self) -> DbUuid {
        self.id
    }

    pub fn order_readable_id(&self) -> &str {
        &self.order_readable_id
    }

    pub fn customer_id(&self) -> Option<DbUuid> {
        self.customer_id
    }

    pub fn customer_name(&self) -> Option<String> {
        self.customer_name.clone()
    }

    pub fn customer_phone_number(&self) -> Option<String> {
        self.customer_phone_number.clone()
    }

    pub fn billing_address(&self) -> Option<String> {
        self.billing_address.clone()
    }

    pub fn shipping_address(&self) -> Option<String> {
        self.shipping_address.clone()
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

    pub fn order_state(&self) -> SalesOrderState {
        self.order_state
    }

    pub fn payment_state(&self) -> SalesOrderPaymentState {
        self.payment_state
    }

    pub fn notes(&self) -> Option<String> {
        self.notes.clone()
    }

    pub fn channel_id(&self) -> DbUuid {
        self.channel_id
    }

    pub fn location_id(&self) -> DbUuid {
        self.location_id
    }

    pub fn cost_center_id(&self) -> DbUuid {
        self.cost_center_id
    }

    pub fn created_by(&self) -> DbUuid {
        self.created_by
    }

    pub fn updated_by(&self) -> DbUuid {
        self.updated_by
    }

    pub fn discount_id(&self) -> Option<DbUuid> {
        self.discount_id
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }

    // Relationships
    pub fn customer(&self, context: &AppState) -> FieldResult<Option<Customer>> {
        if let Some(customer_id) = self.customer_id {
            let mut service = context.service.lock().unwrap();
            let customer = customers::table
                .find(customer_id)
                .select(Customer::as_select())
                .first::<Customer>(&mut service.conn)?;
            Ok(Some(customer))
        } else {
            Ok(None)
        }
    }

    pub fn cost_center(&self, context: &AppState) -> FieldResult<CostCenter> {
        let mut service = context.service.lock().unwrap();
        let cost_center = cost_centers::table
            .find(self.cost_center_id)
            .select(CostCenter::as_select())
            .first::<CostCenter>(&mut service.conn)?;
        Ok(cost_center)
    }

    pub fn items(&self, context: &AppState) -> FieldResult<Vec<SalesOrderItem>> {
        let mut service = context.service.lock().unwrap();
        let items = sales_order_items::table
            .filter(sales_order_items::order_id.eq(self.id))
            .select(SalesOrderItem::as_select())
            .load::<SalesOrderItem>(&mut service.conn)?;
        Ok(items)
    }

    pub fn charges(&self, context: &AppState) -> FieldResult<Vec<SalesOrderCharge>> {
        let mut service = context.service.lock().unwrap();
        let charges = sales_order_charges::table
            .filter(sales_order_charges::order_id.eq(self.id))
            .select(SalesOrderCharge::as_select())
            .load::<SalesOrderCharge>(&mut service.conn)?;
        Ok(charges)
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
