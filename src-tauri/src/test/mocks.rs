use mockall::mock;
use std::io::Error;

use crate::core::{
    common::interface::sql::query::{JoinEntities, QueryInterface},
    entities::catalog::{
        item::{
            interface::ItemInterface,
            model::{Item, ItemRelation},
        },
        item_category::{
            interface::ItemCategoryInterface,
            model::{ItemCategory, ItemCategoryRelation},
        },
    },
};

mock! {
    pub ItemCategoryRepo {}

    impl QueryInterface<ItemCategory, ItemCategoryRelation> for ItemCategoryRepo {
        fn get_many(&self, with: JoinEntities<ItemCategoryRelation>) -> Result<Vec<ItemCategory>, Error>;
        fn get_one_by_id(&self, id: &str, with: JoinEntities<ItemCategoryRelation>) -> Result<ItemCategory, Error>;
    }

    impl ItemCategoryInterface for ItemCategoryRepo {
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

    impl QueryInterface<Item, ItemRelation> for ItemRepo {
        fn get_many(&self, with: JoinEntities<ItemRelation>) -> Result<Vec<Item>, Error>;
        fn get_one_by_id(&self, id: &str, with: JoinEntities<ItemRelation>) -> Result<Item, Error>;
    }

    impl ItemInterface for ItemRepo {
        fn insert(&self, item: &Item) -> Result<Item, Error>;
        fn update(&self, item: &Item) -> Result<Item, Error>;
        fn delete(&self, id: &str) -> Result<bool, Error>;
    }
}
