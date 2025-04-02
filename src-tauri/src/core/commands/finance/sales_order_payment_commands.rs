use chrono::Utc;
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{
    core::{
        commands::{app_service::AppService, Command},
        models::{
            finance::sales_order_payment_model::{
                SalesOrderPayment, SalesOrderPaymentNewInput, SalesOrderPaymentState,
                SalesOrderPaymentUpdateChangeset, SalesOrderPaymentUpdateInput,
            },
            sales::sales_order_model::{SalesOrder, SalesOrderState},
        },
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
    schema::{sales_order_payments, sales_orders},
};

// Commands
pub struct CreateSalesOrderPaymentCommand {
    pub payment: SalesOrderPaymentNewInput,
}

pub struct UpdateSalesOrderPaymentCommand {
    pub payment: SalesOrderPaymentUpdateInput,
}

pub struct VoidSalesOrderPaymentCommand {
    pub id: DbUuid,
}

pub struct GetSalesOrderPaymentsCommand {
    pub order_id: DbUuid,
}

// Command Implementations
impl Command for CreateSalesOrderPaymentCommand {
    type Output = SalesOrderPayment;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let now = Utc::now().naive_utc();

            // Check if the order exists and is in Completed state
            let _order = sales_orders::table
                .find(self.payment.order_id)
                .filter(sales_orders::order_state.eq(SalesOrderState::Completed))
                .select(SalesOrder::as_select())
                .first::<SalesOrder>(conn)
                .map_err(|_| Error::NotFoundError)?;

            // Create the payment
            let new_payment = SalesOrderPayment {
                id: Uuid::now_v7().into(),
                order_id: self.payment.order_id,
                payment_method_id: self.payment.payment_method_id,
                payment_date: self.payment.payment_date,
                amount: self.payment.amount,
                reference_number: self.payment.reference_number.clone(),
                notes: self.payment.notes.clone(),
                state: self
                    .payment
                    .state
                    .unwrap_or(SalesOrderPaymentState::Completed),
                created_at: now,
                updated_at: now,
            };

            // Insert the payment
            let payment = diesel::insert_into(sales_order_payments::table)
                .values(&new_payment)
                .returning(SalesOrderPayment::as_returning())
                .get_result(conn)?;

            Ok(payment)
        })
    }
}

impl Command for UpdateSalesOrderPaymentCommand {
    type Output = SalesOrderPayment;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let now = Utc::now().naive_utc();

            // Check if the payment exists and is in Completed state
            let _payment = sales_order_payments::table
                .find(self.payment.id)
                .filter(sales_order_payments::state.eq(SalesOrderPaymentState::Completed))
                .first::<SalesOrderPayment>(conn)
                .map_err(|_| Error::NotFoundError)?;

            // Create the changeset
            let changeset = SalesOrderPaymentUpdateChangeset {
                id: self.payment.id,
                payment_method_id: self.payment.payment_method_id,
                payment_date: self.payment.payment_date,
                amount: self.payment.amount,
                reference_number: self.payment.reference_number.clone(),
                notes: self.payment.notes.clone(),
                state: self.payment.state,
                updated_at: now,
            };

            // Update the payment
            let updated_payment = diesel::update(sales_order_payments::table)
                .filter(sales_order_payments::id.eq(self.payment.id))
                .set(&changeset)
                .returning(SalesOrderPayment::as_returning())
                .get_result(conn)?;

            Ok(updated_payment)
        })
    }
}

impl Command for VoidSalesOrderPaymentCommand {
    type Output = SalesOrderPayment;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let now = Utc::now().naive_utc();

            // Check if the payment exists and is in Completed state
            let _payment = sales_order_payments::table
                .find(self.id)
                .filter(sales_order_payments::state.eq(SalesOrderPaymentState::Completed))
                .first::<SalesOrderPayment>(conn)
                .map_err(|_| Error::NotFoundError)?;

