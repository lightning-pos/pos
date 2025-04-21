use chrono::Utc;
use sea_query::{Expr, Query};
use uuid::Uuid;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        commands::{app_service::AppService, Command},
        models::{
            finance::sales_order_payment_model::{
                SalesOrderPayment, SalesOrderPaymentNewInput, SalesOrderPaymentState,
                SalesOrderPaymentUpdateChangeset, SalesOrderPaymentUpdateInput, SalesOrderPayments,
            },
            sales::sales_order_model::{SalesOrder, SalesOrderState, SalesOrders},
        },
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
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

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let now = Utc::now().naive_utc();

        // Check if the order exists and is in Completed state
        let mut select_query = Query::select();
        let check_stmt = select_query
            .from(SalesOrders::Table)
            .column(SalesOrders::Id)
            .and_where(Expr::col(SalesOrders::Id).eq(self.payment.order_id.to_string()))
            .and_where(Expr::col(SalesOrders::OrderState).eq(SalesOrderState::Completed.to_string()));

        let order = service.db_adapter.query_optional::<SalesOrder>(&check_stmt).await?;
        if order.is_none() {
            return Err(Error::NotFoundError);
        }

        // Create a new payment ID
        let payment_id: DbUuid = Uuid::now_v7().into();

        // Create the payment
        let new_payment = SalesOrderPayment {
            id: payment_id,
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

        // Build the insert query
        let mut insert_query = Query::insert();
        let insert_stmt = insert_query
            .into_table(SalesOrderPayments::Table)
            .columns([
                SalesOrderPayments::Id,
                SalesOrderPayments::OrderId,
                SalesOrderPayments::PaymentMethodId,
                SalesOrderPayments::PaymentDate,
                SalesOrderPayments::Amount,
                SalesOrderPayments::ReferenceNumber,
                SalesOrderPayments::Notes,
                SalesOrderPayments::State,
                SalesOrderPayments::CreatedAt,
                SalesOrderPayments::UpdatedAt,
            ])
            .values_panic([
                payment_id.to_string().into(),
                self.payment.order_id.to_string().into(),
                self.payment.payment_method_id.to_string().into(),
                self.payment.payment_date.to_string().into(),
                self.payment.amount.to_string().into(),
                match &self.payment.reference_number {
                    Some(ref_num) => ref_num.clone().into(),
                    None => sea_query::Value::String(None).into(),
                },
                match &self.payment.notes {
                    Some(notes) => notes.clone().into(),
                    None => sea_query::Value::String(None).into(),
                },
                new_payment.state.to_string().into(),
                now.to_string().into(),
                now.to_string().into(),
            ]);

        // Execute the insert query
        service.db_adapter.insert_many(&insert_stmt).await?;

        Ok(new_payment)
    }
}

impl Command for UpdateSalesOrderPaymentCommand {
    type Output = SalesOrderPayment;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let now = Utc::now().naive_utc();

        // Check if the payment exists and is in Completed state
        let mut select_query = Query::select();
        let check_stmt = select_query
            .from(SalesOrderPayments::Table)
            .columns([
                SalesOrderPayments::Id,
                SalesOrderPayments::OrderId,
                SalesOrderPayments::PaymentMethodId,
                SalesOrderPayments::PaymentDate,
                SalesOrderPayments::Amount,
                SalesOrderPayments::ReferenceNumber,
                SalesOrderPayments::Notes,
                SalesOrderPayments::State,
                SalesOrderPayments::CreatedAt,
                SalesOrderPayments::UpdatedAt,
            ])
            .and_where(Expr::col(SalesOrderPayments::Id).eq(self.payment.id.to_string()))
            .and_where(Expr::col(SalesOrderPayments::State).eq(SalesOrderPaymentState::Completed.to_string()));

        let payment = service.db_adapter.query_optional::<SalesOrderPayment>(&check_stmt).await?;
        if payment.is_none() {
            return Err(Error::NotFoundError);
        }

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

        // Build the update query
        let mut update_query = Query::update();
        let update_stmt = update_query.table(SalesOrderPayments::Table)
            .value(SalesOrderPayments::UpdatedAt, now.to_string());

        // Add optional fields if they exist
        if let Some(payment_method_id) = &changeset.payment_method_id {
            update_stmt.value(SalesOrderPayments::PaymentMethodId, payment_method_id.to_string());
        }

        if let Some(payment_date) = &changeset.payment_date {
            update_stmt.value(SalesOrderPayments::PaymentDate, payment_date.to_string());
        }

