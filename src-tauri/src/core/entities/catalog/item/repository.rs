use std::io::Error;

use super::model::Item;

#[cfg_attr(test, mockall::automock)]
pub trait ItemRepository {
    fn insert(&self, item: &Item) -> Result<Item, Error>;
    fn update(&self, item: &Item) -> Result<Item, Error>;
    fn delete(&self, id: &str) -> Result<bool, Error>;
}
