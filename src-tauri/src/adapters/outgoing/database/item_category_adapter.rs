use crate::core::entities::catalog::item_category::item_category_model::{
    ItemCategory, ItemCategoryRepository, ItemCategoryState,
};
use std::io::Error;
struct ItemCategoryAdapter {}

impl ItemCategoryRepository for ItemCategoryAdapter {
    fn is_name_taken(&self, name: &str) -> Result<bool, Error> {
        unimplemented!()
    }

    fn get_one_by_id(&self, id: &str) -> Result<ItemCategory, Error> {
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
}
