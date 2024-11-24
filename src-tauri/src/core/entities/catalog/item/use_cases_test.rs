use mockall::{mock, predicate};
use std::io::Error;

use crate::core::{
    common::repository::{JoinEntities, QueryRepository},
    entities::catalog::{
        catalog_service::CatalogService,
        item::{
            model::{Item, ItemNature},
            repository::ItemRepository,
            use_cases::ItemUseCase,
        },
        item_category::{
            model::{ItemCategory, ItemCategoryRelation, ItemCategoryState},
            repository::ItemCategoryRepository,
        },
    },
};

use super::model::ItemRelation;

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
fn test_create_item() {
    let mut mock_item_repo = MockItemRepo::new();
    let mut mock_category_repo = MockItemCategoryRepo::new();

    mock_item_repo
        .expect_insert()
        .with(predicate::always())
        .times(1)
        .returning(|item| {
            Ok(Item {
                id: item.id.clone(),
                name: item.name.clone(),
                description: item.description.clone(),
                nature: item.nature.clone(),
                category_id: item.category_id.clone(),
                category: item.category.clone(),
                created_at: item.created_at,
                updated_at: item.updated_at,
            })
        });

    mock_category_repo
        .expect_get_one_by_id()
        .with(
            predicate::always(),
            predicate::function(|_: &JoinEntities<ItemCategoryRelation>| true),
        )
        .times(1)
        .returning(|_, _| {
            Ok(ItemCategory {
                id: "1".to_string(),
                name: "Category 1".to_string(),
                state: ItemCategoryState::Active,
                description: None,
                items: None,
                created_at: 0,
                updated_at: 0,
            })
        });

    let service = CatalogService {
        item_category: &mock_category_repo,
        item: &mock_item_repo,
    };

    let result = service.create_item(&Item {
        id: "1".to_string(),
        name: "Item 1".to_string(),
        description: None,
        nature: ItemNature::Goods,
        category_id: "1".to_string(),
        category: None,
        created_at: 0,
        updated_at: 0,
    });

    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_create_item_with_invalid_category_id() {
    let mock_item_repo = MockItemRepo::new();
    let mut mock_category_repo = MockItemCategoryRepo::new();

    mock_category_repo
        .expect_get_one_by_id()
        .with(
            predicate::always(),
            predicate::function(|_: &JoinEntities<ItemCategoryRelation>| true),
        )
        .times(1)
        .returning(|_, _| Err(Error::new(std::io::ErrorKind::Other, "")));

    let service = CatalogService {
        item_category: &mock_category_repo,
        item: &mock_item_repo,
    };

    let item = Item {
        id: "1".to_string(),
        name: "Item 1".to_string(),
        description: None,
        nature: ItemNature::Goods,
        category_id: "1".to_string(),
        category: None,
        created_at: 0,
        updated_at: 0,
    };

    let result = service.create_item(&item);

    assert_eq!(result.is_err(), true);
}

#[test]
fn test_update_item() {
    let mut mock_item_repo = MockItemRepo::new();
    let mut mock_category_repo = MockItemCategoryRepo::new();

    mock_item_repo
        .expect_update()
        .with(predicate::always())
        .times(1)
        .returning(|item| Ok(item.clone()));

    mock_category_repo
        .expect_get_one_by_id()
        .with(
            predicate::always(),
            predicate::function(|_: &JoinEntities<ItemCategoryRelation>| true),
        )
        .times(1)
        .returning(|_, _| {
            Ok(ItemCategory {
                id: "1".to_string(),
                name: "Category 1".to_string(),
                state: ItemCategoryState::Active,
                description: None,
                items: None,
                created_at: 0,
                updated_at: 0,
            })
        });

    let service = CatalogService {
        item_category: &mock_category_repo,
        item: &mock_item_repo,
    };

    let item = Item {
        id: "1".to_string(),
        name: "Item 1".to_string(),
        description: None,
        nature: ItemNature::Goods,
        category_id: "1".to_string(),
        category: None,
        created_at: 0,
        updated_at: 0,
    };

    let result = service.update_item(&item);

    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_update_item_with_invalid_category_id() {
    let mock_item_repo = MockItemRepo::new();
    let mut mock_category_repo = MockItemCategoryRepo::new();

    mock_category_repo
        .expect_get_one_by_id()
        .with(
            predicate::always(),
            predicate::function(|_: &JoinEntities<ItemCategoryRelation>| true),
        )
        .times(1)
        .returning(|_, _| Err(Error::new(std::io::ErrorKind::Other, "")));
    let service = CatalogService {
        item_category: &mock_category_repo,
        item: &mock_item_repo,
    };

    let item = Item {
        id: "1".to_string(),
        name: "Item 1".to_string(),
        description: None,
        nature: ItemNature::Goods,
        category_id: "1".to_string(),
        category: None,
        created_at: 0,
        updated_at: 0,
    };

    let result = service.update_item(&item);

    assert_eq!(result.is_err(), true);
}

#[test]
fn test_delete_item() {}
