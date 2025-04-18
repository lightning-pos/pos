use chrono::Utc;
use sea_query::{Expr, Query, SqliteQueryBuilder};
use uuid::Uuid;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        commands::{app_service::AppService, Command},
        models::finance::payment_method_model::{
            PaymentMethod, PaymentMethodNewInput, PaymentMethodState, PaymentMethodUpdateInput, PaymentMethods,
        },
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
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
        // Check if a payment method with the same code already exists
        let check_query = Query::select()
            .from(PaymentMethods::Table)
            .columns([PaymentMethods::Id])
            .and_where(Expr::col(PaymentMethods::Code).eq(&self.payment_method.code))
            .to_string(SqliteQueryBuilder);

        let existing = service.db_adapter.query_optional::<DbUuid>(&check_query, vec![])?;

        if existing.is_some() {
            return Err(Error::UniqueConstraintError);
        }

        let now = Utc::now().naive_utc();
        let new_id = Uuid::now_v7();

        let new_payment_method = PaymentMethod {
            id: new_id.into(),
            name: self.payment_method.name.clone(),
            code: self.payment_method.code.clone(),
            description: self.payment_method.description.clone(),
            state: self.payment_method.state.unwrap_or(PaymentMethodState::Active),
            created_at: now,
            updated_at: now,
        };

        // Build the insert query with SeaQuery
        let query = Query::insert()
            .into_table(PaymentMethods::Table)
            .columns([
                PaymentMethods::Id,
                PaymentMethods::Name,
                PaymentMethods::Code,
                PaymentMethods::Description,
                PaymentMethods::State,
                PaymentMethods::CreatedAt,
                PaymentMethods::UpdatedAt,
            ])
            .values_panic([
                new_id.to_string().into(),
                self.payment_method.name.clone().into(),
                self.payment_method.code.clone().into(),
                self.payment_method.description.clone().into(),
                self.payment_method.state.unwrap_or(PaymentMethodState::Active).to_string().into(),
                now.to_string().into(),
                now.to_string().into(),
            ])
            .to_string(SqliteQueryBuilder);

        // Execute the query
        service.db_adapter.execute(&query, vec![])?;

        // Return the newly created payment method
        Ok(new_payment_method)
    }
}

impl Command for UpdatePaymentMethodCommand {
    type Output = PaymentMethod;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Check if the payment method exists
        let check_query = Query::select()
            .from(PaymentMethods::Table)
            .columns([
                PaymentMethods::Id,
                PaymentMethods::Name,
                PaymentMethods::Code,
                PaymentMethods::Description,
                PaymentMethods::State,
                PaymentMethods::CreatedAt,
                PaymentMethods::UpdatedAt,
            ])
            .and_where(Expr::col(PaymentMethods::Id).eq(self.payment_method.id.to_string()))
            .to_string(SqliteQueryBuilder);

        let existing = service.db_adapter.query_optional::<PaymentMethod>(&check_query, vec![])?;

        if existing.is_none() {
            return Err(Error::NotFoundError);
        }

        // Check if we're trying to update the code and it already exists
        if let Some(code) = &self.payment_method.code {
            let duplicate_check_query = Query::select()
                .from(PaymentMethods::Table)
                .columns([PaymentMethods::Id])
                .and_where(Expr::col(PaymentMethods::Code).eq(code))
                .and_where(Expr::col(PaymentMethods::Id).ne(self.payment_method.id.to_string()))
                .to_string(SqliteQueryBuilder);

            let duplicate = service.db_adapter.query_optional::<DbUuid>(&duplicate_check_query, vec![])?;

            if duplicate.is_some() {
                return Err(Error::UniqueConstraintError);
            }
        }

        let now = Utc::now().naive_utc();

        // Build the update query with SeaQuery
        let mut update_query = Query::update();
        let query = update_query.table(PaymentMethods::Table);

        // Only set fields that are provided in the update input
        if let Some(name) = &self.payment_method.name {
            query.value(PaymentMethods::Name, name.clone());
        }

