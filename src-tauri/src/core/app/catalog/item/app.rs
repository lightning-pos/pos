use crate::{
    core::{
        app::app_service::AppService,
        common::interface::sql::SQLInterface,
        entities::catalog::{
            item::Item,
            item_category::{ItemCategory, ItemCategoryFilter},
        },
    },
    error::{Error, Result},
};

pub trait ItemUseCase {
    fn create_item(&self, item: &Item) -> Result<Item>;
    fn update_item(&self, item: &Item) -> Result<Item>;
    fn delete_item(&self, item: &Item) -> Result<bool>;
}

impl<T: SQLInterface> ItemUseCase for AppService<T> {
    fn create_item(&self, item: &Item) -> Result<Item> {
        // First check if the category exists
        let cat_filter = serde_json::from_value::<ItemCategoryFilter>(serde_json::json!({
            "id": item.category_id
        }))?;
        let category = self
            .model
            .get_one::<ItemCategory>(Some(cat_filter.into()), None);

        match category {
            Some(_) => {
                // Then check if item with same ID already exists
                let item_filter =
                    serde_json::from_value::<ItemCategoryFilter>(serde_json::json!({
                        "id": item.id
                    }))?;
                let existing_item = self.model.get_one::<Item>(Some(item_filter.into()), None);

                match existing_item {
                    Some(_) => Err(Error::UniqueConstraintError),
                    None => self.model.save::<Item>(&item),
                }
            }
            None => Err(Error::NotFoundError),
        }
    }

    fn update_item(&self, item: &Item) -> Result<Item> {
        // First check if the item exists
        let item_filter = serde_json::from_value::<ItemCategoryFilter>(serde_json::json!({
            "id": item.id
        }))?;
        let existing_item = self.model.get_one::<Item>(Some(item_filter.into()), None);

        match existing_item {
            Some(_) => {
                // Then check if the new category exists
                let cat_filter = serde_json::from_value::<ItemCategoryFilter>(serde_json::json!({
                    "id": item.category_id
                }))?;
                let category = self
                    .model
                    .get_one::<ItemCategory>(Some(cat_filter.into()), None);

                match category {
                    Some(_) => self.model.save::<Item>(&item),
                    None => Err(Error::NotFoundError),
                }
            }
            None => Err(Error::NotFoundError),
        }
    }

    fn delete_item(&self, item: &Item) -> Result<bool> {
        // Check if the item exists first
        let item_filter = serde_json::from_value::<ItemCategoryFilter>(serde_json::json!({
            "id": item.id
        }))?;
        let existing_item = self.model.get_one::<Item>(Some(item_filter.into()), None);

        match existing_item {
            Some(_) => self.model.delete::<Item>(&item),
            None => Err(Error::NotFoundError),
        }
    }
}
