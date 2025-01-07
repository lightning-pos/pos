use crate::{
    core::{
        app::app_service::AppService,
        common::{interface::sql::SQLInterface, queries},
        entities::catalog::{item::Item, item_category::ItemCategory},
    },
    error::{Error, Result},
};

pub trait ItemUseCase {
    async fn create_item(&self, item: Item) -> Result<Item>;
    async fn update_item(&self, item: Item) -> Result<Item>;
    async fn delete_item(&self, item: Item) -> Result<bool>;
}

impl<T: SQLInterface> ItemUseCase for AppService<T> {
    async fn create_item(&self, item: Item) -> Result<Item> {
        // First check if the category exists
        let cat_filter = queries::get_item_cat_by_id(item.category_id.clone());
        let category = self
            .model
            .get_one::<ItemCategory>(Some(cat_filter.into()), None)
            .await?;

        match category {
            Some(_) => {
                // Then check if item with same name already exists
                let item_filter = queries::get_item_by_name(item.name.clone());
                let existing_item = self
                    .model
                    .get_one::<Item>(Some(item_filter.into()), None)
                    .await?;

                match existing_item {
                    Some(_) => Err(Error::UniqueConstraintError),
                    None => self.model.save::<Item>(item).await,
                }
            }
            None => Err(Error::NotFoundError),
        }
    }

    async fn update_item(&self, item: Item) -> Result<Item> {
        // First check if the item exists
        let item_filter = queries::get_item_by_id(item.id.clone());
        let existing_item = self
            .model
            .get_one::<Item>(Some(item_filter.into()), None)
            .await?;

        match existing_item {
            Some(_) => {
                let cat_filter = queries::get_item_cat_by_id(item.category_id.clone());
                let category = self
                    .model
                    .get_one::<ItemCategory>(Some(cat_filter.into()), None)
                    .await?;

                match category {
                    Some(_) => self.model.save::<Item>(item).await,
                    None => Err(Error::NotFoundError),
                }
            }
            None => Err(Error::NotFoundError),
        }
    }

    async fn delete_item(&self, item: Item) -> Result<bool> {
        // Check if the item exists first
        let item_filter = queries::get_item_by_id(item.id.clone());
        let existing_item = self
            .model
            .get_one::<Item>(Some(item_filter.into()), None)
            .await?;

        match existing_item {
            Some(_) => self.model.delete::<Item>(item).await,
            None => Err(Error::NotFoundError),
        }
    }
}

#[cfg(test)]
mod test {

    #[async_std::test]
    async fn test_create_item() {}

    #[async_std::test]
    async fn test_create_item_already_exists() {}

    #[async_std::test]
    async fn test_update_item() {}

    #[async_std::test]
    async fn test_update_item_does_not_exist() {}

    #[async_std::test]
    async fn test_delete_item() {}

    #[async_std::test]
    async fn test_delete_item_does_not_exist() {}
}
