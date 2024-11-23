use std::io::Error;

use crate::core::entities::catalog::item::{model::Item, repository::ItemRepository};

struct ItemAdapter {}

impl ItemRepository for ItemAdapter {
    fn insert(&self, item: &Item) -> Result<Item, Error> {
        unimplemented!()
    }

    fn update(&self, item: &Item) -> Result<Item, Error> {
        unimplemented!()
    }

    fn delete(&self, id: &str) -> Result<bool, Error> {
        unimplemented!()
    }
}
