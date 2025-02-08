use chrono::Utc;
use diesel::Connection;
use uuid::Uuid;

use crate::{
    core::{
        commands::{app_service::AppService, Command},
        models::sales::cart_model::{CartNewInput, CartUpdateInput},
    },
    error::Error,
    test_helpers::setup_test_db,
};

use super::cart_commands::{CreateCartCommand, DeleteCartCommand, UpdateCartCommand};

#[test]
fn test_create_cart_command() {
    let mut service = AppService {
        conn: setup_test_db(),
    };

    let customer_id = Uuid::now_v7().into();
    let cart_data = r#"{"items": []}"#.to_string();

    let command = CreateCartCommand {
        cart: CartNewInput {
            customer_id,
            cart_data: cart_data.clone(),
        },
    };

    let result = command.exec(&mut service).unwrap();

    assert_eq!(result.customer_id, customer_id);
    assert_eq!(result.cart_data, cart_data);
    assert!(result.created_at <= Utc::now().naive_utc());
    assert_eq!(result.created_at, result.updated_at);
}

#[test]
fn test_update_cart_command() {
    let mut service = AppService {
        conn: setup_test_db(),
    };

    // First create a cart
    let customer_id = Uuid::now_v7().into();
    let initial_cart_data = r#"{"items": []}"#.to_string();
    let create_command = CreateCartCommand {
        cart: CartNewInput {
            customer_id,
            cart_data: initial_cart_data,
        },
    };
    let created_cart = create_command.exec(&mut service).unwrap();

    // Then update it
    let updated_cart_data = r#"{"items": [{"id": "123", "quantity": 1}]}"#.to_string();
    let update_command = UpdateCartCommand {
        cart: CartUpdateInput {
            id: created_cart.id,
            cart_data: Some(updated_cart_data.clone()),
        },
    };

    let result = update_command.exec(&mut service).unwrap();

    assert_eq!(result.id, created_cart.id);
    assert_eq!(result.cart_data, updated_cart_data);
    assert!(result.updated_at > result.created_at);
}

#[test]
fn test_update_nonexistent_cart() {
    let mut service = AppService {
        conn: setup_test_db(),
    };

    let nonexistent_id = Uuid::now_v7().into();
    let command = UpdateCartCommand {
        cart: CartUpdateInput {
            id: nonexistent_id,
            cart_data: Some("{}".to_string()),
        },
    };

    let result = command.exec(&mut service);
    assert!(result.is_err());
}

#[test]
fn test_delete_cart_command() {
    let mut service = AppService {
        conn: setup_test_db(),
    };

    // First create a cart
    let customer_id = Uuid::now_v7().into();
    let create_command = CreateCartCommand {
        cart: CartNewInput {
            customer_id,
            cart_data: "{}".to_string(),
        },
    };
    let created_cart = create_command.exec(&mut service).unwrap();

    // Then delete it
    let delete_command = DeleteCartCommand {
        id: created_cart.id,
    };

    let result = delete_command.exec(&mut service).unwrap();
    assert_eq!(result, 1); // Should return 1 for successful deletion

    // Verify deletion by attempting to update (should fail)
    let update_command = UpdateCartCommand {
        cart: CartUpdateInput {
            id: created_cart.id,
            cart_data: Some("{}".to_string()),
        },
    };
    let update_result = update_command.exec(&mut service);
    assert!(update_result.is_err());
}

#[test]
fn test_delete_nonexistent_cart() {
    let mut service = AppService {
        conn: setup_test_db(),
    };

    let nonexistent_id = Uuid::now_v7().into();
    let command = DeleteCartCommand {
        id: nonexistent_id,
    };

    let result = command.exec(&mut service);
    assert!(matches!(result.unwrap_err(), Error::NotFoundError));
}
