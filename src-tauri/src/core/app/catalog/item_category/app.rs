use crate::{
    core::{
        app::app_service::AppService,
        common::{interface::sql::SQLInterface, queries},
        entities::catalog::{
            item::Item,
            item_category::{ItemCategory, ItemCategoryFilter},
        },
    },
    error::{Error, Result},
};

pub trait ItemCategoryUseCase {
    fn create_item_category(&self, item_category: &ItemCategory) -> Result<ItemCategory>;
    fn update_item_category(&self, item_category: &ItemCategory) -> Result<ItemCategory>;
    fn delete_item_category(&self, item_category: &ItemCategory) -> Result<bool>;
}

impl<T: SQLInterface> ItemCategoryUseCase for AppService<T> {
    fn create_item_category(&self, item_category: &ItemCategory) -> Result<ItemCategory> {
        let cat_filter = queries::get_item_cat_by_name(item_category.name.clone());
        let list_options = queries::get_list_options(Some(1), None);
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
        let cat_filter: ItemCategoryFilter = queries::get_item_cat_by_id(item_category.id.clone());
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
        let cat_filter = queries::get_item_cat_by_id(item_category.id.clone());
        let existing_category = self
            .model
            .get_one::<ItemCategory>(Some(cat_filter.into()), None);

        match existing_category {
            Some(_) => {
                // Then check if it has items
                let item_filter = queries::get_item_by_cat_id(item_category.id.clone());
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
