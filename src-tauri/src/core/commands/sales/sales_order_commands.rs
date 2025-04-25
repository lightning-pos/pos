use chrono::Utc;
use sea_query::{Expr, Query};
use rand::Rng;
use uuid::Uuid;

use crate::{
    adapters::outgoing::database::DatabaseAdapter, core::{
        commands::{app_service::AppService, Command},
        models::sales::{
            sales_order_charge_model::{SalesOrderCharge, SalesOrderCharges},
            sales_order_item_model::{SalesOrderItem, SalesOrderItems},
            sales_order_model::{
                SalesOrder, SalesOrderNewInput, SalesOrderPaymentState, SalesOrderState, SalesOrders
            },
        },
        types::db_uuid::DbUuid,
    },
    error::{Error, Result}
};

// Helper function to generate readable ID (Example: ORD-YYYYMMDD-XXXX)
fn generate_readable_order_id() -> String {
    let now = Utc::now();
    let date_str = now.format("%Y%m%d").to_string();
    let random_part: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(4)
        .map(|c| c as char)
        .flat_map(char::to_uppercase)
        .collect();
    format!("ORD-{}-{}", date_str, random_part)
}

// Commands
pub struct CreateSalesOrderCommand {
    pub sales_order: SalesOrderNewInput,
    pub created_by_user_id: DbUuid,
}

pub struct VoidSalesOrderCommand {
    pub id: DbUuid,
    pub updated_by_user_id: DbUuid,
}

// Command Implementations
impl Command for CreateSalesOrderCommand {
    type Output = SalesOrder;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let db = &service.db_adapter;
        let now = Utc::now().naive_utc();
        let user_id = self.created_by_user_id;
        let order_id: DbUuid = Uuid::now_v7().into();

        let new_sales_order = SalesOrder {
            id: order_id,
            order_readable_id: generate_readable_order_id(),
            customer_id: self.sales_order.customer_id,
            customer_name: self.sales_order.customer_name.clone(),
            customer_phone_number: self.sales_order.customer_phone_number.clone(),
            billing_address: self.sales_order.billing_address.clone(),
            shipping_address: self.sales_order.shipping_address.clone(),
            order_date: self.sales_order.order_date,
            net_amount: self.sales_order.net_amount,
            disc_amount: self.sales_order.disc_amount,
            taxable_amount: self.sales_order.taxable_amount,
            tax_amount: self.sales_order.tax_amount,
            total_amount: self.sales_order.total_amount,
            order_state: SalesOrderState::Completed,
            payment_state: SalesOrderPaymentState::Pending,
            notes: self.sales_order.notes.clone(),
            channel_id: self.sales_order.channel_id,
            location_id: self.sales_order.location_id,
            cost_center_id: self.sales_order.cost_center_id,
            discount_id: self.sales_order.discount_id,
            created_by: user_id,
            updated_by: user_id,
            created_at: now,
            updated_at: now,
        };

        // Insert the sales order
        let mut insert_stmt = Query::insert();
        let insert_stmt = insert_stmt
            .into_table(SalesOrders::Table)
            .columns([
                SalesOrders::Id,
                SalesOrders::OrderReadableId,
                SalesOrders::OrderDate,
                SalesOrders::CustomerId,
                SalesOrders::CustomerName,
                SalesOrders::CustomerPhoneNumber,
                SalesOrders::BillingAddress,
                SalesOrders::ShippingAddress,
                SalesOrders::NetAmount,
                SalesOrders::DiscAmount,
                SalesOrders::TaxableAmount,
                SalesOrders::TaxAmount,
                SalesOrders::TotalAmount,
                SalesOrders::OrderState,
                SalesOrders::PaymentState,
                SalesOrders::Notes,
                SalesOrders::ChannelId,
                SalesOrders::LocationId,
                SalesOrders::CostCenterId,
                SalesOrders::CreatedBy,
                SalesOrders::UpdatedBy,
                SalesOrders::DiscountId,
                SalesOrders::CreatedAt,
                SalesOrders::UpdatedAt,
            ])
            .values_panic([
                new_sales_order.id.to_string().into(),
                new_sales_order.order_readable_id.clone().into(),
                new_sales_order.order_date.to_string().into(),
                match new_sales_order.customer_id {
                    Some(id) => id.to_string().into(),
                    None => sea_query::Value::String(None).into(),
                },
                match &new_sales_order.customer_name {
                    Some(name) => name.clone().into(),
                    None => sea_query::Value::String(None).into(),
                },
                match &new_sales_order.customer_phone_number {
                    Some(phone) => phone.clone().into(),
                    None => sea_query::Value::String(None).into(),
                },
                match &new_sales_order.billing_address {
                    Some(addr) => addr.clone().into(),
                    None => sea_query::Value::String(None).into(),
                },
                match &new_sales_order.shipping_address {
                    Some(addr) => addr.clone().into(),
                    None => sea_query::Value::String(None).into(),
                },
                new_sales_order.net_amount.to_string().into(),
                new_sales_order.disc_amount.to_string().into(),
                new_sales_order.taxable_amount.to_string().into(),
                new_sales_order.tax_amount.to_string().into(),
                new_sales_order.total_amount.to_string().into(),
                new_sales_order.order_state.to_string().into(),
                new_sales_order.payment_state.to_string().into(),
                match &new_sales_order.notes {
                    Some(notes) => notes.clone().into(),
                    None => sea_query::Value::String(None).into(),
                },
                new_sales_order.channel_id.to_string().into(),
                new_sales_order.location_id.to_string().into(),
                new_sales_order.cost_center_id.to_string().into(),
                new_sales_order.created_by.to_string().into(),
                new_sales_order.updated_by.to_string().into(),
                match new_sales_order.discount_id {
                    Some(id) => id.to_string().into(),
                    None => sea_query::Value::String(None).into(),
                },
                new_sales_order.created_at.to_string().into(),
                new_sales_order.updated_at.to_string().into(),
            ]);

