use std::io::Error;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::core::app::catalog::item::app::ItemUseCase;
use crate::core::common::interface::sql::MockSQLInterface;
use crate::core::entities::catalog::item::{ItemNature, ItemState};
use crate::core::entities::catalog::item_category::{ItemCategory, ItemCategoryState};
use crate::core::{app::app_service::AppService, entities::catalog::item::Item};
use mockall::predicate::*;

#[test]
fn test_create_item() -> Result<(), Error> {
    let mut mock = MockSQLInterface::new();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let item = Item {
        id: "1".to_string(),
        name: "Test Item".to_string(),
        description: None,
        nature: ItemNature::Goods,
        category_id: "1".to_string(),
        state: ItemState::Active,
        created_at: now,
        updated_at: now,
    };

    let item_clone = item.clone();

    // Mock category exists check
    mock.expect_get_one::<ItemCategory>()
        .with(always(), always())
        .returning(move |_, _| {
            Some(ItemCategory {
                id: "1".to_string(),
                name: "Test Category".to_string(),
                description: None,
                state: ItemCategoryState::Active,
                created_at: now,
                updated_at: now,
            })
        });

    // Mock item doesn't exist check
    mock.expect_get_one::<Item>()
        .with(always(), always())
        .returning(|_, _| None);

    // Mock save operation
    mock.expect_save::<Item>()
        .with(always())
        .returning(move |_| Ok(item_clone.clone()));

    let app_service = AppService::new(mock);
    let result = app_service.create_item(&item);
    assert!(result.is_ok());
    Ok(())
}

#[test]
fn test_create_item_already_exists() -> Result<(), Error> {
    let mut mock = MockSQLInterface::new();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let item = Item {
        id: "1".to_string(),
        name: "Test Item".to_string(),
        description: None,
        nature: ItemNature::Goods,
        category_id: "1".to_string(),
        state: ItemState::Active,
        created_at: now,
        updated_at: now,
    };

    let item_clone = item.clone();

    // Mock category exists check
    mock.expect_get_one::<ItemCategory>()
        .with(always(), always())
        .returning(move |_, _| {
            Some(ItemCategory {
                id: "1".to_string(),
                name: "Test Category".to_string(),
                description: None,
                state: ItemCategoryState::Active,
                created_at: now,
                updated_at: now,
            })
        });

    // Mock item already exists check
    mock.expect_get_one::<Item>()
        .with(always(), always())
        .returning(move |_, _| Some(item.clone()));

    let app_service = AppService::new(mock);
    let result = app_service.create_item(&item_clone);
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_update_item() -> Result<(), Error> {
    let mut mock = MockSQLInterface::new();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let item = Item {
        id: "1".to_string(),
        name: "Test Item".to_string(),
        description: None,
        nature: ItemNature::Goods,
        category_id: "1".to_string(),
        state: ItemState::Active,
        created_at: now,
        updated_at: now,
    };

    let updated_item = Item {
        name: "Updated Item".to_string(),
        ..item.clone()
    };

    let updated_item_clone = updated_item.clone();

    // Mock category exists check
    mock.expect_get_one::<ItemCategory>()
        .with(always(), always())
        .returning(move |_, _| {
            Some(ItemCategory {
                id: "1".to_string(),
                name: "Test Category".to_string(),
                description: None,
                state: ItemCategoryState::Active,
                created_at: now,
                updated_at: now,
            })
        });

    // Mock item exists check
    mock.expect_get_one::<Item>()
        .with(always(), always())
        .returning(move |_, _| Some(item.clone()));

    // Mock save operation
    mock.expect_save::<Item>()
        .with(always())
        .returning(move |_| Ok(updated_item_clone.clone()));

    let app_service = AppService::new(mock);
    let result = app_service.update_item(&updated_item);
    assert!(result.is_ok());
    Ok(())
}

#[test]
fn test_update_nonexistent_item() -> Result<(), Error> {
    let mut mock = MockSQLInterface::new();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let item = Item {
        id: "nonexistent".to_string(),
        name: "Test Item".to_string(),
        description: None,
        nature: ItemNature::Goods,
        category_id: "1".to_string(),
        state: ItemState::Active,
        created_at: now,
        updated_at: now,
    };

    // Mock category exists check
    mock.expect_get_one::<ItemCategory>()
        .with(always(), always())
        .returning(move |_, _| {
            Some(ItemCategory {
                id: "1".to_string(),
                name: "Test Category".to_string(),
                description: None,
                state: ItemCategoryState::Active,
                created_at: now,
                updated_at: now,
            })
        });

    // Mock item doesn't exist check
    mock.expect_get_one::<Item>()
        .with(always(), always())
        .returning(|_, _| None);

    let app_service = AppService::new(mock);
    let result = app_service.update_item(&item);
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_delete_item() -> Result<(), Error> {
    let mut mock = MockSQLInterface::new();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let item = Item {
        id: "1".to_string(),
        name: "Test Item".to_string(),
        description: None,
        nature: ItemNature::Goods,
        category_id: "1".to_string(),
        state: ItemState::Active,
        created_at: now,
        updated_at: now,
    };

    let item_clone = item.clone();

    // Mock item exists check
    mock.expect_get_one::<Item>()
        .with(always(), always())
        .returning(move |_, _| Some(item.clone()));

    // Mock delete operation
    mock.expect_delete::<Item>()
        .with(always())
        .returning(|_| Ok(true));

    let app_service = AppService::new(mock);
    let result = app_service.delete_item(&item_clone);
    assert!(result.is_ok());
    Ok(())
}
