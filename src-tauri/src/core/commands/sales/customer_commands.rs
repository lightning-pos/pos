use chrono::Utc;
use diesel::{Connection, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{
    core::{
        commands::{app_service::AppService, Command},
        models::sales::customer_model::{
            Customer, CustomerNewInput, CustomerUpdateChangeset, CustomerUpdateInput,
        },
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
    schema::customers,
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

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let now = Utc::now().naive_utc();
            let new_customer = Customer {
                id: Uuid::now_v7().into(),
                full_name: self.customer.full_name.clone(),
                email: self.customer.email.clone(),
                phone: self.customer.phone.clone(),
                address: self.customer.address.clone(),
                created_at: now,
                updated_at: now,
            };

            let res = diesel::insert_into(customers::table)
                .values(&new_customer)
                .returning(Customer::as_returning())
                .get_result(conn)?;

            Ok(res)
        })
    }
}

impl Command for UpdateCustomerCommand {
    type Output = Customer;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Verify customer exists
            customers::table
                .find(&self.customer.id)
                .select(Customer::as_select())
                .get_result::<Customer>(conn)?;

            let now = Utc::now().naive_utc();

            let changeset = CustomerUpdateChangeset {
                id: self.customer.id,
                full_name: self.customer.full_name.clone(),
                email: self.customer.email.clone(),
                phone: self.customer.phone.clone(),
                address: self.customer.address.clone(),
                updated_at: now,
            };

            let res = diesel::update(customers::table.find(&self.customer.id))
                .set(&changeset)
                .returning(Customer::as_returning())
                .get_result(conn)?;

            Ok(res)
        })
    }
}

impl Command for DeleteCustomerCommand {
    type Output = i32;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let result = diesel::delete(customers::table.find(&self.id)).execute(conn)?;

            if result == 0 {
                return Err(Error::NotFoundError);
            }

            Ok(result as i32)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_create_customer() {
        let mut app_service = AppService::new(":memory:");
        let new_customer = CustomerNewInput {
            full_name: String::from("John Doe"),
            email: Some(String::from("john@example.com")),
            phone: Some(String::from("+1234567890")),
            address: Some(String::from("123 Main St")),
        };
        let command = CreateCustomerCommand {
            customer: new_customer,
        };
        let result = command.exec(&mut app_service);

        assert!(result.is_ok());
        let customer = result.unwrap();
        assert_eq!(customer.full_name, "John Doe");
        assert_eq!(customer.email, Some("john@example.com".to_string()));
    }

    #[test]
    fn test_update_customer() {
        let mut app_service = AppService::new(":memory:");
        let new_customer = CustomerNewInput {
            full_name: String::from("John Doe"),
            email: Some(String::from("john@example.com")),
            phone: Some(String::from("+1234567890")),
            address: Some(String::from("123 Main St")),
        };

        let create_command = CreateCustomerCommand {
            customer: new_customer,
        };
        let customer = create_command.exec(&mut app_service).unwrap();

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
        let result = update_command.exec(&mut app_service);
        assert!(result.is_ok());
        let updated = result.unwrap();
        assert_eq!(updated.full_name, "John Smith");
        assert_eq!(updated.email, None);
        assert_eq!(updated.address, Some("456 Oak Ave".to_string()));
    }

    #[test]
    fn test_update_customer_does_not_exist() {
        let mut app_service = AppService::new(":memory:");
        let customer = CustomerUpdateInput {
            id: Uuid::now_v7().into(),
            full_name: Some(String::from("John Smith")),
            email: None,
            phone: None,
            address: None,
        };

        let command = UpdateCustomerCommand { customer };
        let result = command.exec(&mut app_service);
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_customer() {
        let mut app_service = AppService::new(":memory:");
        let new_customer = CustomerNewInput {
            full_name: String::from("John Doe"),
            email: Some(String::from("john@example.com")),
            phone: Some(String::from("+1234567890")),
            address: Some(String::from("123 Main St")),
        };

        let create_command = CreateCustomerCommand {
            customer: new_customer,
        };
        let customer = create_command.exec(&mut app_service).unwrap();

        let delete_command = DeleteCustomerCommand { id: customer.id };
        let result = delete_command.exec(&mut app_service);
        assert!(result.is_ok());
    }

    #[test]
    fn test_delete_customer_does_not_exist() {
        let mut app_service = AppService::new(":memory:");
        let command = DeleteCustomerCommand {
            id: Uuid::now_v7().into(),
        };
        let result = command.exec(&mut app_service);
        assert!(result.is_err());
    }
}