        db.insert_one::<SalesOrder>(&insert_stmt).await?;

        // Insert order items
        for item in &self.sales_order.items {
            let item_id: DbUuid = Uuid::now_v7().into();

            let mut item_insert_stmt = Query::insert();
            let item_insert_stmt = item_insert_stmt
                .into_table(SalesOrderItems::Table)
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
                .values_panic([
                    item_id.to_string().into(),
                    order_id.to_string().into(),
                    match item.item_id {
                        Some(id) => id.to_string().into(),
                        None => sea_query::Value::String(None).into(),
                    },
                    item.item_name.clone().into(),
                    item.quantity.into(),
                    match &item.sku {
                        Some(sku) => sku.clone().into(),
                        None => sea_query::Value::String(None).into(),
                    },
                    item.price_amount.to_string().into(),
                    item.disc_amount.to_string().into(),
                    item.taxable_amount.to_string().into(),
                    item.tax_amount.to_string().into(),
                    item.total_amount.to_string().into(),
                    now.to_string().into(),
                    now.to_string().into(),
                ]);

            db.insert_one::<SalesOrderItem>(&item_insert_stmt).await?;
        }

        // Insert order charges if any
        if let Some(charges_input) = &self.sales_order.charges {
            for charge in charges_input {
                let charge_id: DbUuid = Uuid::now_v7().into();

                let mut charge_insert_stmt = Query::insert();
                let charge_insert_stmt = charge_insert_stmt
                    .into_table(SalesOrderCharges::Table)
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
                    .values_panic([
                        charge_id.to_string().into(),
                        order_id.to_string().into(),
                        charge.charge_type_id.to_string().into(),
                        charge.charge_type_name.clone().into(),
                        charge.amount.to_string().into(),
                        charge.tax_amount.to_string().into(),
                        match charge.tax_group_id {
                            Some(id) => id.to_string().into(),
                            None => sea_query::Value::String(None).into(),
                        },
                        now.to_string().into(),
                        now.to_string().into(),
                    ]);

                db.insert_one::<SalesOrderCharge>(&charge_insert_stmt).await?;
            }
        }

        // Retrieve the created order
        let mut select_stmt = Query::select();
        let select_stmt = select_stmt
            .from(SalesOrders::Table)
            .columns([
                SalesOrders::Id,
                SalesOrders::OrderReadableId,
                SalesOrders::OrderDate,
                SalesOrders::CustomerId,
                SalesOrders::CustomerName,
                SalesOrders::CustomerPhoneNumber,
                SalesOrders::BillingAddress,
                SalesOrders::ShippingAddress,
                SalesOrders::NetAmount,
                SalesOrders::DiscAmount,
                SalesOrders::TaxableAmount,
                SalesOrders::TaxAmount,
                SalesOrders::TotalAmount,
                SalesOrders::OrderState,
                SalesOrders::PaymentState,
                SalesOrders::Notes,
                SalesOrders::ChannelId,
                SalesOrders::LocationId,
                SalesOrders::CostCenterId,
                SalesOrders::CreatedBy,
                SalesOrders::UpdatedBy,
                SalesOrders::DiscountId,
                SalesOrders::CreatedAt,
                SalesOrders::UpdatedAt,
            ])
            .and_where(Expr::col(SalesOrders::Id).eq(order_id.to_string()));

