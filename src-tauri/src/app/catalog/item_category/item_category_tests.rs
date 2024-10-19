use mockall::predicate;
use std::io::Error;

use crate::app::catalog::catalog_service::CatalogService;
use crate::app::catalog::item::item_model::MockItemRepository;
use crate::app::catalog::item_category::item_category_model::{
    ItemCategory, ItemCategoryState, ItemCategoryUseCase, MockItemCategoryRepository,
};

#[test]
fn test_create_item_category() {
    let mut mock_category_repo = MockItemCategoryRepository::new();

    mock_category_repo
        .expect_insert()
        .with(predicate::always())
        .times(1)
        .returning(|item_category| Ok(item_category.clone()));

    mock_category_repo
        .expect_is_name_taken()
        .with(predicate::always())
        .times(1)
        .returning(|_| Err(Error::new(std::io::ErrorKind::Other, "Error")));

    let service = CatalogService {
        item_category: mock_category_repo,
        item: MockItemRepository::new(),
    };

    let item_category = ItemCategory {
        id: "1".to_string(),
        name: "Test Category".to_string(),
        state: ItemCategoryState::Active,
        description: Some("Test Description".to_string()),
        items: None,
        created_at: 0,
        updated_at: 0,
    };

    let result = service.create_item_category(&item_category);
    assert!(result.is_ok());
}

#[test]
fn test_create_item_category_already_exists() {
    let mut mock_category_repo = MockItemCategoryRepository::new();

    let item_category = ItemCategory {
        id: "1".to_string(),
        name: "Test Category".to_string(),
        state: ItemCategoryState::Active,
        description: Some("Test Description".to_string()),
        items: None,
        created_at: 0,
        updated_at: 0,
    };

    mock_category_repo
        .expect_is_name_taken()
        .with(predicate::always())
        .times(1)
        .returning(|_| Ok(true));

    let catalog_service = CatalogService {
        item_category: mock_category_repo,
        item: MockItemRepository::new(),
    };

    let result = catalog_service.create_item_category(&item_category);
    assert_eq!(result.is_err(), true);
}

#[test]
fn test_update_item_category() {
    let mut mock_category_repo = MockItemCategoryRepository::new();

    let item_category = ItemCategory {
        id: "1".to_string(),
        name: "Test Category".to_string(),
        state: ItemCategoryState::Active,
        description: Some("Test Description".to_string()),
        items: None,
        created_at: 0,
        updated_at: 0,
    };

    mock_category_repo
        .expect_is_name_taken()
        .with(predicate::always())
        .times(1)
        .returning(|_| Err(Error::new(std::io::ErrorKind::Other, "Error")));

    mock_category_repo
        .expect_update()
        .with(predicate::always())
        .times(1)
        .returning(|item_category| Ok(item_category.clone()));

    let catalog_service = CatalogService {
        item_category: mock_category_repo,
        item: MockItemRepository::new(),
    };

    let result = catalog_service.update_item_category(&item_category);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_update_item_category_already_exists() {
    let mut mock_category_repo = MockItemCategoryRepository::new();

    let item_category = ItemCategory {
        id: "1".to_string(),
        name: "Test Category".to_string(),
        state: ItemCategoryState::Active,
        description: Some("Test Description".to_string()),
        items: None,
        created_at: 0,
        updated_at: 0,
    };

    mock_category_repo
        .expect_is_name_taken()
        .with(predicate::always())
        .times(1)
        .returning(|_| Ok(true));

    let catalog_service = CatalogService {
        item_category: mock_category_repo,
        item: MockItemRepository::new(),
    };

    let result = catalog_service.update_item_category(&item_category);
    assert_eq!(result.is_err(), true);
}

#[test]
fn test_delete_item_category_with_items() {
    let mut mock_category_repo = MockItemCategoryRepository::new();

    mock_category_repo
        .expect_has_items()
        .with(predicate::always())
        .times(1)
        .returning(|_| Ok(true));

    let catalog_service = CatalogService {
        item_category: mock_category_repo,
        item: MockItemRepository::new(),
    };

    let result = catalog_service.delete_item_category("1");
    assert_eq!(result.is_err(), true);
}

#[test]
fn test_delete_item_category_without_items() {
    let mut mock_category_repo = MockItemCategoryRepository::new();

    mock_category_repo
        .expect_has_items()
        .with(predicate::always())
        .times(1)
        .returning(|_| Ok(false));

    mock_category_repo
        .expect_delete()
        .with(predicate::always())
        .times(1)
        .returning(|_| Ok(true));

    let catalog_service = CatalogService {
        item_category: mock_category_repo,
        item: MockItemRepository::new(),
    };

    let result = catalog_service.delete_item_category("1");
    assert_eq!(result.is_ok(), true);
}
