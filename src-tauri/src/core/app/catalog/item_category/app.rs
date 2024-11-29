use std::io::Error;

use crate::core::{
    entities::catalog::item_category::model::ItemCategory,
    app::catalog_service::CatalogService,
};

pub trait ItemCategoryUseCase {
    fn create_item_category(&self, item_category: &ItemCategory) -> Result<ItemCategory, Error>;
    fn update_item_category(&self, item_category: &ItemCategory) -> Result<ItemCategory, Error>;
    fn delete_item_category(&self, id: &str) -> Result<bool, Error>;
}

impl<'a> ItemCategoryUseCase for CatalogService<'a> {
    fn create_item_category(&self, item_category: &ItemCategory) -> Result<ItemCategory, Error> {
        let existing_item_category = self.item_category.is_name_taken(&item_category.name);

        match existing_item_category {
            Ok(true) => {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Item category already exists",
                ));
            }
            _ => {}
        }

        self.item_category.insert(item_category)
    }

    fn update_item_category(&self, item_category: &ItemCategory) -> Result<ItemCategory, Error> {
        let existing_item_category = self.item_category.is_name_taken(&item_category.name);

        match existing_item_category {
            Ok(true) => {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Item category already exists",
                ));
            }
            _ => {}
        }

        self.item_category.update(item_category)
    }

    fn delete_item_category(&self, id: &str) -> Result<bool, Error> {
        let has_items = self.item_category.has_items(id);

        match has_items {
            Ok(true) => {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Item category has items",
                ));
            }
            _ => {}
        }

        self.item_category.delete(id)
    }
}
