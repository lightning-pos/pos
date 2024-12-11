use std::io::Error;

use crate::core::{
    common::interface::sql::query::query::QueryInterface,
    entities::catalog::{
        item::model::Item,
        item_category::model::{ItemCategory, ItemCategoryRelation},
    },
};

pub trait ItemCategoryInterface: QueryInterface<ItemCategory, ItemCategoryRelation> {
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