        if let Some(amount) = &changeset.amount {
            update_stmt.value(SalesOrderPayments::Amount, amount.to_string());
        }

        if let Some(reference_number) = &changeset.reference_number {
            match reference_number {
                Some(ref_num) => update_stmt.value(SalesOrderPayments::ReferenceNumber, ref_num.clone()),
                None => update_stmt.value(SalesOrderPayments::ReferenceNumber, sea_query::Value::String(None)),
            };
        }

        if let Some(notes) = &changeset.notes {
            match notes {
                Some(note_text) => update_stmt.value(SalesOrderPayments::Notes, note_text.clone()),
                None => update_stmt.value(SalesOrderPayments::Notes, sea_query::Value::String(None)),
            };
        }

        if let Some(state) = &changeset.state {
            update_stmt.value(SalesOrderPayments::State, state.to_string());
        }

        // Add WHERE condition
        update_stmt.and_where(Expr::col(SalesOrderPayments::Id).eq(self.payment.id.to_string()));

        // Execute the update
        service.db_adapter.update_many(&update_stmt).await?;

        // Retrieve the updated payment
        let mut select_query = Query::select();
        let select_stmt = select_query
            .from(SalesOrderPayments::Table)
            .columns([
                SalesOrderPayments::Id,
                SalesOrderPayments::OrderId,
                SalesOrderPayments::PaymentMethodId,
                SalesOrderPayments::PaymentDate,
                SalesOrderPayments::Amount,
                SalesOrderPayments::ReferenceNumber,
                SalesOrderPayments::Notes,
                SalesOrderPayments::State,
                SalesOrderPayments::CreatedAt,
                SalesOrderPayments::UpdatedAt,
            ])
            .and_where(Expr::col(SalesOrderPayments::Id).eq(self.payment.id.to_string()));

        let updated_payment = service.db_adapter.query_one::<SalesOrderPayment>(&select_stmt).await?;

        Ok(updated_payment)
    }
}

impl Command for VoidSalesOrderPaymentCommand {
    type Output = SalesOrderPayment;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let now = Utc::now().naive_utc();

        // Check if the payment exists and is in Completed state
        let mut select_query = Query::select();
        let check_stmt = select_query
            .from(SalesOrderPayments::Table)
            .columns([
                SalesOrderPayments::Id,
                SalesOrderPayments::OrderId,
                SalesOrderPayments::PaymentMethodId,
                SalesOrderPayments::PaymentDate,
                SalesOrderPayments::Amount,
                SalesOrderPayments::ReferenceNumber,
                SalesOrderPayments::Notes,
                SalesOrderPayments::State,
                SalesOrderPayments::CreatedAt,
                SalesOrderPayments::UpdatedAt,
            ])
            .and_where(Expr::col(SalesOrderPayments::Id).eq(self.id.to_string()))
            .and_where(Expr::col(SalesOrderPayments::State).eq(SalesOrderPaymentState::Completed.to_string()));

        let payment = service.db_adapter.query_optional::<SalesOrderPayment>(&check_stmt).await?;
        if payment.is_none() {
            return Err(Error::NotFoundError);
        }

        // Build the update query
        let mut update_query = Query::update();
        let update_stmt = update_query
            .table(SalesOrderPayments::Table)
            .value(SalesOrderPayments::State, SalesOrderPaymentState::Voided.to_string())
            .value(SalesOrderPayments::UpdatedAt, now.to_string())
            .and_where(Expr::col(SalesOrderPayments::Id).eq(self.id.to_string()));

        // Execute the update
        service.db_adapter.update_many(&update_stmt).await?;

        // Retrieve the updated payment
        let mut select_query = Query::select();
        let select_stmt = select_query
            .from(SalesOrderPayments::Table)
            .columns([
                SalesOrderPayments::Id,
                SalesOrderPayments::OrderId,
                SalesOrderPayments::PaymentMethodId,
                SalesOrderPayments::PaymentDate,
                SalesOrderPayments::Amount,
                SalesOrderPayments::ReferenceNumber,
                SalesOrderPayments::Notes,
                SalesOrderPayments::State,
                SalesOrderPayments::CreatedAt,
                SalesOrderPayments::UpdatedAt,
            ])
            .and_where(Expr::col(SalesOrderPayments::Id).eq(self.id.to_string()));

        let updated_payment = service.db_adapter.query_one::<SalesOrderPayment>(&select_stmt).await?;

        Ok(updated_payment)
    }
}