            // Update the payment state
            let updated_payment = diesel::update(sales_order_payments::table)
                .filter(sales_order_payments::id.eq(self.id))
                .set((
                    sales_order_payments::state.eq(SalesOrderPaymentState::Voided),
                    sales_order_payments::updated_at.eq(now),
                ))
                .returning(SalesOrderPayment::as_returning())
                .get_result(conn)?;

            Ok(updated_payment)
        })
    }
}

impl Command for GetSalesOrderPaymentsCommand {
    type Output = Vec<SalesOrderPayment>;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Check if the order exists
        let _ = sales_orders::table
            .find(self.order_id)
            .first::<SalesOrder>(&mut service.conn)
            .map_err(|_| Error::NotFoundError)?;

        // Get all payments for the order
        let payments = sales_order_payments::table
            .filter(sales_order_payments::order_id.eq(self.order_id))
            .load::<SalesOrderPayment>(&mut service.conn)?;

        Ok(payments)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::finance::sales_order_payment_model::SalesOrderPaymentState as FinancePaymentState;
    use crate::core::{
        commands::{
            finance::payment_method_commands::CreatePaymentMethodCommand,
            sales::sales_order_commands::CreateSalesOrderCommand,
        },
        models::{
            finance::payment_method_model::{PaymentMethodNewInput, PaymentMethodState},
            sales::{
                sales_order_charge_model::SalesOrderChargeNewInput,
                sales_order_item_model::SalesOrderItemInput,
                sales_order_model::{SalesOrderNewInput, SalesOrderState},
            },
        },
    };

    fn create_test_payment_method(
        service: &mut AppService,
    ) -> crate::core::models::finance::payment_method_model::PaymentMethod {
        let command = CreatePaymentMethodCommand {
            payment_method: PaymentMethodNewInput {
                name: "Test Payment Method".to_string(),
                code: "TPM001".to_string(),
                description: None,
                state: Some(PaymentMethodState::Active),
            },
        };
        command.exec(service).unwrap()
    }

