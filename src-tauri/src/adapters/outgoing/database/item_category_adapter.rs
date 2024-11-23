use crate::core::{
    common::repository::JoinEntities,
    entities::catalog::{
        item::model::Item,
        item_category::{
            model::{ItemCategory, ItemCategoryRelation},
            repository::ItemCategoryRepository,
        },
    },
};
use std::io::Error;
struct ItemCategoryAdapter {}

impl ItemCategoryRepository for ItemCategoryAdapter {
    fn is_name_taken(&self, name: &str) -> Result<bool, Error> {
        unimplemented!()
    }

    fn get_many(
        &self,
        with: crate::core::common::repository::JoinEntities<
            crate::core::entities::catalog::item_category::model::ItemCategoryRelation,
        >,
    ) -> Result<Vec<ItemCategory>, Error> {
        unimplemented!()
    }

    fn get_one_by_id(
        &self,
        id: &str,
        with: JoinEntities<ItemCategoryRelation>,
    ) -> Result<ItemCategory, Error> {
        unimplemented!()
    }

    fn has_items(&self, id: &str) -> Result<bool, Error> {
        unimplemented!()
    }

    fn insert(&self, entity: &ItemCategory) -> Result<ItemCategory, Error> {
        unimplemented!()
    }

    fn update(&self, entity: &ItemCategory) -> Result<ItemCategory, Error> {
        unimplemented!()
    }

    fn delete(&self, id: &str) -> Result<bool, Error> {
        unimplemented!()
    }

    fn add_item(&self, item: &Item) -> Result<Item, Error> {
        unimplemented!()
    }
}
