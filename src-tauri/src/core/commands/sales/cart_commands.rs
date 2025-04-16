use chrono::Utc;
use sea_query::{Alias, Expr, Query, SqliteQueryBuilder};
use uuid::Uuid;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        commands::{app_service::AppService, Command},
        models::sales::cart_model::{Cart, CartNewInput, CartUpdateInput, Carts},
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
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
        let now = Utc::now().naive_utc();
        let new_id = Uuid::now_v7();

        let new_cart = Cart {
            id: new_id.into(),
            customer_id: self.cart.customer_id,
            cart_data: self.cart.cart_data.clone(),
            created_at: now,
            updated_at: now,
        };

        // Build the insert query with SeaQuery
        let query = Query::insert()
            .into_table(Carts::Table)
            .columns([
                Carts::Id,
                Carts::CustomerId,
                Carts::CartData,
                Carts::CreatedAt,
                Carts::UpdatedAt,
            ])
            .values_panic([
                new_id.to_string().into(),
                self.cart.customer_id.map(|id| id.to_string()).into(),
                self.cart.cart_data.clone().into(),
                now.to_string().into(),
                now.to_string().into(),
            ])
            .to_string(SqliteQueryBuilder);

        // Execute the query
        service.db_adapter.execute(&query, vec![])?;

        // Return the newly created cart
        Ok(new_cart)
    }
}

impl Command for UpdateCartCommand {
    type Output = Cart;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let now = Utc::now().naive_utc();
        let cart_id = self.cart.id;

        // First, check if the cart exists
        let check_query = Query::select()
            .from(Carts::Table)
            .columns([
                Carts::Id,
                Carts::CustomerId,
                Carts::CartData,
                Carts::CreatedAt,
                Carts::UpdatedAt,
            ])
            .and_where(Expr::col(Carts::Id).eq(cart_id.to_string()))
            .to_string(SqliteQueryBuilder);

        let existing = service.db_adapter.query_optional::<Cart>(&check_query, vec![])?;

        if existing.is_none() {
            return Err(Error::NotFoundError);
        }

        // Build the update query with SeaQuery
        let mut update_query = Query::update();
        let query = update_query.table(Carts::Table);

        // Only set fields that are provided in the update input
        if let Some(cart_data) = &self.cart.cart_data {
            query.value(Carts::CartData, cart_data.clone());
        }

        // Always update the updated_at timestamp
        query.value(Carts::UpdatedAt, now.to_string());

        // Add the WHERE clause
        query.and_where(Expr::col(Carts::Id).eq(cart_id.to_string()));

        let sql = query.to_string(SqliteQueryBuilder);

        // Execute the query
        service.db_adapter.execute(&sql, vec![])?;

        // Get the updated cart
        let updated_cart = service.db_adapter.query_one::<Cart>(&check_query, vec![])?;

        Ok(updated_cart)
    }
}

impl Command for DeleteCartCommand {
    type Output = i32;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Build the delete query with SeaQuery
        let query = Query::delete()
            .from_table(Carts::Table)
            .and_where(Expr::col(Carts::Id).eq(self.id.to_string()))
            .to_string(SqliteQueryBuilder);

        // Execute the query
        let affected_rows = service.db_adapter.execute(&query, vec![])?;

        if affected_rows == 0 {
            return Err(Error::NotFoundError);
        }

        Ok(affected_rows as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;
    use crate::core::{
        commands::{sales::customer_commands::CreateCustomerCommand, tests::setup_service},
        models::sales::customer_model::CustomerNewInput,
    };
    use rand::Rng;
    use sea_query::{Expr, Query, SqliteQueryBuilder};

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
        let mut app_service = setup_service();
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
        let mut app_service = setup_service();
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
        let mut app_service = setup_service();

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
        let mut app_service = setup_service();

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
        let mut app_service = setup_service();

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

        // Verify deletion by checking if cart exists
        let check_query = Query::select()
            .from(Carts::Table)
            .expr_as(Expr::col(Carts::Id).count(), Alias::new("count"))
            .and_where(Expr::col(Carts::Id).eq(created_cart.id.to_string()))
            .to_string(SqliteQueryBuilder);

        let count = app_service.db_adapter.query_one::<i64>(&check_query, vec![]).unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_delete_cart_does_not_exist() {
        let mut app_service = setup_service();

        let nonexistent_id = Uuid::now_v7().into();
        let command = DeleteCartCommand { id: nonexistent_id };

        let result = command.exec(&mut app_service);
        assert!(matches!(result.unwrap_err(), Error::NotFoundError));
    }
}
