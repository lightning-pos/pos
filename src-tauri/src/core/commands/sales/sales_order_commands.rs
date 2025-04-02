use chrono::Utc;
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use rand::Rng;
use uuid::Uuid;

use crate::{
    core::{
        commands::{app_service::AppService, Command},
        models::sales::{
            sales_order_charge_model::{SalesOrderCharge, SalesOrderChargeNewInput},
            sales_order_item_model::{SalesOrderItem, SalesOrderItemInput},
            sales_order_model::{
                SalesOrder, SalesOrderNewInput, SalesOrderPaymentState, SalesOrderState,
            },
        },
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
    schema::{sales_order_charges, sales_order_items, sales_orders},
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

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let now = Utc::now().naive_utc();
            let user_id = self.created_by_user_id;

            let new_sales_order = SalesOrder {
                id: Uuid::now_v7().into(),
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

            let order = diesel::insert_into(sales_orders::table)
                .values(&new_sales_order)
                .returning(SalesOrder::as_returning())
                .get_result(conn)?;

            let order_items: Vec<SalesOrderItem> = self
                .sales_order
                .items
                .iter()
                .map(|item| SalesOrderItem {
                    id: Uuid::now_v7().into(),
                    order_id: order.id,
                    item_id: item.item_id,
                    item_name: item.item_name.clone(),
                    quantity: item.quantity,
                    sku: item.sku.clone(),
                    price_amount: item.price_amount,
                    disc_amount: item.disc_amount,
                    taxable_amount: item.taxable_amount,
                    tax_amount: item.tax_amount,
                    total_amount: item.total_amount,
                    created_at: now,
                    updated_at: now,
                })
                .collect();

            diesel::insert_into(sales_order_items::table)
                .values(&order_items)
                .execute(conn)?;

            if let Some(charges_input) = &self.sales_order.charges {
                let order_charges: Vec<SalesOrderCharge> = charges_input
                    .iter()
                    .map(|charge| SalesOrderCharge {
                        id: Uuid::now_v7().into(),
                        order_id: order.id,
                        charge_type_id: charge.charge_type_id,
                        charge_type_name: charge.charge_type_name.clone(),
                        amount: charge.amount,
                        tax_amount: charge.tax_amount,
                        tax_group_id: charge.tax_group_id,
                        created_at: now,
                        updated_at: now,
                    })
                    .collect();

                diesel::insert_into(sales_order_charges::table)
                    .values(&order_charges)
                    .execute(conn)?;
            }

            Ok(order)
        })
    }
}

impl Command for VoidSalesOrderCommand {
    type Output = SalesOrder;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let now = Utc::now().naive_utc();
            let user_id = self.updated_by_user_id;

            let order = sales_orders::table
                .find(self.id)
                .filter(sales_orders::order_state.eq(SalesOrderState::Completed))
                .select(SalesOrder::as_select())
                .first::<SalesOrder>(conn)
                .map_err(|_| Error::NotFoundError)?;

            let res = diesel::update(&order)
                .set((
                    sales_orders::order_state.eq(SalesOrderState::Cancelled),
                    sales_orders::payment_state.eq(SalesOrderPaymentState::Voided),
                    sales_orders::updated_by.eq(user_id),
                    sales_orders::updated_at.eq(now),
                ))
                .returning(SalesOrder::as_returning())
                .get_result(conn)?;