        let created_order = db.query_one::<SalesOrder>(&select_stmt).await?;

        Ok(created_order)
    }
}

impl Command for VoidSalesOrderCommand {
    type Output = SalesOrder;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let db = &service.db_adapter;
        let now = Utc::now().naive_utc();
        let user_id = self.updated_by_user_id;

        // Check if the order exists and is in Completed state
        let mut check_stmt = Query::select();
        let check_stmt = check_stmt
            .from(SalesOrders::Table)
            .columns([
                SalesOrders::Id,
                SalesOrders::OrderReadableId,
                SalesOrders::OrderDate,
                SalesOrders::CustomerId,
                SalesOrders::CustomerName,
                SalesOrders::CustomerPhoneNumber,
                SalesOrders::BillingAddress,
                SalesOrders::ShippingAddress,
                SalesOrders::NetAmount,
                SalesOrders::DiscAmount,
                SalesOrders::TaxableAmount,
                SalesOrders::TaxAmount,
                SalesOrders::TotalAmount,
                SalesOrders::OrderState,
                SalesOrders::PaymentState,
                SalesOrders::Notes,
                SalesOrders::ChannelId,
                SalesOrders::LocationId,
                SalesOrders::CostCenterId,
                SalesOrders::CreatedBy,
                SalesOrders::UpdatedBy,
                SalesOrders::DiscountId,
                SalesOrders::CreatedAt,
                SalesOrders::UpdatedAt,
            ])
            .and_where(Expr::col(SalesOrders::Id).eq(self.id.to_string()))
            .and_where(Expr::col(SalesOrders::OrderState).eq(SalesOrderState::Completed.to_string()));

        let order = db.query_optional::<SalesOrder>(&check_stmt).await?;
        if order.is_none() {
            return Err(Error::NotFoundError);
        }

        // Update the order state
        let mut update_stmt = Query::update();
        let update_stmt = update_stmt
            .table(SalesOrders::Table)
            .value(SalesOrders::OrderState, SalesOrderState::Cancelled.to_string())
            .value(SalesOrders::PaymentState, SalesOrderPaymentState::Voided.to_string())
            .value(SalesOrders::UpdatedBy, user_id.to_string())
            .value(SalesOrders::UpdatedAt, now.to_string())
            .and_where(Expr::col(SalesOrders::Id).eq(self.id.to_string()));

        db.update_one::<SalesOrder>(&update_stmt).await?;

        // Retrieve the updated order
        let mut select_stmt = Query::select();
        let select_stmt = select_stmt
            .from(SalesOrders::Table)
            .columns([
                SalesOrders::Id,
                SalesOrders::OrderReadableId,
                SalesOrders::OrderDate,
                SalesOrders::CustomerId,
                SalesOrders::CustomerName,
                SalesOrders::CustomerPhoneNumber,
                SalesOrders::BillingAddress,
                SalesOrders::ShippingAddress,
                SalesOrders::NetAmount,
                SalesOrders::DiscAmount,
                SalesOrders::TaxableAmount,
                SalesOrders::TaxAmount,
                SalesOrders::TotalAmount,
                SalesOrders::OrderState,
                SalesOrders::PaymentState,
                SalesOrders::Notes,
                SalesOrders::ChannelId,
                SalesOrders::LocationId,
                SalesOrders::CostCenterId,
                SalesOrders::CreatedBy,
                SalesOrders::UpdatedBy,
                SalesOrders::DiscountId,
                SalesOrders::CreatedAt,
                SalesOrders::UpdatedAt,
            ])
            .and_where(Expr::col(SalesOrders::Id).eq(self.id.to_string()));

        let updated_order = db.query_one::<SalesOrder>(&select_stmt).await?;

