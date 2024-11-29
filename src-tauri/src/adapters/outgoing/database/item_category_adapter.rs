use std::io::Error;

use crate::core::{
    common::interface::sql::query::{JoinEntities, QueryInterface},
    entities::catalog::{
        item::model::Item,
        item_category::{
            interface::ItemCategoryInterface,
            model::{ItemCategory, ItemCategoryRelation},
        },
    },
};

pub struct ItemCategoryAdapter;

// Automatically gets default QueryInterface implementation
impl QueryInterface<ItemCategory, ItemCategoryRelation> for ItemCategoryAdapter {}

// Implement only the specific ItemCategoryInterface methods
impl ItemCategoryInterface for ItemCategoryAdapter {
    fn insert(&self, _entity: &ItemCategory) -> Result<ItemCategory, Error> {
        unimplemented!()
    }

    fn update(&self, _entity: &ItemCategory) -> Result<ItemCategory, Error> {
        unimplemented!()
    }

    fn delete(&self, _id: &str) -> Result<bool, Error> {
        unimplemented!()
    }

    fn has_items(&self, _id: &str) -> Result<bool, Error> {
        unimplemented!()
    }

    fn add_item(&self, _item: &Item) -> Result<Item, Error> {
        unimplemented!()
    }

    fn is_name_taken(&self, _name: &str) -> Result<bool, Error> {
        unimplemented!()
    }
}
