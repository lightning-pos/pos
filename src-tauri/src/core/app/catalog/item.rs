use crate::{
    core::{
        app::app_service::AppService,
        entities::catalog::{item::Item, item_category::ItemCategory},
    },
    error::{Error, Result},
    schema::{
        item_categories::dsl::{id as cat_id, *},
        items::dsl::*,
    },
};
use diesel::{
    query_dsl::methods::{FilterDsl, FindDsl, SelectDsl},
    ExpressionMethods, RunQueryDsl, SelectableHelper,
};

pub trait ItemUseCase {
    fn create_item(&mut self, item: Item) -> Result<Item>;
    fn update_item(&mut self, item: Item) -> Result<Item>;
    fn delete_item(&mut self, item: Item) -> Result<bool>;
}

impl ItemUseCase for AppService {
    fn create_item(&mut self, item: Item) -> Result<Item> {
        let cat = item_categories
            .filter(cat_id.eq(&item.category_id))
            .select(ItemCategory::as_select())
            .get_result::<ItemCategory>(&mut self.conn);

        if let Err(_) = cat {
            return Err(Error::NotFoundError);
        }

        let result = diesel::insert_into(items)
            .values(&item)
            .returning(Item::as_returning())
            .get_result::<Item>(&mut self.conn);

        if let Err(_) = result {
            return Err(Error::UniqueConstraintError);
        }

        Ok(result.unwrap())
    }

    fn update_item(&mut self, item: Item) -> Result<Item> {
        let cat = item_categories
            .filter(cat_id.eq(&item.category_id))
            .select(ItemCategory::as_select())
            .get_result::<ItemCategory>(&mut self.conn);

        if let Err(_) = cat {
            return Err(Error::NotFoundError);
        }

        let result = diesel::update(items.find(&item.id))
            .set(&item)
            .returning(Item::as_returning())
            .get_result::<Item>(&mut self.conn);

        if let Err(_) = result {
            return Err(Error::NotFoundError);
        }

        Ok(result.unwrap())
    }

    fn delete_item(&mut self, item: Item) -> Result<bool> {
        let result = diesel::delete(items.find(&item.id)).execute(&mut self.conn);

        if let Err(_) = result {
            return Err(Error::NotFoundError);
        }

        Ok(result.unwrap() > 0)
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
    fn test_create_item() {
        let mut app_service = AppService::new(":memory:");
        let cat_id = Uuid::now_v7().to_string();
        let cat = ItemCategory {
            id: cat_id.clone(),
            name: "test".to_string(),
            description: Some("test description".to_string()),
            state: ItemCategoryState::Active,
            created_at: 0,
            updated_at: 0,
        };
        let result = app_service.create_item_category(cat);

        assert!(result.is_ok());

        let item = Item {
            id: Uuid::now_v7().to_string(),
            name: "test".to_string(),
            description: Some("test description".to_string()),
            nature: ItemNature::Goods,
            state: ItemState::Active,
            category_id: cat_id,
            created_at: 0,
            updated_at: 0,
        };
        let result = app_service.create_item(item);

        println!("{:?}", result);

        assert!(result.is_ok());
    }

    #[test]
    fn test_update_item() {
        let mut app_service = AppService::new(":memory:");
        let cat_id = Uuid::now_v7().to_string();
        let cat = ItemCategory {
            id: cat_id.clone(),
            name: "test".to_string(),
            description: Some("test description".to_string()),
            state: ItemCategoryState::Active,
            created_at: 0,
            updated_at: 0,
        };
        let result = app_service.create_item_category(cat);

        assert!(result.is_ok());

        let item_id = Uuid::now_v7().to_string();
        let item = Item {
            id: item_id.clone(),
            name: "test".to_string(),
            description: Some("test description".to_string()),
            nature: ItemNature::Goods,
            state: ItemState::Active,
            category_id: cat_id.clone(),
            created_at: 0,
            updated_at: 0,
        };
        let result = app_service.create_item(item);

        assert!(result.is_ok());

        let item = Item {
            id: item_id,
            name: "test2".to_string(),
            description: Some("test description".to_string()),
            nature: ItemNature::Goods,
            state: ItemState::Inactive,
            category_id: cat_id,
            created_at: 0,
            updated_at: 0,
        };
        let result = app_service.update_item(item);

        assert!(result.is_ok());
        assert!(result.as_ref().unwrap().name == "test2");
        assert!(result.as_ref().unwrap().state == ItemState::Inactive);
    }

    #[test]
    fn test_update_item_does_not_exist() {
        let mut app_service = AppService::new(":memory:");
        let item = Item {
            id: Uuid::now_v7().to_string(),
            name: "test".to_string(),
            description: Some("test description".to_string()),
            nature: ItemNature::Goods,
            state: ItemState::Active,
            category_id: Uuid::now_v7().to_string(),
            created_at: 0,
            updated_at: 0,
        };
        let result = app_service.update_item(item);

        assert!(result.is_err());
    }

    #[test]
    fn test_delete_item() {
        let mut app_service = AppService::new(":memory:");
        let cat_id = Uuid::now_v7().to_string();
        let cat = ItemCategory {
            id: cat_id.clone(),
            name: "test".to_string(),
            description: Some("test description".to_string()),
            state: ItemCategoryState::Active,
            created_at: 0,
            updated_at: 0,
        };
        let result = app_service.create_item_category(cat);

        assert!(result.is_ok());

        let item_id = Uuid::now_v7().to_string();
        let item = Item {
            id: item_id.clone(),
            name: "test".to_string(),
            description: Some("test description".to_string()),
            nature: ItemNature::Goods,
            state: ItemState::Active,
            category_id: cat_id.clone(),
            created_at: 0,
            updated_at: 0,
        };
        let result = app_service.create_item(item.clone());

        assert!(result.is_ok());

        let result = app_service.delete_item(item);

        assert!(result.is_ok());
    }

    #[test]
    fn test_delete_item_does_not_exist() {
        let mut app_service = AppService::new(":memory:");
        let item = Item {
            id: Uuid::now_v7().to_string(),
            name: "test".to_string(),
            description: Some("test description".to_string()),
            nature: ItemNature::Goods,
            state: ItemState::Active,
            category_id: Uuid::now_v7().to_string(),
            created_at: 0,
            updated_at: 0,
        };
        let result = app_service.delete_item(item);

        assert!(result.is_ok());

        assert!(result.unwrap() == false);
    }
}