        Ok(updated_order)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::commands::sales::sales_charge_type_commands::CreateSalesChargeTypeCommand;
    use crate::core::commands::tests::setup_service;
    use tokio;
    use crate::{
        core::{
            commands::{
                auth::user_commands::AddUserCommand,
                common::{
                    channel_commands::CreateChannelCommand,
                    location_commands::CreateLocationCommand,
                },
                finance::cost_center_commands::CreateCostCenterCommand,
            },
            models::{
                auth::user_model::UserNewInput,
                common::{
                    channel_model::{Channel, ChannelNewInput},
                    location_model::{Location, LocationNewInput},
                },
                finance::cost_center_model::{CostCenter, CostCenterNewInput, CostCenterState},
                sales::{
                    sales_charge_type_model::{SalesChargeType, SalesChargeTypeNewInput},
                    sales_order_charge_model::*,
                    sales_order_item_model::*,
                    sales_order_model::*,
                },
            },
        },
        error::Error,
    };
    use rand::Rng;
    use uuid::Uuid;

    use super::*;

    async fn create_test_cost_center(service: &mut AppService) -> CostCenter {
        let command = CreateCostCenterCommand {
            cost_center: CostCenterNewInput {
                name: "Test Cost Center".to_string(),
                code: format!("TCC{:03}", rand::thread_rng().gen_range(1..999)),
                description: None,
                state: Some(CostCenterState::Active),
            },
        };
        command.exec(service).await.unwrap()
    }

    async fn create_test_user(service: &mut AppService) -> DbUuid {
        let random_suffix = rand::thread_rng().gen_range(1000..9999).to_string();
        let command = AddUserCommand {
            user: UserNewInput {
                username: format!("testuser{}", random_suffix),
                pin: "1234".to_string(),
                full_name: format!("Test User {}", random_suffix),
            },
        };
        command.exec(service).await.unwrap().id
    }

    async fn create_test_channel(service: &mut AppService) -> Channel {
        let command = CreateChannelCommand {
            channel: ChannelNewInput {
                name: format!("Test Channel {}", rand::thread_rng().gen_range(1..999)),
                description: None,
                is_active: Some(true),
            },
        };
        command.exec(service).await.unwrap()
    }

    async fn create_test_location(service: &mut AppService) -> Location {
        let command = CreateLocationCommand {
            location: LocationNewInput {
                name: format!("Test Location {}", rand::thread_rng().gen_range(1..999)),
                description: None,
                address: None,
                is_active: Some(true),
            },
        };
        command.exec(service).await.unwrap()
    }

    async fn create_test_charge_type(service: &mut AppService, name: &str) -> SalesChargeType {
        let cmd = CreateSalesChargeTypeCommand {
            charge_type: SalesChargeTypeNewInput {
                name: name.to_string(),
                description: None,
            },
        };
        cmd.exec(service).await.unwrap()
    }

    #[tokio::test]
    async fn test_create_sales_order() {
        let mut service = setup_service().await;
        let now = Utc::now().naive_utc();
        let cost_center = create_test_cost_center(&mut service).await;
        let user_id = create_test_user(&mut service).await;
        let channel = create_test_channel(&mut service).await;
        let location = create_test_location(&mut service).await;

        // Print IDs for debugging
        println!("Test IDs: user_id={:?}, channel_id={:?}, location_id={:?}, cost_center_id={:?}",
            user_id, channel.id, location.id, cost_center.id);

        // Don't use customer_id for now to avoid foreign key constraint issues
        let customer_id = None;
        println!("Using customer_id={:?}", customer_id);

        let input = SalesOrderNewInput {
            customer_id,
            customer_name: Some("John Doe".to_string()),
            customer_phone_number: Some("+1234567890".to_string()),
            billing_address: Some("123 Billing St".to_string()),
            shipping_address: Some("456 Shipping Ave".to_string()),
            order_date: now,
            net_amount: 1000.into(),
            disc_amount: 100.into(),
            taxable_amount: 900.into(),
            tax_amount: 90.into(),
            total_amount: 990.into(),
            notes: Some("Test order notes".to_string()),
            channel_id: channel.id,
            location_id: location.id,
            cost_center_id: cost_center.id,
            discount_id: None,
            items: vec![
                SalesOrderItemInput {
                    item_id: None, // Don't use item_id to avoid foreign key constraint issues
                    item_name: "Item 1".to_string(),
                    quantity: 2,
                    sku: Some("SKU001".to_string()),
                    price_amount: 500.into(),
                    disc_amount: 50.into(),
                    taxable_amount: 450.into(),
                    tax_amount: 45.into(),
                    total_amount: 495.into(),
                },
                SalesOrderItemInput {
                    item_id: None, // Don't use item_id to avoid foreign key constraint issues
                    item_name: "Item 2".to_string(),
                    quantity: 1,
                    sku: None,
                    price_amount: 100.into(),
                    disc_amount: 0.into(),
                    taxable_amount: 100.into(),
                    tax_amount: 10.into(),
                    total_amount: 110.into(),
                },
            ],
            charges: None,
        };

        let cmd = CreateSalesOrderCommand {
            sales_order: input,
            created_by_user_id: user_id,
        };

        let result = match cmd.exec(&mut service).await {
            Ok(r) => r,
            Err(e) => {
                println!("Error creating sales order: {:?}", e);
                panic!("Failed to create sales order: {}", e);
            }
        };
        assert_eq!(result.customer_name, Some("John Doe".to_string()));
        assert_eq!(result.order_state, SalesOrderState::Completed);
        assert_eq!(result.payment_state, SalesOrderPaymentState::Pending);
        assert_eq!(result.cost_center_id, cost_center.id);
        assert_eq!(result.created_by, user_id);
        assert_eq!(result.updated_by, user_id);
        assert!(!result.order_readable_id.is_empty());

        // Query items using SeaQuery
        let mut items_query = Query::select();
        let items_stmt = items_query
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
            .and_where(Expr::col(SalesOrderItems::OrderId).eq(result.id.to_string()));

        let inserted_items = service.db_adapter.query_many::<SalesOrderItem>(&items_stmt).await.unwrap();
        assert_eq!(inserted_items.len(), 2);
        assert!(inserted_items.iter().any(|item| item.item_name == "Item 1"));
        assert!(inserted_items.iter().any(|item| item.item_name == "Item 2"));

        // Query charges using SeaQuery
        let mut charges_query = Query::select();
        let charges_stmt = charges_query
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
            .and_where(Expr::col(SalesOrderCharges::OrderId).eq(result.id.to_string()));

        let inserted_charges = service.db_adapter.query_many::<SalesOrderCharge>(&charges_stmt).await.unwrap();
        assert!(inserted_charges.is_empty());
    }

