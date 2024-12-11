use std::io::Error;

use crate::core::{
    app::app_service::AppService, common::interface::sql::query::join_entity::JoinEntities,
    entities::catalog::item::model::Item,
};

pub trait ItemUseCase {
    fn create_item(&self, item: &Item) -> Result<Item, Error>;
    fn update_item(&self, item: &Item) -> Result<Item, Error>;
    fn delete_item(&self, id: &str) -> Result<bool, Error>;
}

impl<'a> ItemUseCase for AppService<'a> {
    fn create_item(&self, item: &Item) -> Result<Item, Error> {
        let category = self
            .item_category
            .get_one_by_id(&item.category_id, JoinEntities::new(vec![]));

        match category {
            Ok(_) => self.item.insert(item),
            _ => {
                return Err(Error::new(std::io::ErrorKind::Other, "Category not found"));
            }
        }
    }

    fn update_item(&self, item: &Item) -> Result<Item, Error> {
        let category = self
            .item_category
            .get_one_by_id(&item.category_id, JoinEntities::new(vec![]));

        match category {
            Ok(_) => self.item.update(item),
            _ => {
                return Err(Error::new(std::io::ErrorKind::Other, "Category not found"));
            }
        }
    }

    fn delete_item(&self, id: &str) -> Result<bool, Error> {
        self.item.delete(id)
    }
}
