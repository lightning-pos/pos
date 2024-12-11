use std::io::Error;

use crate::core::{
    common::interface::sql::query::{join_entity::JoinEntities, query::QueryInterface},
    entities::catalog::{
        item::model::Item,
        item_category::{
            interface::ItemCategoryInterface,
            model::{ItemCategory, ItemCategoryRelation},
        },
    },
};

pub struct ItemCategoryAdapter;

impl QueryInterface<ItemCategory, ItemCategoryRelation> for ItemCategoryAdapter {
    fn get_many(
        &self,
        _with: JoinEntities<ItemCategoryRelation>,
    ) -> Result<Vec<ItemCategory>, Error> {
        unimplemented!()
    }

    fn get_one_by_id(
        &self,
        _id: &str,
        _with: crate::core::common::interface::sql::query::join_entity::JoinEntities<
            ItemCategoryRelation,
        >,
    ) -> Result<ItemCategory, Error> {
        unimplemented!()
    }
}

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
