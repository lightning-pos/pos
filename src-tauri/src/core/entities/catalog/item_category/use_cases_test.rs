use mockall::{mock, predicate};
use std::io::Error;

use crate::core::{
    common::repository::{JoinEntities, QueryRepository},
    entities::catalog::{
        catalog_service::CatalogService,
        item::model::{Item, ItemRelation},
        item::repository::ItemRepository,
        item_category::{
            model::{ItemCategory, ItemCategoryRelation, ItemCategoryState},
            repository::ItemCategoryRepository,
            use_cases::ItemCategoryUseCase,
        },
    },
};

mock! {
    pub ItemCategoryRepo {}

    impl QueryRepository<ItemCategory, ItemCategoryRelation> for ItemCategoryRepo {
        fn get_many(&self, with: JoinEntities<ItemCategoryRelation>) -> Result<Vec<ItemCategory>, Error> {
            Ok(vec![])
        }

        fn get_one_by_id(&self, id: &str, with: JoinEntities<ItemCategoryRelation>) -> Result<ItemCategory, Error> {
            Ok(ItemCategory {
                id: id.to_string(),
                name: "Test".to_string(),
                description: None,
                state: Default::default(),
            })
        }
    }

    impl ItemCategoryRepository for ItemCategoryRepo {
        fn is_name_taken(&self, name: &str) -> Result<bool, Error>;
        fn insert(&self, entity: &ItemCategory) -> Result<ItemCategory, Error>;
        fn update(&self, entity: &ItemCategory) -> Result<ItemCategory, Error>;
        fn delete(&self, id: &str) -> Result<bool, Error>;
        fn has_items(&self, id: &str) -> Result<bool, Error>;
        fn add_item(&self, item: &Item) -> Result<Item, Error>;
    }
}

mock! {
    pub ItemRepo {}

    impl QueryRepository<Item, ItemRelation> for ItemRepo {
        fn get_many(&self, with: JoinEntities<ItemRelation>) -> Result<Vec<Item>, Error>;
        fn get_one_by_id(&self, id: &str, with: JoinEntities<ItemRelation>) -> Result<Item, Error>;
    }

    impl ItemRepository for ItemRepo {
        fn insert(&self, item: &Item) -> Result<Item, Error>;
        fn update(&self, item: &Item) -> Result<Item, Error>;
        fn delete(&self, id: &str) -> Result<bool, Error>;
    }
}

#[test]
fn test_create_item_category() {
    let mut mock_category_repo = MockItemCategoryRepo::new();

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
        item_category: &mock_category_repo,
        item: &MockItemRepo::new(),
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
    let mut mock_category_repo = MockItemCategoryRepo::new();

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
        item_category: &mock_category_repo,
        item: &MockItemRepo::new(),
    };

    let result = catalog_service.create_item_category(&item_category);
    assert_eq!(result.is_err(), true);
}

#[test]
fn test_update_item_category() {
    let mut mock_category_repo = MockItemCategoryRepo::new();

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
        item_category: &mock_category_repo,
        item: &MockItemRepo::new(),
    };

    let result = catalog_service.update_item_category(&item_category);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_update_item_category_already_exists() {
    let mut mock_category_repo = MockItemCategoryRepo::new();

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
        item_category: &mock_category_repo,
        item: &MockItemRepo::new(),
    };

    let result = catalog_service.update_item_category(&item_category);
    assert_eq!(result.is_err(), true);
}

#[test]
fn test_delete_item_category_with_items() {
    let mut mock_category_repo = MockItemCategoryRepo::new();

    mock_category_repo
        .expect_has_items()
        .with(predicate::always())
        .times(1)
        .returning(|_| Ok(true));

    let catalog_service = CatalogService {
        item_category: &mock_category_repo,
        item: &MockItemRepo::new(),
    };

    let result = catalog_service.delete_item_category("1");
    assert_eq!(result.is_err(), true);
}

#[test]
fn test_delete_item_category_without_items() {
    let mut mock_category_repo = MockItemCategoryRepo::new();

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
        item_category: &mock_category_repo,
        item: &MockItemRepo::new(),
    };

    let result = catalog_service.delete_item_category("1");
    assert_eq!(result.is_ok(), true);
}