    fn create_test_cost_center(
        service: &mut AppService,
    ) -> crate::core::models::finance::cost_center_model::CostCenter {
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

    fn create_test_sales_order(service: &mut AppService) -> SalesOrder {
        let now = Utc::now().naive_utc();
        let cost_center = create_test_cost_center(service);
        let user_id = Uuid::new_v4().into();
        let channel_id = Uuid::new_v4().into();
        let location_id = Uuid::new_v4().into();

        let input = SalesOrderNewInput {
            customer_id: Some(Uuid::now_v7().into()),
            customer_name: Some("John Doe".to_string()),
            customer_phone_number: Some("+1234567890".to_string()),
            billing_address: None,
            shipping_address: None,
            order_date: now,
            net_amount: 1000.into(),
            disc_amount: 100.into(),
            taxable_amount: 900.into(),
            tax_amount: 90.into(),
            total_amount: 990.into(),
            notes: None,
            channel_id,
            location_id,
            cost_center_id: cost_center.id,
            discount_id: None,
            items: vec![SalesOrderItemInput {
                item_id: Some(Uuid::now_v7().into()),
                item_name: "Item 1".to_string(),
                quantity: 2,
                sku: None,
                price_amount: 500.into(),
                disc_amount: 50.into(),
                taxable_amount: 450.into(),
                tax_amount: 50.into(),
                total_amount: 990.into(),
            }],
            charges: None,
        };

        let cmd = CreateSalesOrderCommand {
            sales_order: input,
            created_by_user_id: user_id,
        };
        cmd.exec(service).unwrap()
    }

    #[test]
    fn test_create_sales_order_payment() {
        let mut service = AppService::new(":memory:");
        let now = Utc::now().naive_utc();
        let order = create_test_sales_order(&mut service);
        let payment_method = create_test_payment_method(&mut service);

        let input = SalesOrderPaymentNewInput {
            order_id: order.id,
            payment_method_id: payment_method.id,
            payment_date: now,
            amount: 500.into(),
            reference_number: Some("REF123".to_string()),
            notes: Some("First payment".to_string()),
            state: None,
        };

        let cmd = CreateSalesOrderPaymentCommand { payment: input };
        let result = cmd.exec(&mut service).unwrap();

        assert_eq!(result.order_id, order.id);
        assert_eq!(result.payment_method_id, payment_method.id);
        assert_eq!(result.amount, 500.into());
        assert_eq!(result.state, FinancePaymentState::Completed);
    }

    #[test]
    fn test_create_multiple_payments_for_order() {
        let mut service = AppService::new(":memory:");
        let now = Utc::now().naive_utc();
        let order = create_test_sales_order(&mut service);
        let payment_method = create_test_payment_method(&mut service);

        // First payment
        let input1 = SalesOrderPaymentNewInput {
            order_id: order.id,
            payment_method_id: payment_method.id,
            payment_date: now,
            amount: 500.into(),
            reference_number: Some("REF123".to_string()),
            notes: Some("First payment".to_string()),
            state: None,
        };

        let cmd1 = CreateSalesOrderPaymentCommand { payment: input1 };
        let result1 = cmd1.exec(&mut service).unwrap();

        // Second payment
        let input2 = SalesOrderPaymentNewInput {
            order_id: order.id,
            payment_method_id: payment_method.id,
            payment_date: now,
            amount: 490.into(),
            reference_number: Some("REF124".to_string()),
            notes: Some("Second payment".to_string()),
            state: None,
        };

        let cmd2 = CreateSalesOrderPaymentCommand { payment: input2 };
        let result2 = cmd2.exec(&mut service).unwrap();

        assert_eq!(result1.order_id, order.id);
        assert_eq!(result1.amount, 500.into());

        assert_eq!(result2.order_id, order.id);
        assert_eq!(result2.amount, 490.into());

        // Query the database to ensure both payments exist
        let payments = sales_order_payments::table
            .filter(sales_order_payments::order_id.eq(order.id))
            .load::<SalesOrderPayment>(&mut service.conn)
            .unwrap();

        assert_eq!(payments.len(), 2);
        assert_eq!(payments[0].id, result1.id);
        assert_eq!(payments[1].id, result2.id);
    }

    #[test]
    fn test_update_sales_order_payment() {
        let mut service = AppService::new(":memory:");
        let now = Utc::now().naive_utc();
        let order = create_test_sales_order(&mut service);
        let payment_method = create_test_payment_method(&mut service);

        // Create a payment
        let input = SalesOrderPaymentNewInput {
            order_id: order.id,
            payment_method_id: payment_method.id,
            payment_date: now,
            amount: 500.into(),
            reference_number: Some("REF123".to_string()),
            notes: Some("First payment".to_string()),
            state: None,
        };

        let cmd = CreateSalesOrderPaymentCommand { payment: input };
        let payment = cmd.exec(&mut service).unwrap();

        // Update the payment
        let update_input = SalesOrderPaymentUpdateInput {
            id: payment.id,
            payment_method_id: None,
            payment_date: None,
            amount: Some(600.into()),
            reference_number: Some(Some("REF123-UPDATED".to_string())),
            notes: None,
            state: None,
        };

        let update_cmd = UpdateSalesOrderPaymentCommand {
            payment: update_input,
        };
        let updated_payment = update_cmd.exec(&mut service).unwrap();

        assert_eq!(updated_payment.id, payment.id);
        assert_eq!(updated_payment.amount, 600.into());
        assert_eq!(
            updated_payment.reference_number,
            Some("REF123-UPDATED".to_string())
        );
        assert_eq!(updated_payment.notes, Some("First payment".to_string())); // Unchanged
    }

    #[test]
    fn test_void_sales_order_payment() {
        let mut service = AppService::new(":memory:");
        let now = Utc::now().naive_utc();
        let order = create_test_sales_order(&mut service);
        let payment_method = create_test_payment_method(&mut service);

        let input = SalesOrderPaymentNewInput {
            order_id: order.id,
            payment_method_id: payment_method.id,
            payment_date: now,
            amount: 500.into(),
            reference_number: Some("REF123".to_string()),
            notes: Some("First payment".to_string()),
            state: None,
        };

        let cmd = CreateSalesOrderPaymentCommand { payment: input };
        let payment = cmd.exec(&mut service).unwrap();

        let void_cmd = VoidSalesOrderPaymentCommand { id: payment.id };
        let voided_payment = void_cmd.exec(&mut service).unwrap();

        assert_eq!(voided_payment.id, payment.id);
        assert_eq!(voided_payment.state, FinancePaymentState::Voided);
    }

    #[test]
    fn test_void_already_voided_payment() {
        let mut service = AppService::new(":memory:");
        let now = Utc::now().naive_utc();
        let order = create_test_sales_order(&mut service);
        let payment_method = create_test_payment_method(&mut service);

        let input = SalesOrderPaymentNewInput {
            order_id: order.id,
            payment_method_id: payment_method.id,
            payment_date: now,
            amount: 500.into(),
            reference_number: Some("REF123".to_string()),
            notes: Some("First payment".to_string()),
            state: None,
        };

        let cmd = CreateSalesOrderPaymentCommand { payment: input };
        let payment = cmd.exec(&mut service).unwrap();
        let void_cmd = VoidSalesOrderPaymentCommand { id: payment.id };
        let _ = void_cmd.exec(&mut service).unwrap(); // First void
        let result = void_cmd.exec(&mut service); // Second void
                                                  // Expect NotFoundError because the payment is no longer in Completed state
        assert!(matches!(result, Err(Error::NotFoundError)));
    }

    #[test]
    fn test_void_non_existent_payment() {
        let mut service = AppService::new(":memory:");
        let id = Uuid::now_v7().into();

        let void_cmd = VoidSalesOrderPaymentCommand { id };
        let result = void_cmd.exec(&mut service);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_non_existent_payment() {
        let mut service = AppService::new(":memory:");
        let id = Uuid::now_v7().into();

        let update_input = SalesOrderPaymentUpdateInput {
            id,
            payment_method_id: None,
            payment_date: None,
            amount: Some(600.into()),
            reference_number: None,
            notes: None,
            state: None,
        };

        let update_cmd = UpdateSalesOrderPaymentCommand {
            payment: update_input,
        };
        let result = update_cmd.exec(&mut service);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_sales_order_payments() {
        let mut service = AppService::new(":memory:");
        let now = Utc::now().naive_utc();
        let order = create_test_sales_order(&mut service);
        let payment_method = create_test_payment_method(&mut service);

        // Create two payments
        let input1 = SalesOrderPaymentNewInput {
            order_id: order.id,
            payment_method_id: payment_method.id,
            payment_date: now,
            amount: 500.into(),
            reference_number: Some("REF123".to_string()),
            notes: Some("First payment".to_string()),
            state: None,
        };

        let input2 = SalesOrderPaymentNewInput {
            order_id: order.id,
            payment_method_id: payment_method.id,
            payment_date: now,
            amount: 490.into(),
            reference_number: Some("REF124".to_string()),
            notes: Some("Second payment".to_string()),
            state: None,
        };

        let cmd1 = CreateSalesOrderPaymentCommand { payment: input1 };
        let cmd2 = CreateSalesOrderPaymentCommand { payment: input2 };

        let _result1 = cmd1.exec(&mut service).unwrap();
        let _result2 = cmd2.exec(&mut service).unwrap();

        // Get all payments for the order
        let get_cmd = GetSalesOrderPaymentsCommand { order_id: order.id };
        let payments = get_cmd.exec(&mut service).unwrap();

        assert_eq!(payments.len(), 2);
        assert_eq!(payments[0].amount, 500.into());
        assert_eq!(payments[1].amount, 490.into());
    }

    #[test]
    fn test_get_payments_for_nonexistent_order() {
        let mut service = AppService::new(":memory:");
        let id = Uuid::now_v7().into();

        let get_cmd = GetSalesOrderPaymentsCommand { order_id: id };
        let result = get_cmd.exec(&mut service);

        assert!(result.is_err());
    }
}
