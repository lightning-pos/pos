use chrono::Utc;
use diesel::{Connection, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{
    core::{
        commands::{app_service::AppService, Command},
        models::sales::cart_model::{Cart, CartNewInput, CartUpdateChangeset, CartUpdateInput},
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
    schema::carts,
};

// Commands
pub struct CreateCartCommand {
    pub cart: CartNewInput,
}

pub struct UpdateCartCommand {
    pub cart: CartUpdateInput,
}

pub struct DeleteCartCommand {
    pub id: DbUuid,
}

// Command Implementations
impl Command for CreateCartCommand {
    type Output = Cart;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let now = Utc::now().naive_utc();
            let new_cart = Cart {
                id: Uuid::now_v7().into(),
                customer_id: self.cart.customer_id,
                cart_data: self.cart.cart_data.clone(),
                created_at: now,
                updated_at: now,
            };

            let res = diesel::insert_into(carts::table)
                .values(&new_cart)
                .returning(Cart::as_returning())
                .get_result(conn)?;

            Ok(res)
        })
    }
}

impl Command for UpdateCartCommand {
    type Output = Cart;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Verify cart exists
            carts::table
                .find(&self.cart.id)
                .select(Cart::as_select())
                .get_result::<Cart>(conn)?;

            let now = Utc::now().naive_utc();

            let changeset = CartUpdateChangeset {
                id: self.cart.id,
                cart_data: self.cart.cart_data.clone(),
                updated_at: Some(now),
            };

            let res = diesel::update(carts::table.find(&self.cart.id))
                .set(&changeset)
                .returning(Cart::as_returning())
                .get_result(conn)?;

            Ok(res)
        })
    }
}

impl Command for DeleteCartCommand {
    type Output = i32;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let result = diesel::delete(carts::table.find(&self.id)).execute(conn)?;

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
    use chrono::Utc;
    use uuid::Uuid;
    use crate::core::{
        commands::sales::customer_commands::CreateCustomerCommand,
        models::sales::customer_model::CustomerNewInput,
    };
    use rand::Rng;

    fn create_test_customer(service: &mut AppService) -> DbUuid {
        let random_suffix = rand::thread_rng().gen_range(1000..9999).to_string();
        let command = CreateCustomerCommand {
            customer: CustomerNewInput {
                full_name: format!("Test Customer {}", random_suffix),
                email: Some(format!("test{}@example.com", random_suffix)),
                phone: Some(format!("+1234567{}", random_suffix)),
                address: None,
            },
        };
        command.exec(service).unwrap().id
    }

    #[test]
    fn test_create_cart_with_customer() {
        let mut app_service = AppService::new(":memory:");
        let customer_id = Some(create_test_customer(&mut app_service));
        let cart_data = r#"{"items": []}"#.to_string();

        let command = CreateCartCommand {
            cart: CartNewInput {
                customer_id,
                cart_data: cart_data.clone(),
            },
        };

        let result = command.exec(&mut app_service).unwrap();

        assert_eq!(result.customer_id, customer_id);
        assert_eq!(result.cart_data, cart_data);
        assert!(result.created_at <= Utc::now().naive_utc());
        assert_eq!(result.created_at, result.updated_at);
    }

    #[test]
    fn test_create_cart_without_customer() {
        let mut app_service = AppService::new(":memory:");
        let cart_data = r#"{"items": []}"#.to_string();

        let command = CreateCartCommand {
            cart: CartNewInput {
                customer_id: None,
                cart_data: cart_data.clone(),
            },
        };

        let result = command.exec(&mut app_service).unwrap();

        assert_eq!(result.customer_id, None);
        assert_eq!(result.cart_data, cart_data);
        assert!(result.created_at <= Utc::now().naive_utc());
        assert_eq!(result.created_at, result.updated_at);
    }

    #[test]
    fn test_update_cart() {
        let mut app_service = AppService::new(":memory:");

        // First create a cart without customer
        let initial_cart_data = r#"{"items": []}"#.to_string();
        let create_command = CreateCartCommand {
            cart: CartNewInput {
                customer_id: None,
                cart_data: initial_cart_data,
            },
        };
        let created_cart = create_command.exec(&mut app_service).unwrap();

        // Then update it
        let updated_cart_data = r#"{"items": [{"id": "123", "quantity": 1}]}"#.to_string();
        let update_command = UpdateCartCommand {
            cart: CartUpdateInput {
                id: created_cart.id,
                cart_data: Some(updated_cart_data.clone()),
            },
        };

        let result = update_command.exec(&mut app_service).unwrap();

        assert_eq!(result.id, created_cart.id);
        assert_eq!(result.cart_data, updated_cart_data);
        assert!(result.updated_at > result.created_at);
    }

    #[test]
    fn test_update_cart_does_not_exist() {
        let mut app_service = AppService::new(":memory:");

        let nonexistent_id = Uuid::now_v7().into();
        let command = UpdateCartCommand {
            cart: CartUpdateInput {
                id: nonexistent_id,
                cart_data: Some("{}".to_string()),
            },
        };

        let result = command.exec(&mut app_service);
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_cart() {
        let mut app_service = AppService::new(":memory:");

        // First create a cart
        let create_command = CreateCartCommand {
            cart: CartNewInput {
                customer_id: None,
                cart_data: "{}".to_string(),
            },
        };
        let created_cart = create_command.exec(&mut app_service).unwrap();

        // Then delete it
        let delete_command = DeleteCartCommand {
            id: created_cart.id,
        };

        let result = delete_command.exec(&mut app_service).unwrap();
        assert_eq!(result, 1); // Should return 1 for successful deletion

        // Verify deletion by attempting to update (should fail)
        let update_command = UpdateCartCommand {
            cart: CartUpdateInput {
                id: created_cart.id,
                cart_data: Some("{}".to_string()),
            },
        };
        let update_result = update_command.exec(&mut app_service);
        assert!(update_result.is_err());
    }

    #[test]
    fn test_delete_cart_does_not_exist() {
        let mut app_service = AppService::new(":memory:");

        let nonexistent_id = Uuid::now_v7().into();
        let command = DeleteCartCommand { id: nonexistent_id };

        let result = command.exec(&mut app_service);
        assert!(matches!(result.unwrap_err(), Error::NotFoundError));
    }
}