            Ok(res)
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::core::commands::sales::sales_charge_type_commands::CreateSalesChargeTypeCommand;
    use crate::{
        core::{
            commands::finance::cost_center_commands::CreateCostCenterCommand,
            models::{
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

    fn create_test_cost_center(service: &mut AppService) -> CostCenter {
        let command = CreateCostCenterCommand {
            cost_center: CostCenterNewInput {
                name: "Test Cost Center".to_string(),
                code: format!("TCC{:03}", rand::thread_rng().gen_range(1..999)),
                description: None,
                state: Some(CostCenterState::Active),
            },
        };
        command.exec(service).unwrap()
    }

    fn test_user_id() -> DbUuid {
        Uuid::new_v4().into()
    }

    fn test_channel_id() -> DbUuid {
        Uuid::new_v4().into()
    }

    fn test_location_id() -> DbUuid {
        Uuid::new_v4().into()
    }

    fn create_test_charge_type(service: &mut AppService, name: &str) -> SalesChargeType {
        let cmd = CreateSalesChargeTypeCommand {
            charge_type: SalesChargeTypeNewInput {
                name: name.to_string(),
                description: None,
            },
        };
        cmd.exec(service).unwrap()
    }

    #[test]
    fn test_create_sales_order() {
        let mut service = AppService::new(":memory:");
        let now = Utc::now().naive_utc();
        let cost_center = create_test_cost_center(&mut service);
        let user_id = test_user_id();
        let channel_id = test_channel_id();
        let location_id = test_location_id();

        let input = SalesOrderNewInput {
            customer_id: Some(Uuid::now_v7().into()),
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
            channel_id,
            location_id,
            cost_center_id: cost_center.id,
            discount_id: None,
            items: vec![
                SalesOrderItemInput {
                    item_id: Some(Uuid::now_v7().into()),
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
                    item_id: Some(Uuid::now_v7().into()),
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

        let result = cmd.exec(&mut service).unwrap();
        assert_eq!(result.customer_name, Some("John Doe".to_string()));
        assert_eq!(result.order_state, SalesOrderState::Completed);
        assert_eq!(result.payment_state, SalesOrderPaymentState::Pending);
        assert_eq!(result.cost_center_id, cost_center.id);
        assert_eq!(result.created_by, user_id);
        assert_eq!(result.updated_by, user_id);
        assert!(!result.order_readable_id.is_empty());

        let inserted_items = sales_order_items::table
            .filter(sales_order_items::order_id.eq(result.id))
            .select(SalesOrderItem::as_select())
            .load::<SalesOrderItem>(&mut service.conn)
            .unwrap();
        assert_eq!(inserted_items.len(), 2);
        assert_eq!(inserted_items[0].item_name, "Item 1");
        assert_eq!(inserted_items[1].item_name, "Item 2");

        let inserted_charges = sales_order_charges::table
            .filter(sales_order_charges::order_id.eq(result.id))
            .select(SalesOrderCharge::as_select())
            .load::<SalesOrderCharge>(&mut service.conn)
            .unwrap();
        assert!(inserted_charges.is_empty());
    }

    #[test]
    fn test_create_sales_order_with_charges() {
        let mut service = AppService::new(":memory:");
        let now = Utc::now().naive_utc();
        let cost_center = create_test_cost_center(&mut service);
        let user_id = test_user_id();
        let channel_id = test_channel_id();
        let location_id = test_location_id();
        let charge_type1 = create_test_charge_type(&mut service, "Service Charge");
        let charge_type2 = create_test_charge_type(&mut service, "Delivery Fee");

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
            channel_id,
            location_id,
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
        let result = cmd.exec(&mut service).unwrap();

        let inserted_charges = sales_order_charges::table
            .filter(sales_order_charges::order_id.eq(result.id))
            .order(sales_order_charges::created_at.asc())
            .select(SalesOrderCharge::as_select())
            .load::<SalesOrderCharge>(&mut service.conn)
            .unwrap();
        assert_eq!(inserted_charges.len(), 2);
        assert_eq!(inserted_charges[0].charge_type_name, "Service Charge");
        assert_eq!(inserted_charges[0].amount, 50.into());
        assert_eq!(inserted_charges[1].charge_type_name, "Delivery Fee");
        assert_eq!(inserted_charges[1].amount, 100.into());

        assert_eq!(result.total_amount, 550.into());
    }

    #[test]
    fn test_void_sales_order() {
        let mut service = AppService::new(":memory:");
        let now = Utc::now().naive_utc();
        let cost_center = create_test_cost_center(&mut service);
        let user_id = test_user_id();
        let channel_id = test_channel_id();
        let location_id = test_location_id();

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
            channel_id,
            location_id,
            cost_center_id: cost_center.id,
            discount_id: None,
            items: vec![SalesOrderItemInput {
                item_id: Some(Uuid::now_v7().into()),
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
        let created = create_cmd.exec(&mut service).unwrap();

        let void_cmd = VoidSalesOrderCommand {
            id: created.id,
            updated_by_user_id: user_id,
        };
        let voided = void_cmd.exec(&mut service).unwrap();

        assert_eq!(voided.order_state, SalesOrderState::Cancelled);
        assert_eq!(voided.payment_state, SalesOrderPaymentState::Voided);
        assert_eq!(voided.updated_by, user_id);
    }

    #[test]
    fn test_void_already_cancelled_order() {
        let mut service = AppService::new(":memory:");
        let now = Utc::now().naive_utc();
        let cost_center = create_test_cost_center(&mut service);
        let user_id = test_user_id();
        let channel_id = test_channel_id();
        let location_id = test_location_id();

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
            channel_id,
            location_id,
            cost_center_id: cost_center.id,
            discount_id: None,
            items: vec![SalesOrderItemInput {
                item_id: Some(Uuid::now_v7().into()),
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
        let created = create_cmd.exec(&mut service).unwrap();

        let void_cmd = VoidSalesOrderCommand {
            id: created.id,
            updated_by_user_id: user_id,
        };
        let _ = void_cmd.exec(&mut service).unwrap();

        let result = void_cmd.exec(&mut service);
        assert!(matches!(result, Err(Error::NotFoundError)));
    }

    #[test]
    fn test_void_non_existent_order() {
        let mut service = AppService::new(":memory:");
        let user_id = test_user_id();

        let cmd = VoidSalesOrderCommand {
            id: Uuid::now_v7().into(),
            updated_by_user_id: user_id,
        };

        let result = cmd.exec(&mut service);
        assert!(matches!(result, Err(Error::NotFoundError)));
    }
}
