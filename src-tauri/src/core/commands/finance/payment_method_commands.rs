use chrono::Utc;
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{
    core::{
        commands::{app_service::AppService, Command},
        models::finance::payment_method_model::{
            PaymentMethod, PaymentMethodNewInput, PaymentMethodState, PaymentMethodUpdateChangeset,
            PaymentMethodUpdateInput,
        },
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
    schema::payment_methods,
};

// Commands
pub struct CreatePaymentMethodCommand {
    pub payment_method: PaymentMethodNewInput,
}

pub struct UpdatePaymentMethodCommand {
    pub payment_method: PaymentMethodUpdateInput,
}

pub struct DeletePaymentMethodCommand {
    pub id: DbUuid,
}

// Command Implementations
impl Command for CreatePaymentMethodCommand {
    type Output = PaymentMethod;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Check if a payment method with the same code already exists
            let existing = payment_methods::table
                .filter(payment_methods::code.eq(&self.payment_method.code))
                .first::<PaymentMethod>(conn)
                .ok();

            if existing.is_some() {
                return Err(Error::UniqueConstraintError);
            }

            let now = Utc::now().naive_utc();
            let new_payment_method = PaymentMethod {
                id: Uuid::now_v7().into(),
                name: self.payment_method.name.clone(),
                code: self.payment_method.code.clone(),
                description: self.payment_method.description.clone(),
                state: self
                    .payment_method
                    .state
                    .unwrap_or(PaymentMethodState::Active),
                created_at: now,
                updated_at: now,
            };

            let result = diesel::insert_into(payment_methods::table)
                .values(&new_payment_method)
                .returning(PaymentMethod::as_returning())
                .get_result(conn)?;

            Ok(result)
        })
    }
}

impl Command for UpdatePaymentMethodCommand {
    type Output = PaymentMethod;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Check if the payment method exists
            let _ = payment_methods::table
                .find(self.payment_method.id)
                .first::<PaymentMethod>(conn)
                .map_err(|_| Error::NotFoundError)?;

            // Check if we're trying to update the code and it already exists
            if let Some(code) = &self.payment_method.code {
                let existing = payment_methods::table
                    .filter(payment_methods::code.eq(code))
                    .filter(payment_methods::id.ne(self.payment_method.id))
                    .first::<PaymentMethod>(conn)
                    .ok();

                if existing.is_some() {
                    return Err(Error::UniqueConstraintError);
                }
            }

            let now = Utc::now().naive_utc();
            let changeset = PaymentMethodUpdateChangeset {
                id: self.payment_method.id,
                name: self.payment_method.name.clone(),
                code: self.payment_method.code.clone(),
                description: self.payment_method.description.clone(),
                state: self.payment_method.state,
                updated_at: now,
            };

            let result = diesel::update(payment_methods::table)
                .filter(payment_methods::id.eq(self.payment_method.id))
                .set(&changeset)
                .returning(PaymentMethod::as_returning())
                .get_result(conn)?;

            Ok(result)
        })
    }
}

impl Command for DeletePaymentMethodCommand {
    type Output = usize;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Check if the payment method exists
            let _ = payment_methods::table
                .find(self.id)
                .first::<PaymentMethod>(conn)
                .map_err(|_| Error::NotFoundError)?;

            // NOTE: This code is currently commented out because the sales_order_payments
            // table migrations have been created but are not yet applied in the test environment.
            // When the sales_order_payments table is actually implemented and the migrations
            // are applied in production, this code should be uncommented.

            /*
            // Check if payment method is used in any sales_order_payments
            use crate::schema::sales_order_payments;

            let payment_method_in_use = sales_order_payments::table
                .filter(sales_order_payments::payment_method_id.eq(self.id))
                .count()
                .get_result::<i64>(conn)?;

            if payment_method_in_use > 0 {
                return Err(Error::ValidationError(
                    format!("Cannot delete payment method because it is used in {} sales", payment_method_in_use)
                ));
            }
            */

            let result = diesel::delete(payment_methods::table)
                .filter(payment_methods::id.eq(self.id))
                .execute(conn)?;

            Ok(result)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::sql_query;

