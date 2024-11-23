use std::io::Error;

use super::model::ItemCategory;

#[cfg_attr(test, mockall::automock)]
pub trait ItemCategoryRepository {
    fn is_name_taken(&self, name: &str) -> Result<bool, Error>;
    fn has_items(&self, id: &str) -> Result<bool, Error>;
    fn get_one_by_id(&self, id: &str) -> Result<ItemCategory, Error>;
    fn insert(&self, entity: &ItemCategory) -> Result<ItemCategory, Error>;
    fn update(&self, entity: &ItemCategory) -> Result<ItemCategory, Error>;
    fn delete(&self, id: &str) -> Result<bool, Error>;
}
