use std::io::Error;

use crate::app::catalog::{
    catalog_service::CatalogService,
    item::item_model::{Item, ItemUseCase},
};

impl<'a> ItemUseCase for CatalogService<'a> {
    fn create_item(&self, item: &Item) -> Result<Item, Error> {
        let category = self.item_category.get_one_by_id(&item.category_id);

        match category {
            Ok(_) => self.item.insert(item),
            _ => {
                return Err(Error::new(std::io::ErrorKind::Other, "Category not found"));
            }
        }
    }

    fn update_item(&self, item: &Item) -> Result<Item, Error> {
        let category = self.item_category.get_one_by_id(&item.category_id);

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