    #[tokio::test]
    async fn test_create_sales_order_with_charges() {
        let mut service = setup_service().await;
        let now = Utc::now().naive_utc();
        let cost_center = create_test_cost_center(&mut service).await;
        let user_id = create_test_user(&mut service).await;
        let channel = create_test_channel(&mut service).await;
        let location = create_test_location(&mut service).await;
        let charge_type1 = create_test_charge_type(&mut service, "Service Charge").await;
        let charge_type2 = create_test_charge_type(&mut service, "Delivery Fee").await;

        let input = SalesOrderNewInput {
            customer_id: None,
            customer_name: None,
            customer_phone_number: None,
            billing_address: None,
            shipping_address: None,
            order_date: now,
            net_amount: 500.into(),
            disc_amount: 0.into(),
            taxable_amount: 500.into(),
            tax_amount: 50.into(),
            total_amount: 550.into(),
            notes: None,
            channel_id: channel.id,
            location_id: location.id,
            cost_center_id: cost_center.id,
            discount_id: None,
            items: vec![SalesOrderItemInput {
                item_id: None,
                item_name: "Custom Item".to_string(),
                quantity: 1,
                sku: None,
                price_amount: 500.into(),
                disc_amount: 0.into(),
                taxable_amount: 500.into(),
                tax_amount: 50.into(),
                total_amount: 550.into(),
            }],
            charges: Some(vec![
                SalesOrderChargeNewInput {
                    charge_type_id: charge_type1.id,
                    charge_type_name: charge_type1.name.clone(),
                    amount: 50.into(),
                    tax_amount: 5.into(),
                    tax_group_id: None,
                },
                SalesOrderChargeNewInput {
                    charge_type_id: charge_type2.id,
                    charge_type_name: charge_type2.name.clone(),
                    amount: 100.into(),
                    tax_amount: 0.into(),
                    tax_group_id: None,
                },
            ]),
        };

        let cmd = CreateSalesOrderCommand {
            sales_order: input,
            created_by_user_id: user_id,
        };
        let result = cmd.exec(&mut service).await.unwrap();

        // Query charges using SeaQuery
        let mut charges_query = Query::select();
        let charges_stmt = charges_query
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
            .and_where(Expr::col(SalesOrderCharges::OrderId).eq(result.id.to_string()));

        let inserted_charges = service.db_adapter.query_many::<SalesOrderCharge>(&charges_stmt).await.unwrap();
        assert_eq!(inserted_charges.len(), 2);

        // Find the charges by name
        let service_charge = inserted_charges.iter().find(|c| c.charge_type_name == "Service Charge").unwrap();
        let delivery_fee = inserted_charges.iter().find(|c| c.charge_type_name == "Delivery Fee").unwrap();

        assert_eq!(service_charge.amount, 50.into());
        assert_eq!(delivery_fee.amount, 100.into());

        assert_eq!(result.total_amount, 550.into());
    }