        if let Some(code) = &self.payment_method.code {
            query.value(PaymentMethods::Code, code.clone());
        }

        if let Some(description) = &self.payment_method.description {
            match description {
                Some(desc) => query.value(PaymentMethods::Description, desc.clone()),
                None => query.value(PaymentMethods::Description, sea_query::Value::String(None)),
            };
        }

        if let Some(state) = &self.payment_method.state {
            query.value(PaymentMethods::State, state.to_string());
        }

        // Always update the updated_at timestamp
        query.value(PaymentMethods::UpdatedAt, now.to_string());

        // Add the WHERE clause
        query.and_where(Expr::col(PaymentMethods::Id).eq(self.payment_method.id.to_string()));

        let sql = query.to_string(SqliteQueryBuilder);

        // Execute the query
        service.db_adapter.execute(&sql, vec![])?;

        // Get the updated payment method
        let updated = service.db_adapter.query_one::<PaymentMethod>(&check_query, vec![])?;

        Ok(updated)
    }
}

impl Command for DeletePaymentMethodCommand {
    type Output = usize;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Check if the payment method exists
        let check_query = Query::select()
            .from(PaymentMethods::Table)
            .columns([PaymentMethods::Id])
            .and_where(Expr::col(PaymentMethods::Id).eq(self.id.to_string()))
            .to_string(SqliteQueryBuilder);

        let existing = service.db_adapter.query_optional::<DbUuid>(&check_query, vec![])?;

        if existing.is_none() {
            return Err(Error::NotFoundError);
        }

        // NOTE: This code is currently commented out because the sales_order_payments
        // table migrations have been created but are not yet applied in the test environment.
        // When the sales_order_payments table is actually implemented and the migrations
        // are applied in production, this code should be uncommented.

        /*
        // Check if payment method is used in any sales_order_payments
        let count_query = Query::select()
            .from(SalesOrderPayments::Table)
            .expr_as(Expr::col(SalesOrderPayments::Id).count(), Alias::new("count"))
            .and_where(Expr::col(SalesOrderPayments::PaymentMethodId).eq(self.id.to_string()))
            .to_string(SqliteQueryBuilder);

        let payment_method_in_use = service.db_adapter.query_one::<i64>(&count_query, vec![])?;

        if payment_method_in_use > 0 {
            return Err(Error::ValidationError(
                format!("Cannot delete payment method because it is used in {} sales", payment_method_in_use)
            ));
        }
        */

        // Build the delete query with SeaQuery
        let query = Query::delete()
            .from_table(PaymentMethods::Table)
            .and_where(Expr::col(PaymentMethods::Id).eq(self.id.to_string()))
            .to_string(SqliteQueryBuilder);

        // Execute the query
        let affected_rows = service.db_adapter.execute(&query, vec![])?;

        Ok(affected_rows as usize)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::commands::tests::setup_service;

    use super::*;

    fn setup_test_db(service: &mut AppService) {
        // Create payment_methods table for testing
        let create_table_sql = "CREATE TABLE IF NOT EXISTS payment_methods (
            id TEXT PRIMARY KEY NOT NULL,
            name TEXT NOT NULL,
            code TEXT NOT NULL UNIQUE,
            description TEXT,
            state TEXT NOT NULL DEFAULT 'Active',
            created_at TIMESTAMP NOT NULL,
            updated_at TIMESTAMP NOT NULL
        )";

        service.db_adapter.execute(create_table_sql, vec![]).unwrap();
    }

    #[test]
    fn test_create_payment_method() {
        let mut service = setup_service();
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
        let mut service = setup_service();
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
        let mut service = setup_service();
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
        let mut service = setup_service();
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
        let mut service = setup_service();
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
        let mut service = setup_service();
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
        let mut service = setup_service();
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
        let mut service = setup_service();
        setup_test_db(&mut service);

        let delete_cmd = DeletePaymentMethodCommand {
            id: Uuid::now_v7().into(),
        };
        let result = delete_cmd.exec(&mut service);

        assert!(result.is_err());
    }
}
