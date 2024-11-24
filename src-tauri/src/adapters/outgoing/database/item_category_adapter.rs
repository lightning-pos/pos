use std::io::Error;

use crate::core::{
    common::repository::{JoinEntities, QueryRepository},
    entities::catalog::{
        item::model::Item,
        item_category::{
            model::{ItemCategory, ItemCategoryRelation},
            repository::ItemCategoryRepository,
        },
    },
};

pub struct ItemCategoryAdapter;

impl QueryRepository<ItemCategory, ItemCategoryRelation> for ItemCategoryAdapter {
    fn get_many(
        &self,
        _with: JoinEntities<ItemCategoryRelation>,
    ) -> Result<Vec<ItemCategory>, Error> {
        unimplemented!()
    }

    fn get_one_by_id(
        &self,
        _id: &str,
        _with: JoinEntities<ItemCategoryRelation>,
    ) -> Result<ItemCategory, Error> {
        unimplemented!()
    }
}

impl ItemCategoryRepository for ItemCategoryAdapter {
    fn is_name_taken(&self, _name: &str) -> Result<bool, Error> {
        unimplemented!()
    }

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
}