impl Command for GetSalesOrderPaymentsCommand {
    type Output = Vec<SalesOrderPayment>;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Check if the order exists
        let mut check_query_builder = Query::select();
        let check_stmt = check_query_builder
            .from(SalesOrders::Table)
            .column(SalesOrders::Id)
            .and_where(Expr::col(SalesOrders::Id).eq(self.order_id.to_string()));

        let order = service.db_adapter.query_optional::<SalesOrder>(&check_stmt).await?;
        if order.is_none() {
            return Err(Error::NotFoundError);
        }

        // Get all payments for the order
        let mut select_query_builder = Query::select();
        let select_stmt = select_query_builder
            .from(SalesOrderPayments::Table)
            .columns([
                SalesOrderPayments::Id,
                SalesOrderPayments::OrderId,
                SalesOrderPayments::PaymentMethodId,
                SalesOrderPayments::PaymentDate,
                SalesOrderPayments::Amount,
                SalesOrderPayments::ReferenceNumber,
                SalesOrderPayments::Notes,
                SalesOrderPayments::State,
                SalesOrderPayments::CreatedAt,
                SalesOrderPayments::UpdatedAt,
            ])
            .and_where(Expr::col(SalesOrderPayments::OrderId).eq(self.order_id.to_string()));

        let payments = service.db_adapter.query_many::<SalesOrderPayment>(&select_stmt).await?;

        Ok(payments)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::commands::tests::setup_service;
    use crate::core::models::finance::sales_order_payment_model::SalesOrderPaymentState;
    use crate::core::{
        commands::{
            auth::user_commands::AddUserCommand,
            common::channel_commands::CreateChannelCommand,
            common::location_commands::CreateLocationCommand,
            finance::payment_method_commands::CreatePaymentMethodCommand,
            sales::sales_order_commands::CreateSalesOrderCommand,
        },
        models::{
            auth::user_model::UserNewInput,
            common::channel_model::{Channel, ChannelNewInput},
            common::location_model::{Location, LocationNewInput},
            finance::payment_method_model::{PaymentMethodNewInput, PaymentMethodState},
            sales::{
                sales_order_item_model::SalesOrderItemInput, sales_order_model::SalesOrderNewInput,
            },
        },
    };
    use rand::Rng;
    use sea_query::{Expr, Query};

    async fn create_test_payment_method(
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
        command.exec(service).await.unwrap()
    }

