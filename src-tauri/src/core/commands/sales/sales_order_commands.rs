use chrono::Utc;
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{
    core::{
        commands::{app_service::AppService, Command},
        models::sales::{
            sales_order_item_model::SalesOrderItem,
            sales_order_model::{SalesOrder, SalesOrderNewInput, SalesOrderState},
        },
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
    schema::{sales_order_items, sales_orders},
};

// Commands
pub struct CreateSalesOrderCommand {
    pub sales_order: SalesOrderNewInput,
}

pub struct VoidSalesOrderCommand {
    pub id: DbUuid,
}

// Command Implementations
impl Command for CreateSalesOrderCommand {
    type Output = SalesOrder;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let now = Utc::now().naive_utc();
            let new_sales_order = SalesOrder {
                id: Uuid::now_v7().into(),
                customer_id: self.sales_order.customer_id,
                customer_name: self.sales_order.customer_name.clone(),
                customer_phone_number: self.sales_order.customer_phone_number.clone(),
                order_date: self.sales_order.order_date,
                net_amount: self.sales_order.net_amount,
                disc_amount: self.sales_order.disc_amount,
                taxable_amount: self.sales_order.taxable_amount,
                tax_amount: self.sales_order.tax_amount,
                total_amount: self.sales_order.total_amount,
                state: SalesOrderState::Completed, // Orders are created in Completed state from cart
                cost_center_id: self.sales_order.cost_center_id,
                created_at: now,
                updated_at: now,
            };

            // Insert the order first
            let order = diesel::insert_into(sales_orders::table)
                .values(&new_sales_order)
                .returning(SalesOrder::as_returning())
                .get_result(conn)?;

            // Then insert all order items
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
                    price_amount: item.price_amount,
                    tax_amount: item.tax_amount,
                    total_amount: item.total_amount,
                    created_at: now,
                    updated_at: now,
                })
                .collect();

            diesel::insert_into(sales_order_items::table)
                .values(&order_items)
                .execute(conn)?;

            Ok(order)
        })
    }
}

impl Command for VoidSalesOrderCommand {
    type Output = SalesOrder;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let now = Utc::now().naive_utc();

            // Check if the order exists and can be voided
            let _order = sales_orders::table
                .find(self.id)
                .filter(sales_orders::state.eq(SalesOrderState::Completed)) // Can only void completed orders
                .first::<SalesOrder>(conn)
                .map_err(|_| Error::NotFoundError)?;

