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
    async fn create_item_category(&self, item_category: ItemCategory) -> Result<ItemCategory>;
    async fn update_item_category(&self, item_category: ItemCategory) -> Result<ItemCategory>;
    async fn delete_item_category(&self, item_category: ItemCategory) -> Result<bool>;
}

impl<T: SQLInterface> ItemCategoryUseCase for AppService<T> {
    async fn create_item_category(&self, item_category: ItemCategory) -> Result<ItemCategory> {
        let cat_filter = queries::get_item_cat_by_name(item_category.name.clone());
        let list_options = queries::get_list_options(Some(1), None);
        let existing_item_category = self
            .model
            .get_one::<ItemCategory>(Some(cat_filter.into()), Some(list_options))
            .await?;

        match existing_item_category {
            Some(_) => Err(Error::UniqueConstraintError),
            None => self.model.save(item_category).await,
        }
    }

    async fn update_item_category(&self, item_category: ItemCategory) -> Result<ItemCategory> {
        let cat_filter: ItemCategoryFilter = queries::get_item_cat_by_id(item_category.id.clone());
        let existing_item_category = self
            .model
            .get_one::<ItemCategory>(Some(cat_filter.into()), None)
            .await?;

        match existing_item_category {
            Some(_) => self.model.save(item_category).await,
            None => Err(Error::NotFoundError),
        }
    }

    async fn delete_item_category(&self, item_category: ItemCategory) -> Result<bool> {
        // First check if the category exists
        let cat_filter = queries::get_item_cat_by_id(item_category.id.clone());
        let existing_category = self
            .model
            .get_one::<ItemCategory>(Some(cat_filter.into()), None)
            .await?;

        match existing_category {
            Some(_) => {
                // Then check if it has items
                let item_filter = queries::get_item_by_cat_id(item_category.id.clone());
                let cat_items = self
                    .model
                    .get_many::<Item>(Some(item_filter.into()), None)
                    .await?;

                if !cat_items.is_empty() {
                    return Err(Error::HasChildrenError);
                }

                self.model.delete::<ItemCategory>(item_category).await
            }
            None => Err(Error::NotFoundError),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        adapters::outgoing::database::sqlite_adapter::SQLiteAdapter,
        core::{
            app::{app_service::AppService, catalog::item_category::ItemCategoryUseCase},
            common::{interface::sql::SQLInterface, queries},
            entities::catalog::item_category::{ItemCategory, ItemCategoryState},
        },
    };

    #[async_std::test]
    async fn test_create_item_category() {
        let sqlite_adapter = SQLiteAdapter::new("sqlite::memory:").await.unwrap();
        let app_service = AppService::new(sqlite_adapter);
        let item_category = ItemCategory {
            id: String::from("test_id"),
            name: String::from("test"),
            description: None,
            state: ItemCategoryState::Active,
            created_at: 0,
            updated_at: 0,
        };

        let result = app_service
            .create_item_category(item_category.clone())
            .await;

        assert!(result.is_ok());

        let item_category_filter = queries::get_item_cat_by_id(item_category.id.clone());
        let existing_item_category = app_service
            .model
            .get_one::<ItemCategory>(Some(item_category_filter.into()), None)
            .await
            .unwrap();

        assert!((existing_item_category.unwrap() == item_category));
    }

    #[async_std::test]
    async fn test_create_item_category_already_exists() {}

    #[async_std::test]
    async fn test_update_item_category() {}

    #[async_std::test]
    async fn test_update_item_category_does_not_exist() {}

    #[async_std::test]
    async fn test_delete_item_category() {}

    #[async_std::test]
    async fn test_delete_item_category_has_items() {}
}