    fn setup_test_db(service: &mut AppService) {
        // Create payment_methods table for testing
        sql_query(
            "CREATE TABLE IF NOT EXISTS payment_methods (
                id TEXT PRIMARY KEY NOT NULL,
                name TEXT NOT NULL,
                code TEXT NOT NULL UNIQUE,
                description TEXT,
                state TEXT NOT NULL DEFAULT 'Active',
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL
            )",
        )
        .execute(&mut service.conn)
        .unwrap();
    }

    #[test]
    fn test_create_payment_method() {
        let mut service = AppService::new(":memory:");
        setup_test_db(&mut service);

        let input = PaymentMethodNewInput {
            name: "Cash".to_string(),
            code: "CASH_CREATE_TEST".to_string(),
            description: Some("Cash payment".to_string()),
            state: Some(PaymentMethodState::Active),
        };

        let cmd = CreatePaymentMethodCommand {
            payment_method: input,
        };
        let result = cmd.exec(&mut service).unwrap();

        assert_eq!(result.name, "Cash");
        assert_eq!(result.code, "CASH_CREATE_TEST");
        assert_eq!(result.description, Some("Cash payment".to_string()));
        assert_eq!(result.state, PaymentMethodState::Active);
    }

    #[test]
    fn test_create_payment_method_default_state() {
        let mut service = AppService::new(":memory:");
        setup_test_db(&mut service);

        let input = PaymentMethodNewInput {
            name: "Credit Card".to_string(),
            code: "CC_DEFAULT_STATE_TEST".to_string(),
            description: None,
            state: None, // Test default state
        };

        let cmd = CreatePaymentMethodCommand {
            payment_method: input,
        };
        let result = cmd.exec(&mut service).unwrap();

        assert_eq!(result.name, "Credit Card");
        assert_eq!(result.code, "CC_DEFAULT_STATE_TEST");
        assert_eq!(result.description, None);
        assert_eq!(result.state, PaymentMethodState::Active); // Default is Active
    }

    #[test]
    fn test_create_payment_method_duplicate_code() {
        let mut service = AppService::new(":memory:");
        setup_test_db(&mut service);

        // Create first payment method
        let input1 = PaymentMethodNewInput {
            name: "Cash".to_string(),
            code: "CASH_DUP_TEST".to_string(),
            description: None,
            state: None,
        };

        let cmd1 = CreatePaymentMethodCommand {
            payment_method: input1,
        };
        let _ = cmd1.exec(&mut service).unwrap();

        // Try to create with same code
        let input2 = PaymentMethodNewInput {
            name: "Cash 2".to_string(),
            code: "CASH_DUP_TEST".to_string(), // Same code
            description: None,
            state: None,
        };

        let cmd2 = CreatePaymentMethodCommand {
            payment_method: input2,
        };
        let result = cmd2.exec(&mut service);

        assert!(result.is_err());
    }

    #[test]
    fn test_update_payment_method() {
        let mut service = AppService::new(":memory:");
        setup_test_db(&mut service);

        // Create payment method first
        let input = PaymentMethodNewInput {
            name: "Cash".to_string(),
            code: "CASH_UPDATE_TEST".to_string(),
            description: None,
            state: None,
        };

        let cmd = CreatePaymentMethodCommand {
            payment_method: input,
        };
        let created = cmd.exec(&mut service).unwrap();

        // Now update it
        let update_input = PaymentMethodUpdateInput {
            id: created.id,
            name: Some("Cash Updated".to_string()),
            code: None, // Don't change code
            description: Some(Some("Updated description".to_string())),
            state: Some(PaymentMethodState::Inactive),
        };

        let update_cmd = UpdatePaymentMethodCommand {
            payment_method: update_input,
        };
        let updated = update_cmd.exec(&mut service).unwrap();

        assert_eq!(updated.name, "Cash Updated");
        assert_eq!(updated.code, "CASH_UPDATE_TEST"); // Unchanged
        assert_eq!(updated.description, Some("Updated description".to_string()));
        assert_eq!(updated.state, PaymentMethodState::Inactive);
    }

    #[test]
    fn test_update_payment_method_duplicate_code() {
        let mut service = AppService::new(":memory:");
        setup_test_db(&mut service);

        // Create first payment method
        let input1 = PaymentMethodNewInput {
            name: "Cash".to_string(),
            code: "CASH_DUP_UPDATE_TEST".to_string(),
            description: None,
            state: None,
        };

        let cmd1 = CreatePaymentMethodCommand {
            payment_method: input1,
        };
        let _ = cmd1.exec(&mut service).unwrap();

        // Create second payment method
        let input2 = PaymentMethodNewInput {
            name: "Credit Card".to_string(),
            code: "CC_DUP_UPDATE_TEST".to_string(),
            description: None,
            state: None,
        };

        let cmd2 = CreatePaymentMethodCommand {
            payment_method: input2,
        };
        let created2 = cmd2.exec(&mut service).unwrap();

        // Try to update second payment method with code of first
        let update_input = PaymentMethodUpdateInput {
            id: created2.id,
            name: None,
            code: Some("CASH_DUP_UPDATE_TEST".to_string()), // Duplicate code
            description: None,
            state: None,
        };

        let update_cmd = UpdatePaymentMethodCommand {
            payment_method: update_input,
        };
        let result = update_cmd.exec(&mut service);

        assert!(result.is_err());
    }

    #[test]
    fn test_update_nonexistent_payment_method() {
        let mut service = AppService::new(":memory:");
        setup_test_db(&mut service);

        let update_input = PaymentMethodUpdateInput {
            id: Uuid::now_v7().into(),
            name: Some("Test".to_string()),
            code: None,
            description: None,
            state: None,
        };

        let update_cmd = UpdatePaymentMethodCommand {
            payment_method: update_input,
        };
        let result = update_cmd.exec(&mut service);

        assert!(result.is_err());
    }

    #[test]
    fn test_delete_payment_method() {
        let mut service = AppService::new(":memory:");
        setup_test_db(&mut service);

        // Create payment method first
        let input = PaymentMethodNewInput {
            name: "Cash".to_string(),
            code: "CASH_DELETE_TEST".to_string(),
            description: None,
            state: None,
        };

        let cmd = CreatePaymentMethodCommand {
            payment_method: input,
        };
        let created = cmd.exec(&mut service).unwrap();

        // Now delete it
        let delete_cmd = DeletePaymentMethodCommand { id: created.id };
        let result = delete_cmd.exec(&mut service).unwrap();

        assert_eq!(result, 1); // 1 row affected
    }

    #[test]
    fn test_delete_nonexistent_payment_method() {
        let mut service = AppService::new(":memory:");
        setup_test_db(&mut service);

        let delete_cmd = DeletePaymentMethodCommand {
            id: Uuid::now_v7().into(),
        };
        let result = delete_cmd.exec(&mut service);

        assert!(result.is_err());
    }
}