            // Update order state
            let res = diesel::update(sales_orders::table)
                .filter(sales_orders::id.eq(self.id))
                .set((
                    sales_orders::state.eq(SalesOrderState::Cancelled),
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
    use crate::core::models::finance::cost_center_model::CostCenter;
    use crate::core::models::sales::sales_order_item_model::SalesOrderItemInput;

    use super::*;

    fn create_test_cost_center(service: &mut AppService) -> CostCenter {
        use crate::core::{
            commands::{finance::cost_center_commands::CreateCostCenterCommand, Command},
            models::finance::cost_center_model::{CostCenterNewInput, CostCenterState},
        };

        let command = CreateCostCenterCommand {
            cost_center: CostCenterNewInput {
                name: "Test Cost Center".to_string(),
                code: "TCC001".to_string(),
                description: None,
                state: Some(CostCenterState::Active),
            },
        };
        command.exec(service).unwrap()
    }

    #[test]
    fn test_create_sales_order() {
        let mut service = AppService::new(":memory:");
        let now = Utc::now().naive_utc();
        let cost_center = create_test_cost_center(&mut service);

        let input = SalesOrderNewInput {
            customer_id: Uuid::now_v7().into(),
            customer_name: "John Doe".to_string(),
            customer_phone_number: "+1234567890".to_string(),
            order_date: now,
            net_amount: 1000.into(),
            disc_amount: 100.into(),
            taxable_amount: 900.into(),
            tax_amount: 90.into(),
            total_amount: 990.into(),
            state: SalesOrderState::Completed,
            cost_center_id: cost_center.id,
            items: vec![
                SalesOrderItemInput {
                    item_id: Some(Uuid::now_v7().into()),
                    item_name: "Item 1".to_string(),
                    quantity: 2,
                    price_amount: 500.into(),
                    tax_amount: 50.into(),
                    total_amount: 1000.into(),
                },
                SalesOrderItemInput {
                    item_id: Some(Uuid::now_v7().into()),
                    item_name: "Item 2".to_string(),
                    quantity: 1,
                    price_amount: 100.into(),
                    tax_amount: 40.into(),
                    total_amount: 140.into(),
                },
            ],
        };

        let cmd = CreateSalesOrderCommand { sales_order: input };

        let result = cmd.exec(&mut service).unwrap();
        assert_eq!(result.customer_name, "John Doe");
        assert_eq!(result.state, SalesOrderState::Completed);
        assert_eq!(result.cost_center_id, cost_center.id);
    }

    #[test]
    fn test_create_sales_order_with_cost_center() {
        let mut service = AppService::new(":memory:");
        let now = Utc::now().naive_utc();
        let cost_center = create_test_cost_center(&mut service);

        let input = SalesOrderNewInput {
            customer_id: Uuid::now_v7().into(),
            customer_name: "Jane Doe".to_string(),
            customer_phone_number: "+1987654321".to_string(),
            order_date: now,
            net_amount: 2000.into(),
            disc_amount: 200.into(),
            taxable_amount: 1800.into(),
            tax_amount: 180.into(),
            total_amount: 1980.into(),
            state: SalesOrderState::Completed,
            cost_center_id: cost_center.id,
            items: vec![SalesOrderItemInput {
                item_id: Some(Uuid::now_v7().into()),
                item_name: "Item 3".to_string(),
                quantity: 2,
                price_amount: 1000.into(),
                tax_amount: 100.into(),
                total_amount: 2000.into(),
            }],
        };

        let cmd = CreateSalesOrderCommand { sales_order: input };

        let result = cmd.exec(&mut service).unwrap();
        assert_eq!(result.customer_name, "Jane Doe");
        assert_eq!(result.cost_center_id, cost_center.id);
    }

    #[test]
    fn test_create_sales_order_with_no_item_id() {
        let mut service = AppService::new(":memory:");
        let now = Utc::now().naive_utc();
        let cost_center = create_test_cost_center(&mut service);

        let input = SalesOrderNewInput {
            customer_id: Uuid::now_v7().into(),
            customer_name: "Custom Order".to_string(),
            customer_phone_number: "+1234567890".to_string(),
            order_date: now,
            net_amount: 500.into(),
            disc_amount: 0.into(),
            taxable_amount: 500.into(),
            tax_amount: 50.into(),
            total_amount: 550.into(),
            state: SalesOrderState::Completed,
            cost_center_id: cost_center.id,
            items: vec![SalesOrderItemInput {
                item_id: None, // No item ID
                item_name: "Custom Item".to_string(),
                quantity: 1,
                price_amount: 500.into(),
                tax_amount: 50.into(),
                total_amount: 550.into(),
            }],
        };

        let cmd = CreateSalesOrderCommand { sales_order: input };

        let result = cmd.exec(&mut service).unwrap();
        assert_eq!(result.customer_name, "Custom Order");
        assert_eq!(result.cost_center_id, cost_center.id);
        // We could query for order items here, but that would require additional schema setup
    }

    #[test]
    fn test_void_sales_order() {
        let mut service = AppService::new(":memory:");
        let now = Utc::now().naive_utc();
        let cost_center = create_test_cost_center(&mut service);

        // First create a sales order
        let input = SalesOrderNewInput {
            customer_id: Uuid::now_v7().into(),
            customer_name: "John Doe".to_string(),
            customer_phone_number: "+1234567890".to_string(),
            order_date: now,
            net_amount: 1000.into(),
            disc_amount: 100.into(),
            taxable_amount: 900.into(),
            tax_amount: 90.into(),
            total_amount: 990.into(),
            state: SalesOrderState::Completed,
            cost_center_id: cost_center.id,
            items: vec![SalesOrderItemInput {
                item_id: Some(Uuid::now_v7().into()),
                item_name: "Item 1".to_string(),
                quantity: 2,
                price_amount: 500.into(),
                tax_amount: 50.into(),
                total_amount: 1000.into(),
            }],
        };

        let create_cmd = CreateSalesOrderCommand { sales_order: input };
        let created = create_cmd.exec(&mut service).unwrap();

        // Now void it
        let void_cmd = VoidSalesOrderCommand { id: created.id };
        let voided = void_cmd.exec(&mut service).unwrap();

        assert_eq!(voided.state, SalesOrderState::Cancelled);
    }

    #[test]
    fn test_void_already_cancelled_order() {
        let mut service = AppService::new(":memory:");
        let now = Utc::now().naive_utc();
        let cost_center = create_test_cost_center(&mut service);

        // First create a sales order
        let input = SalesOrderNewInput {
            customer_id: Uuid::now_v7().into(),
            customer_name: "John Doe".to_string(),
            customer_phone_number: "+1234567890".to_string(),
            order_date: now,
            net_amount: 1000.into(),
            disc_amount: 100.into(),
            taxable_amount: 900.into(),
            tax_amount: 90.into(),
            total_amount: 990.into(),
            state: SalesOrderState::Completed,
            cost_center_id: cost_center.id,
            items: vec![SalesOrderItemInput {
                item_id: Some(Uuid::now_v7().into()),
                item_name: "Item 1".to_string(),
                quantity: 2,
                price_amount: 500.into(),
                tax_amount: 50.into(),
                total_amount: 1000.into(),
            }],
        };

        let create_cmd = CreateSalesOrderCommand { sales_order: input };
        let created = create_cmd.exec(&mut service).unwrap();

        // Void it first time
        let void_cmd = VoidSalesOrderCommand { id: created.id };
        let _ = void_cmd.exec(&mut service).unwrap();

        // Try to void it again
        let result = void_cmd.exec(&mut service);
        assert!(result.is_err());
    }

    #[test]
    fn test_void_non_existent_order() {
        let mut service = AppService::new(":memory:");

        let cmd = VoidSalesOrderCommand {
            id: Uuid::now_v7().into(),
        };

        let result = cmd.exec(&mut service);
        assert!(result.is_err());
    }
}
