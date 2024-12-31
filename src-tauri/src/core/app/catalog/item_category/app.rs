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
use serde_json::json;

pub trait ItemCategoryUseCase {
    fn create_item_category(&self, item_category: &ItemCategory) -> Result<ItemCategory>;
    fn update_item_category(&self, item_category: &ItemCategory) -> Result<ItemCategory>;
    fn delete_item_category(&self, item_category: &ItemCategory) -> Result<bool>;
}

impl<T: SQLInterface> ItemCategoryUseCase for AppService<T> {
    fn create_item_category(&self, item_category: &ItemCategory) -> Result<ItemCategory> {
        let cat_filter = serde_json::from_value::<ItemCategoryFilter>(json!({
            "id": item_category.id
        }))?;

        let list_options = serde_json::from_value(json!({
            "limit": 1
        }))?;

        let existing_item_category = self
            .model
            .get_one::<ItemCategory>(Some(cat_filter.into()), Some(list_options));

        match existing_item_category {
            Some(_) => {
                return Err(Error::UniqueConstraintError);
            }
            None => {}
        }

        self.model.save(&item_category)
    }

    fn update_item_category(&self, item_category: &ItemCategory) -> Result<ItemCategory> {
        let cat_filter: ItemCategoryFilter = serde_json::from_value::<ItemCategoryFilter>(json!({
            "id": item_category.id
        }))?;
        let existing_item_category = self
            .model
            .get_one::<ItemCategory>(Some(cat_filter.into()), None);

        match existing_item_category {
            Some(_) => self.model.save(&item_category),
            None => Err(Error::NotFoundError),
        }
    }

    fn delete_item_category(&self, item_category: &ItemCategory) -> Result<bool> {
        // First check if the category exists
        let cat_filter = serde_json::from_value::<ItemCategoryFilter>(json!({
            "id": item_category.id
        }))?;
        let existing_category = self
            .model
            .get_one::<ItemCategory>(Some(cat_filter.into()), None);

        match existing_category {
            Some(_) => {
                // Then check if it has items
                let item_filter = serde_json::from_value::<ItemCategoryFilter>(json!({
                    "category_id": item_category.id
                }))?;
                let cat_items = self.model.get_many::<Item>(Some(item_filter.into()), None);

                if !cat_items.is_empty() {
                    return Err(Error::HasChildrenError);
                }

                self.model.delete::<ItemCategory>(&item_category)
            }
            None => Err(Error::NotFoundError),
        }
    }
}
