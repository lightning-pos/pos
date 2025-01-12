use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{
    core::{app::app_service::AppService, entities::catalog::item_category::ItemCategory},
    error::{Error, Result},
    schema::item_categories::dsl::*,
};

pub trait ItemCategoryUseCase {
    fn create_item_category(&mut self, item_category: ItemCategory) -> Result<ItemCategory>;
    fn update_item_category(&mut self, item_category: ItemCategory) -> Result<ItemCategory>;
    fn delete_item_category(&mut self, item_category: ItemCategory) -> Result<bool>;
}

impl ItemCategoryUseCase for AppService {
    fn create_item_category(&mut self, item_category: ItemCategory) -> Result<ItemCategory> {
        let existing_cat = item_categories
            .filter(name.eq(&item_category.name))
            .select(ItemCategory::as_select())
            .get_result::<ItemCategory>(&mut self.conn);

        if let Ok(_) = existing_cat {
            return Err(Error::UniqueConstraintError);
        }

        let cat = diesel::insert_into(item_categories)
            .values(&item_category)
            .returning(ItemCategory::as_returning())
            .get_result::<ItemCategory>(&mut self.conn)?;

        Ok(cat)
    }

    fn update_item_category(&mut self, item_category: ItemCategory) -> Result<ItemCategory> {
        let existing_cat = item_categories
            .filter(id.eq(&item_category.id))
            .select(ItemCategory::as_select())
            .get_result::<ItemCategory>(&mut self.conn);

        if let Err(_) = existing_cat {
            return Err(Error::NotFoundError);
        }

        let cat = diesel::update(item_categories.filter(id.eq(&item_category.id)))
            .set(&item_category)
            .returning(ItemCategory::as_returning())
            .get_result::<ItemCategory>(&mut self.conn)?;

        Ok(cat)
    }

    fn delete_item_category(&mut self, item_category: ItemCategory) -> Result<bool> {
        diesel::delete(item_categories.filter(id.eq(&item_category.id)))
            .returning(ItemCategory::as_returning())
            .get_result::<ItemCategory>(&mut self.conn)?;

        Ok(true)
    }
}

#[cfg(test)]
mod test {
    use uuid::Uuid;

    use crate::core::{
        app::{
            app_service::AppService,
            catalog::{item::ItemUseCase, item_category::ItemCategoryUseCase},
        },
        entities::catalog::{
            item::{Item, ItemNature, ItemState},
            item_category::{ItemCategory, ItemCategoryState},
        },
    };

    #[test]
    fn test_create_item_category() {
        let mut app_service = AppService::new(":memory:");
        let item_category = ItemCategory {
            id: Uuid::now_v7().to_string(),
            name: "test".to_string(),
            description: Some("test description".to_string()),
            state: ItemCategoryState::Active,
            created_at: 0,
            updated_at: 0,
        };
        let result = app_service.create_item_category(item_category);

        assert!(result.is_ok());
    }

    #[test]
    fn test_create_item_category_already_exists() {
        let mut app_service = AppService::new(":memory:");
        let item_category = ItemCategory {
            id: Uuid::now_v7().to_string(),
            name: "test".to_string(),
            description: Some("test description".to_string()),
            state: ItemCategoryState::Active,
            created_at: 0,
            updated_at: 0,
        };
        let result = app_service.create_item_category(item_category);

        assert!(result.is_ok());

        let item_category = ItemCategory {
            id: Uuid::now_v7().to_string(),
            name: "test".to_string(),
            description: Some("test description".to_string()),
            state: ItemCategoryState::Active,
            created_at: 0,
            updated_at: 0,
        };
        let result = app_service.create_item_category(item_category);

        assert!(result.is_err());
    }

    #[test]
    fn test_update_item_category() {
        let mut app_service = AppService::new(":memory:");
        let cat_id = Uuid::now_v7().to_string();
        let item_category = ItemCategory {
            id: cat_id.clone(),
            name: "test".to_string(),
            description: Some("test description".to_string()),
            state: ItemCategoryState::Active,
            created_at: 0,
            updated_at: 0,
        };
        let result = app_service.create_item_category(item_category);

        assert!(result.is_ok());

        let item_category = ItemCategory {
            id: cat_id,
            name: "test".to_string(),
            description: Some("test description".to_string()),
            state: ItemCategoryState::Inactive,
            created_at: 0,
            updated_at: 0,
        };
        let result = app_service.update_item_category(item_category);

        assert!(result.is_ok());

        assert!(result.unwrap().state == ItemCategoryState::Inactive);
    }

    #[test]
    fn test_update_item_category_does_not_exist() {
        let mut app_service = AppService::new(":memory:");
        let item_category = ItemCategory {
            id: Uuid::now_v7().to_string(),
            name: "test".to_string(),
            description: Some("test description".to_string()),
            state: ItemCategoryState::Active,
            created_at: 0,
            updated_at: 0,
        };
        let result = app_service.update_item_category(item_category);

        assert!(result.is_err());
    }

    #[test]
    fn test_delete_item_category() {
        let mut app_service = AppService::new(":memory:");
        let cat_id = Uuid::now_v7().to_string();
        let item_category = ItemCategory {
            id: cat_id.clone(),
            name: "test".to_string(),
            description: Some("test description".to_string()),
            state: ItemCategoryState::Active,
            created_at: 0,
            updated_at: 0,
        };
        let result = app_service.create_item_category(item_category.clone());

        assert!(result.is_ok());

        let result = app_service.delete_item_category(item_category);

        assert!(result.is_ok());
    }

    #[test]
    fn test_delete_item_category_has_items() {
        let mut app_service = AppService::new(":memory:");
        let cat_id = Uuid::now_v7().to_string();
        let item_category = ItemCategory {
            id: cat_id.clone(),
            name: "test".to_string(),
            description: Some("test description".to_string()),
            state: ItemCategoryState::Active,
            created_at: 0,
            updated_at: 0,
        };

        let item = Item {
            id: Uuid::now_v7().to_string(),
            name: "test".to_string(),
            description: Some("test description".to_string()),
            nature: ItemNature::Goods,
            state: ItemState::Active,
            category_id: cat_id.clone(),
            created_at: 0,
            updated_at: 0,
        };

        let result = app_service.create_item_category(item_category.clone());

        assert!(result.is_ok());

        let result = app_service.create_item(item);

        assert!(result.is_ok());

        let result = app_service.delete_item_category(item_category);

        assert!(result.is_err());
    }
}
