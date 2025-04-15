use chrono::NaiveDateTime;
use sea_query::{Expr, Query, SqliteQueryBuilder};
use juniper::{graphql_object, FieldResult};

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        commands::{finance::sales_order_payment_commands::GetSalesOrderPaymentsCommand, Command},
        models::{
            finance::{
                cost_center_model::{CostCenter, CostCenters},
                sales_order_payment_model::{SalesOrderPayment, SalesOrderPaymentState},
            },
            sales::{
                customer_model::{Customer, Customers},
                sales_order_charge_model::{SalesOrderCharge, SalesOrderCharges},
                sales_order_item_model::{SalesOrderItem, SalesOrderItems},
                sales_order_model::{SalesOrder, SalesOrderPaymentState as OrderPaymentState, SalesOrderState},
            },
        },
        types::{db_uuid::DbUuid, money::Money},
    },
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

    pub fn payment_state(&self) -> OrderPaymentState {
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
            let service = context.service.lock().unwrap();
            
            let query = Query::select()
                .from(Customers::Table)
                .columns([
                    Customers::Id,
                    Customers::FullName,
                    Customers::Email,
                    Customers::Phone,
                    Customers::Address,
                    Customers::CreatedAt,
                    Customers::UpdatedAt,
                ])
                .and_where(Expr::col(Customers::Id).eq(customer_id.to_string()))
                .to_string(SqliteQueryBuilder);
                
            let customer = service.db_adapter.query_one::<Customer>(&query, vec![])?;
            
            Ok(Some(customer))
        } else {
            Ok(None)
        }
    }

    pub fn cost_center(&self, context: &AppState) -> FieldResult<CostCenter> {
        let service = context.service.lock().unwrap();
        
        let query = Query::select()
            .from(CostCenters::Table)
            .columns([
                CostCenters::Id,
                CostCenters::Name,
                CostCenters::Code,
                CostCenters::Description,
                CostCenters::State,
                CostCenters::CreatedAt,
                CostCenters::UpdatedAt,
            ])
            .and_where(Expr::col(CostCenters::Id).eq(self.cost_center_id.to_string()))
            .to_string(SqliteQueryBuilder);
            
        let cost_center = service.db_adapter.query_one::<CostCenter>(&query, vec![])?;
        
        Ok(cost_center)
    }

    pub fn items(&self, context: &AppState) -> FieldResult<Vec<SalesOrderItem>> {
        let service = context.service.lock().unwrap();
        
        let query = Query::select()
            .from(SalesOrderItems::Table)
            .columns([
                SalesOrderItems::Id,
                SalesOrderItems::OrderId,
                SalesOrderItems::ItemId,
                SalesOrderItems::ItemName,
                SalesOrderItems::Quantity,
                SalesOrderItems::Sku,
                SalesOrderItems::PriceAmount,
                SalesOrderItems::DiscAmount,
                SalesOrderItems::TaxableAmount,
                SalesOrderItems::TaxAmount,
                SalesOrderItems::TotalAmount,
                SalesOrderItems::CreatedAt,
                SalesOrderItems::UpdatedAt,
            ])
            .and_where(Expr::col(SalesOrderItems::OrderId).eq(self.id.to_string()))
            .to_string(SqliteQueryBuilder);
            
        let items = service.db_adapter.query_many::<SalesOrderItem>(&query, vec![])?;
        
        Ok(items)
    }

    pub fn charges(&self, context: &AppState) -> FieldResult<Vec<SalesOrderCharge>> {
        let service = context.service.lock().unwrap();
        
        let query = Query::select()
            .from(SalesOrderCharges::Table)
            .columns([
                SalesOrderCharges::Id,
                SalesOrderCharges::OrderId,
                SalesOrderCharges::ChargeTypeId,
                SalesOrderCharges::ChargeTypeName,
                SalesOrderCharges::Amount,
                SalesOrderCharges::TaxAmount,
                SalesOrderCharges::TaxGroupId,
                SalesOrderCharges::CreatedAt,
                SalesOrderCharges::UpdatedAt,
            ])
            .and_where(Expr::col(SalesOrderCharges::OrderId).eq(self.id.to_string()))
            .to_string(SqliteQueryBuilder);
            
        let charges = service.db_adapter.query_many::<SalesOrderCharge>(&query, vec![])?;
        
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
            .filter(|p| p.state == SalesOrderPaymentState::Completed)
            .map(|p| p.amount)
            .sum();

        Ok(total)
    }
}