    #[tokio::test]
    async fn test_void_sales_order() {
        let mut service = setup_service().await;
        let now = Utc::now().naive_utc();
        let cost_center = create_test_cost_center(&mut service).await;
        let user_id = create_test_user(&mut service).await;
        let channel = create_test_channel(&mut service).await;
        let location = create_test_location(&mut service).await;

        let input = SalesOrderNewInput {
            customer_id: None,
            customer_name: None,
            customer_phone_number: None,
            billing_address: None,
            shipping_address: None,
            order_date: now,
            net_amount: 1000.into(),
            disc_amount: 0.into(),
            taxable_amount: 1000.into(),
            tax_amount: 100.into(),
            total_amount: 1100.into(),
            notes: None,
            channel_id: channel.id,
            location_id: location.id,
            cost_center_id: cost_center.id,
            discount_id: None,
            items: vec![SalesOrderItemInput {
                item_id: None, // Don't use item_id to avoid foreign key constraint issues
                item_name: "Item 1".to_string(),
                quantity: 1,
                sku: None,
                price_amount: 1000.into(),
                disc_amount: 0.into(),
                taxable_amount: 1000.into(),
                tax_amount: 100.into(),
                total_amount: 1100.into(),
            }],
            charges: None,
        };

        let create_cmd = CreateSalesOrderCommand {
            sales_order: input,
            created_by_user_id: user_id,
        };
        let created = create_cmd.exec(&mut service).await.unwrap();

        let void_cmd = VoidSalesOrderCommand {
            id: created.id,
            updated_by_user_id: user_id,
        };
        let voided = void_cmd.exec(&mut service).await.unwrap();

        assert_eq!(voided.order_state, SalesOrderState::Cancelled);
        assert_eq!(voided.payment_state, SalesOrderPaymentState::Voided);
        assert_eq!(voided.updated_by, user_id);
    }

    #[tokio::test]
    async fn test_void_already_cancelled_order() {
        let mut service = setup_service().await;
        let now = Utc::now().naive_utc();
        let cost_center = create_test_cost_center(&mut service).await;
        let user_id = create_test_user(&mut service).await;
        let channel = create_test_channel(&mut service).await;
        let location = create_test_location(&mut service).await;

        let input = SalesOrderNewInput {
            customer_id: None,
            customer_name: None,
            customer_phone_number: None,
            billing_address: None,
            shipping_address: None,
            order_date: now,
            net_amount: 1000.into(),
            disc_amount: 0.into(),
            taxable_amount: 1000.into(),
            tax_amount: 100.into(),
            total_amount: 1100.into(),
            notes: None,
            channel_id: channel.id,
            location_id: location.id,
            cost_center_id: cost_center.id,
            discount_id: None,
            items: vec![SalesOrderItemInput {
                item_id: None, // Don't use item_id to avoid foreign key constraint issues
                item_name: "Item 1".to_string(),
                quantity: 1,
                sku: None,
                price_amount: 1000.into(),
                disc_amount: 0.into(),
                taxable_amount: 1000.into(),
                tax_amount: 100.into(),
                total_amount: 1100.into(),
            }],
            charges: None,
        };

        let create_cmd = CreateSalesOrderCommand {
            sales_order: input,
            created_by_user_id: user_id,
        };
        let created = create_cmd.exec(&mut service).await.unwrap();

        let void_cmd = VoidSalesOrderCommand {
            id: created.id,
            updated_by_user_id: user_id,
        };
        let _ = void_cmd.exec(&mut service).await.unwrap();

        let result = void_cmd.exec(&mut service).await;
        assert!(matches!(result, Err(Error::NotFoundError)));
    }

    #[tokio::test]
    async fn test_void_non_existent_order() {
        let mut service = setup_service().await;
        let user_id = create_test_user(&mut service).await;

        let cmd = VoidSalesOrderCommand {
            id: Uuid::now_v7().into(),
            updated_by_user_id: user_id,
        };

        let result = cmd.exec(&mut service).await;
        assert!(matches!(result, Err(Error::NotFoundError)));
    }
}
