use std::io::Error;

use crate::core::{
    common::repository::JoinEntities,
    entities::catalog::{
        item::model::Item,
        item_category::model::{ItemCategory, ItemCategoryRelation},
    },
};

#[cfg_attr(test, mockall::automock)]
pub trait ItemCategoryRepository {
    // Read with flexible includes
    fn get_many(
        &self,
        with: JoinEntities<ItemCategoryRelation>,
    ) -> Result<Vec<ItemCategory>, Error>;

    fn get_one_by_id(
        &self,
        id: &str,
        with: JoinEntities<ItemCategoryRelation>,
    ) -> Result<ItemCategory, Error>;

    // Write
    fn insert(&self, entity: &ItemCategory) -> Result<ItemCategory, Error>;
    fn update(&self, entity: &ItemCategory) -> Result<ItemCategory, Error>;
    fn delete(&self, id: &str) -> Result<bool, Error>;

    // Items
    fn has_items(&self, id: &str) -> Result<bool, Error>;
    fn add_item(&self, item: &Item) -> Result<Item, Error>;

    // Validation
    fn is_name_taken(&self, name: &str) -> Result<bool, Error>;
}
