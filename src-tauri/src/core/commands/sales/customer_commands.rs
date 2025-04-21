use chrono::Utc;
use sea_query::{Expr, Query};
use uuid::Uuid;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        commands::{app_service::AppService, Command},
        models::sales::customer_model::{
            Customer, CustomerNewInput, CustomerUpdateInput, Customers,
        },
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
};

// Commands
pub struct CreateCustomerCommand {
    pub customer: CustomerNewInput,
}

pub struct UpdateCustomerCommand {
    pub customer: CustomerUpdateInput,
}

pub struct DeleteCustomerCommand {
    pub id: DbUuid,
}

// Command Implementations
impl Command for CreateCustomerCommand {
    type Output = Customer;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let now = Utc::now().naive_utc();
        let new_id = Uuid::now_v7();

        let new_customer = Customer {
            id: new_id.into(),
            full_name: self.customer.full_name.clone(),
            email: self.customer.email.clone(),
            phone: self.customer.phone.clone(),
            address: self.customer.address.clone(),
            created_at: now,
            updated_at: now,
        };

        // Build the insert query with SeaQuery
        let mut insert_query = Query::insert();
        let insert_stmt = insert_query
            .into_table(Customers::Table)
            .columns([
                Customers::Id,
                Customers::FullName,
                Customers::Email,
                Customers::Phone,
                Customers::Address,
                Customers::CreatedAt,
                Customers::UpdatedAt,
            ])
            .values_panic([
                new_id.to_string().into(),
                self.customer.full_name.clone().into(),
                self.customer.email.clone().into(),
                self.customer.phone.clone().into(),
                self.customer.address.clone().into(),
                now.to_string().into(),
                now.to_string().into(),
            ]);

        // Execute the query
        service.db_adapter.insert_one(&insert_stmt).await?;

        // Return the newly created customer
        Ok(new_customer)
    }
}

impl Command for UpdateCustomerCommand {
    type Output = Customer;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let now = Utc::now().naive_utc();
        let customer_id = self.customer.id;

        // First, check if the customer exists
        let mut select_query = Query::select();
        let select_stmt = select_query
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
            .and_where(Expr::col(Customers::Id).eq(customer_id.to_string()));

        let existing = service.db_adapter.query_optional::<Customer>(&select_stmt).await?;

        if existing.is_none() {
            return Err(Error::NotFoundError);
        }

        // Build the update query with SeaQuery
        let mut update_query = Query::update();
        let update_stmt = update_query.table(Customers::Table);

        // Only set fields that are provided in the update input
        if let Some(full_name) = &self.customer.full_name {
            update_stmt.value(Customers::FullName, full_name.clone());
        }

        if let Some(email) = &self.customer.email {
            match email {
                Some(e) => update_stmt.value(Customers::Email, e.clone()),
                None => update_stmt.value(Customers::Email, sea_query::Value::String(None)),
            };
        }

        if let Some(phone) = &self.customer.phone {
            match phone {
                Some(p) => update_stmt.value(Customers::Phone, p.clone()),
                None => update_stmt.value(Customers::Phone, sea_query::Value::String(None)),
            };
        }

        if let Some(address) = &self.customer.address {
            match address {
                Some(a) => update_stmt.value(Customers::Address, a.clone()),
                None => update_stmt.value(Customers::Address, sea_query::Value::String(None)),
            };
        }

        // Always update the updated_at timestamp
        update_stmt.value(Customers::UpdatedAt, now.to_string());

        // Add the WHERE clause
        update_stmt.and_where(Expr::col(Customers::Id).eq(customer_id.to_string()));

        // Execute the query
        service.db_adapter.update_one(&update_stmt).await?;

        // Get the updated customer
        let updated_customer = service.db_adapter.query_one::<Customer>(&select_stmt).await?;

        Ok(updated_customer)
    }
}