    async fn create_test_cost_center(
        service: &mut AppService,
    ) -> crate::core::models::finance::cost_center_model::CostCenter {
        use crate::core::{
            commands::{finance::cost_center_commands::CreateCostCenterCommand, Command},
            models::finance::cost_center_model::{CostCenterNewInput, CostCenterState},
        };

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

    async fn create_test_sales_order(service: &mut AppService) -> SalesOrder {
        let now = Utc::now().naive_utc();
        let cost_center = create_test_cost_center(service).await;
        let user_id = create_test_user(service).await;
        let channel = create_test_channel(service).await;
        let location = create_test_location(service).await;

        let input = SalesOrderNewInput {
            customer_id: None, // No customer needed for this test
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
            channel_id: channel.id,
            location_id: location.id,
            cost_center_id: cost_center.id,
            discount_id: None,
            items: vec![SalesOrderItemInput {
                item_id: None, // No item needed for this test
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
        cmd.exec(service).await.unwrap()
    }

    #[tokio::test]
    async fn test_create_sales_order_payment() {
        let mut service = setup_service();
        let now = Utc::now().naive_utc();
        let order = create_test_sales_order(&mut service).await;
        let payment_method = create_test_payment_method(&mut service).await;

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
        let result = cmd.exec(&mut service).await.unwrap();

        assert_eq!(result.order_id, order.id);
        assert_eq!(result.payment_method_id, payment_method.id);
        assert_eq!(result.amount, 500.into());
        assert_eq!(result.state, SalesOrderPaymentState::Completed);
    }

    #[tokio::test]
    async fn test_create_multiple_payments_for_order() {
        let mut service = setup_service();
        let now = Utc::now().naive_utc();
        let order = create_test_sales_order(&mut service).await;
        let payment_method = create_test_payment_method(&mut service).await;

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
        let result1 = cmd1.exec(&mut service).await.unwrap();

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
        let result2 = cmd2.exec(&mut service).await.unwrap();

        assert_eq!(result1.order_id, order.id);
        assert_eq!(result1.amount, 500.into());

        assert_eq!(result2.order_id, order.id);
        assert_eq!(result2.amount, 490.into());

        // Query the database to ensure both payments exist
        let mut select_query_builder = Query::select();
        let select_stmt = select_query_builder
            .from(SalesOrderPayments::Table)
            .columns([
                SalesOrderPayments::Id,
                SalesOrderPayments::OrderId,
                SalesOrderPayments::PaymentMethodId,
                SalesOrderPayments::PaymentDate,
                SalesOrderPayments::Amount,
                SalesOrderPayments::ReferenceNumber,
                SalesOrderPayments::Notes,
                SalesOrderPayments::State,
                SalesOrderPayments::CreatedAt,
                SalesOrderPayments::UpdatedAt,
            ])
            .and_where(Expr::col(SalesOrderPayments::OrderId).eq(order.id.to_string()));

        let payments = service.db_adapter.query_many::<SalesOrderPayment>(&select_stmt).await.unwrap();

        assert_eq!(payments.len(), 2);
        assert!(payments.iter().any(|p| p.id == result1.id));
        assert!(payments.iter().any(|p| p.id == result2.id));
    }

    #[tokio::test]
    async fn test_update_sales_order_payment() {
        let mut service = setup_service();
        let now = Utc::now().naive_utc();
        let order = create_test_sales_order(&mut service).await;
        let payment_method = create_test_payment_method(&mut service).await;

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
        let payment = cmd.exec(&mut service).await.unwrap();

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
        let updated_payment = update_cmd.exec(&mut service).await.unwrap();

        assert_eq!(updated_payment.id, payment.id);
        assert_eq!(updated_payment.amount, 600.into());
        assert_eq!(
            updated_payment.reference_number,
            Some("REF123-UPDATED".to_string())
        );
        assert_eq!(updated_payment.notes, Some("First payment".to_string())); // Unchanged
    }

    #[tokio::test]
    async fn test_void_sales_order_payment() {
        let mut service = setup_service();
        let now = Utc::now().naive_utc();
        let order = create_test_sales_order(&mut service).await;
        let payment_method = create_test_payment_method(&mut service).await;

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
        let payment = cmd.exec(&mut service).await.unwrap();

        let void_cmd = VoidSalesOrderPaymentCommand { id: payment.id };
        let voided_payment = void_cmd.exec(&mut service).await.unwrap();

        assert_eq!(voided_payment.id, payment.id);
        assert_eq!(voided_payment.state, SalesOrderPaymentState::Voided);
    }

    #[tokio::test]
    async fn test_void_already_voided_payment() {
        let mut service = setup_service();
        let now = Utc::now().naive_utc();
        let order = create_test_sales_order(&mut service).await;
        let payment_method = create_test_payment_method(&mut service).await;

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
        let payment = cmd.exec(&mut service).await.unwrap();
        let void_cmd = VoidSalesOrderPaymentCommand { id: payment.id };
        let _ = void_cmd.exec(&mut service).await.unwrap(); // First void
        let result = void_cmd.exec(&mut service).await; // Second void
                                                  // Expect NotFoundError because the payment is no longer in Completed state
        assert!(matches!(result, Err(Error::NotFoundError)));
    }

    #[tokio::test]
    async fn test_void_non_existent_payment() {
        let mut service = setup_service();
        let id = Uuid::now_v7().into();

        let void_cmd = VoidSalesOrderPaymentCommand { id };
        let result = void_cmd.exec(&mut service).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_non_existent_payment() {
        let mut service = setup_service();
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
        let result = update_cmd.exec(&mut service).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_sales_order_payments() {
        let mut service = setup_service();
        let now = Utc::now().naive_utc();
        let order = create_test_sales_order(&mut service).await;
        let payment_method = create_test_payment_method(&mut service).await;

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

        let _result1 = cmd1.exec(&mut service).await.unwrap();
        let _result2 = cmd2.exec(&mut service).await.unwrap();

        // Get all payments for the order
        let get_cmd = GetSalesOrderPaymentsCommand { order_id: order.id };
        let payments = get_cmd.exec(&mut service).await.unwrap();

        assert_eq!(payments.len(), 2);
        assert!(payments.iter().any(|p| p.amount == 500.into()));
        assert!(payments.iter().any(|p| p.amount == 490.into()));
    }

    #[tokio::test]
    async fn test_get_payments_for_nonexistent_order() {
        let mut service = setup_service();
        let id = Uuid::now_v7().into();

        let get_cmd = GetSalesOrderPaymentsCommand { order_id: id };
        let result = get_cmd.exec(&mut service).await;

        assert!(result.is_err());
    }
}
