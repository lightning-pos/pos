use crate::core::app::catalog::item_category::app::ItemCategoryUseCase;
use crate::core::common::interface::sql::MockSQLInterface;
use crate::core::entities::catalog::item::Item;
use crate::core::{
    app::app_service::AppService,
    entities::catalog::item_category::{ItemCategory, ItemCategoryState},
};
use mockall::predicate::*;
use std::io::Error;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn test_create_item_category() -> Result<(), Error> {
    let mut mock = MockSQLInterface::new();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let item_category = ItemCategory {
        id: "1".to_string(),
        name: "Test Category".to_string(),
        state: ItemCategoryState::Active,
        description: Some("Test Description".to_string()),
        created_at: now,
        updated_at: now,
    };

    let item_category_clone = item_category.clone();

    // Mock category doesn't exist check
    mock.expect_get_one::<ItemCategory>()
        .with(always(), always())
        .returning(|_, _| None);

    // Mock save operation
    mock.expect_save::<ItemCategory>()
        .with(always())
        .returning(move |_| Ok(item_category.clone()));

    let app_service = AppService::new(mock);
    let result = app_service.create_item_category(&item_category_clone);
    assert!(result.is_ok());
    Ok(())
}

#[test]
fn test_create_item_category_already_exists() -> Result<(), Error> {
    let mut mock = MockSQLInterface::new();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let item_category = ItemCategory {
        id: "1".to_string(),
        name: "Test Category".to_string(),
        state: ItemCategoryState::Active,
        description: Some("Test Description".to_string()),
        created_at: now,
        updated_at: now,
    };

    let item_category_clone = item_category.clone();

    // Mock category already exists check
    mock.expect_get_one::<ItemCategory>()
        .with(always(), always())
        .returning(move |_, _| Some(item_category.clone()));

    let app_service = AppService::new(mock);
    let result = app_service.create_item_category(&item_category_clone);
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_update_item_category() -> Result<(), Error> {
    let mut mock = MockSQLInterface::new();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let item_category = ItemCategory {
        id: "1".to_string(),
        name: "Test Category".to_string(),
        state: ItemCategoryState::Active,
        description: Some("Test Description".to_string()),
        created_at: now,
        updated_at: now,
    };

    let updated_category = ItemCategory {
        name: "Updated Category".to_string(),
        ..item_category.clone()
    };

    let updated_category_clone = updated_category.clone();

    // Mock category exists check
    mock.expect_get_one::<ItemCategory>()
        .with(always(), always())
        .returning(move |_, _| Some(item_category.clone()));

    // Mock save operation
    mock.expect_save::<ItemCategory>()
        .with(always())
        .returning(move |_| Ok(updated_category_clone.clone()));

    let app_service = AppService::new(mock);
    let result = app_service.update_item_category(&updated_category);
    assert!(result.is_ok());
    Ok(())
}

#[test]
fn test_update_nonexistent_item_category() -> Result<(), Error> {
    let mut mock = MockSQLInterface::new();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let item_category = ItemCategory {
        id: "nonexistent".to_string(),
        name: "Test Category".to_string(),
        state: ItemCategoryState::Active,
        description: Some("Test Description".to_string()),
        created_at: now,
        updated_at: now,
    };

    let item_category_clone = item_category.clone();

    // Mock category doesn't exist check
    mock.expect_get_one::<ItemCategory>()
        .with(always(), always())
        .returning(|_, _| None);

    let app_service = AppService::new(mock);
    let result = app_service.update_item_category(&item_category_clone);
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_delete_item_category_with_items() -> Result<(), Error> {
    let mut mock = MockSQLInterface::new();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let item_category = ItemCategory {
        id: "1".to_string(),
        name: "Test Category".to_string(),
        state: ItemCategoryState::Active,
        description: Some("Test Description".to_string()),
        created_at: now,
        updated_at: now,
    };

    let item_category_clone = item_category.clone();

    // Mock category exists check
    mock.expect_get_one::<ItemCategory>()
        .with(always(), always())
        .returning(move |_, _| Some(item_category.clone()));

    // Mock items check - return some items to prevent deletion
    mock.expect_get_many::<Item>()
        .with(always(), always())
        .returning(move |_, _| {
            vec![Item {
                id: "1".to_string(),
                name: "Test Item".to_string(),
                description: None,
                nature: crate::core::entities::catalog::item::ItemNature::Goods,
                category_id: "1".to_string(),
                state: crate::core::entities::catalog::item::ItemState::Active,
                created_at: now,
                updated_at: now,
            }]
        });

    let app_service = AppService::new(mock);
    let result = app_service.delete_item_category(&item_category_clone);
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_delete_item_category_without_items() -> Result<(), Error> {
    let mut mock = MockSQLInterface::new();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let item_category = ItemCategory {
        id: "1".to_string(),
        name: "Test Category".to_string(),
        state: ItemCategoryState::Active,
        description: Some("Test Description".to_string()),
        created_at: now,
        updated_at: now,
    };

    let item_category_clone = item_category.clone();

    // Mock category exists check
    mock.expect_get_one::<ItemCategory>()
        .with(always(), always())
        .returning(move |_, _| Some(item_category.clone()));

    // Mock items check - return empty vector to allow deletion
    mock.expect_get_many::<Item>()
        .with(always(), always())
        .returning(|_, _| vec![]);

    // Mock delete operation
    mock.expect_delete::<ItemCategory>()
        .with(always())
        .returning(|_| Ok(true));

    let app_service = AppService::new(mock);
    let result = app_service.delete_item_category(&item_category_clone);
    assert!(result.is_ok());
    Ok(())
}