impl Command for DeleteCustomerCommand {
    type Output = i32;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Build the delete query with SeaQuery
        let mut delete_query = Query::delete();
        let delete_stmt = delete_query
            .from_table(Customers::Table)
            .and_where(Expr::col(Customers::Id).eq(self.id.to_string()));

        // Execute the query
        let affected_rows = service.db_adapter.delete(&delete_stmt).await?;

        if affected_rows == 0 {
            return Err(Error::NotFoundError);
        }

        Ok(affected_rows as i32)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::commands::tests::setup_service;

    use super::*;
    use uuid::Uuid;
    use sea_query::{Alias, Expr, Query};
    use tokio;

    #[tokio::test]
    async fn test_create_customer() {
        let mut app_service = setup_service();
        let new_customer = CustomerNewInput {
            full_name: String::from("John Doe"),
            email: Some(String::from("john@example.com")),
            phone: Some(String::from("+1234567890")),
            address: Some(String::from("123 Main St")),
        };
        let command = CreateCustomerCommand {
            customer: new_customer,
        };
        let result = command.exec(&mut app_service).await;

        assert!(result.is_ok());
        let customer = result.unwrap();
        assert_eq!(customer.full_name, "John Doe");
        assert_eq!(customer.email, Some("john@example.com".to_string()));
    }

    #[tokio::test]
    async fn test_update_customer() {
        let mut app_service = setup_service();
        let new_customer = CustomerNewInput {
            full_name: String::from("John Doe"),
            email: Some(String::from("john@example.com")),
            phone: Some(String::from("+1234567890")),
            address: Some(String::from("123 Main St")),
        };

        let create_command = CreateCustomerCommand {
            customer: new_customer,
        };
        let customer = create_command.exec(&mut app_service).await.unwrap();

        let updated_customer = CustomerUpdateInput {
            id: customer.id,
            full_name: Some(String::from("John Smith")),
            email: Some(None),
            phone: None,
            address: Some(Some(String::from("456 Oak Ave"))),
        };

        let update_command = UpdateCustomerCommand {
            customer: updated_customer,
        };
        let result = update_command.exec(&mut app_service).await;
        assert!(result.is_ok());
        let updated = result.unwrap();
        assert_eq!(updated.full_name, "John Smith");
        assert_eq!(updated.email, None);
        assert_eq!(updated.address, Some("456 Oak Ave".to_string()));
    }

    #[tokio::test]
    async fn test_update_customer_does_not_exist() {
        let mut app_service = setup_service();
        let customer = CustomerUpdateInput {
            id: Uuid::now_v7().into(),
            full_name: Some(String::from("John Smith")),
            email: None,
            phone: None,
            address: None,
        };

        let command = UpdateCustomerCommand { customer };
        let result = command.exec(&mut app_service).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_customer() {
        let mut app_service = setup_service();
        let new_customer = CustomerNewInput {
            full_name: String::from("John Doe"),
            email: Some(String::from("john@example.com")),
            phone: Some(String::from("+1234567890")),
            address: Some(String::from("123 Main St")),
        };

        let create_command = CreateCustomerCommand {
            customer: new_customer,
        };
        let customer = create_command.exec(&mut app_service).await.unwrap();

        let delete_command = DeleteCustomerCommand { id: customer.id };
        let result = delete_command.exec(&mut app_service).await;
        assert!(result.is_ok());

        // Verify customer no longer exists
        let mut count_query = Query::select();
        let count_stmt = count_query
            .from(Customers::Table)
            .expr_as(Expr::col(Customers::Id).count(), Alias::new("count"))
            .and_where(Expr::col(Customers::Id).eq(customer.id.to_string()));

        let count = app_service.db_adapter.query_one::<i64>(&count_stmt).await.unwrap();
        assert_eq!(count, 0);
    }

    #[tokio::test]
    async fn test_delete_customer_does_not_exist() {
        let mut app_service = setup_service();
        let command = DeleteCustomerCommand {
            id: Uuid::now_v7().into(),
        };
        let result = command.exec(&mut app_service).await;
        assert!(result.is_err());
    }
}
