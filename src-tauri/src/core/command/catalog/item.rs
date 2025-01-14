use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    core::{
        command::{app_service::AppService, Command},
        entities::catalog::{item::Item, item_category::ItemCategory},
    },
    error::{Error, Result},
    schema::{
        item_categories::dsl::{id as cat_id, item_categories},
        items::dsl::*,
    },
};

// Commands
pub struct CreateItemCommand {
    pub item: Item,
}

pub struct UpdateItemCommand {
    pub item: Item,
}

pub struct DeleteItemCommand {
    pub item: Item,
}

// Command Implementations
impl Command for CreateItemCommand {
    type Output = ();

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Verify category exists
            let cat = item_categories
                .filter(cat_id.eq(&self.item.category_id))
                .select(ItemCategory::as_select())
                .get_result::<ItemCategory>(conn);

            if let Err(_) = cat {
                return Err(Error::NotFoundError);
            }

            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;

            let mut item = self.item.clone();
            item.created_at = now;
            item.updated_at = now;

            diesel::insert_into(items).values(&item).execute(conn)?;

            Ok(())
        })
    }
}

impl Command for UpdateItemCommand {
    type Output = ();

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Verify category exists
            let cat = item_categories
                .filter(cat_id.eq(&self.item.category_id))
                .select(ItemCategory::as_select())
                .get_result::<ItemCategory>(conn);

            if let Err(_) = cat {
                return Err(Error::NotFoundError);
            }

            // Verify item exists
            let existing_item = items
                .find(&self.item.id)
                .select(Item::as_select())
                .get_result::<Item>(conn);

            if let Err(_) = existing_item {
                return Err(Error::NotFoundError);
            }

            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;

            let mut item = self.item.clone();
            item.updated_at = now;

            diesel::update(items.find(&self.item.id))
                .set(&item)
                .execute(conn)?;

            Ok(())
        })
    }
}

impl Command for DeleteItemCommand {
    type Output = ();

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let result = diesel::delete(items.find(&self.item.id)).execute(conn)?;

            if result == 0 {
                return Err(Error::NotFoundError);
            }

            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        command::catalog::item_category::CreateItemCategoryCommand,
        entities::catalog::item::{ItemNature, ItemState},
    };
    use uuid::Uuid;

    #[test]
    fn test_create_item() {
        let mut app_service = AppService::new(":memory:");
        let create_cat_command = CreateItemCategoryCommand {
            name: "test".to_string(),
            description: None,
        };
        let cat = create_cat_command.exec(&mut app_service).unwrap();
        let item = Item {
            id: Uuid::now_v7().to_string(),
            name: "test".to_string(),
            description: Some("test description".to_string()),
            nature: ItemNature::Goods,
            state: ItemState::Active,
            price: 0,
            category_id: cat.id,
            created_at: 0,
            updated_at: 0,
        };
        let command = CreateItemCommand { item };
        let result = command.exec(&mut app_service);

        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_item() {
        let mut app_service = AppService::new(":memory:");

        let create_cat_command = CreateItemCategoryCommand {
            name: "test".to_string(),
            description: None,
        };
        let cat = create_cat_command.exec(&mut app_service).unwrap();

        let item_id = Uuid::now_v7().to_string();
        let item = Item {
            id: item_id.clone(),
            name: "test".to_string(),
            description: Some("test description".to_string()),
            nature: ItemNature::Goods,
            state: ItemState::Active,
            price: 0,
            category_id: cat.id.clone(),
            created_at: 0,
            updated_at: 0,
        };

        let create_command = CreateItemCommand { item };
        create_command.exec(&mut app_service).unwrap();

        let updated_item = Item {
            id: item_id,
            name: "test2".to_string(),
            description: Some("test description".to_string()),
            nature: ItemNature::Goods,
            state: ItemState::Inactive,
            price: 0,
            category_id: cat.id.clone(),
            created_at: 0,
            updated_at: 0,
        };

        let update_command = UpdateItemCommand { item: updated_item };
        let result = update_command.exec(&mut app_service);
        assert!(result.is_ok());
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
            price: 0,
            category_id: Uuid::now_v7().to_string(),
            created_at: 0,
            updated_at: 0,
        };

        let command = UpdateItemCommand { item };
        let result = command.exec(&mut app_service);
        assert!(matches!(result, Err(Error::NotFoundError)));
    }

    #[test]
    fn test_delete_item() {
        let mut app_service = AppService::new(":memory:");

        let create_cat_command = CreateItemCategoryCommand {
            name: "test".to_string(),
            description: None,
        };
        let cat = create_cat_command.exec(&mut app_service).unwrap();

        let item = Item {
            id: Uuid::now_v7().to_string(),
            name: "test".to_string(),
            description: Some("test description".to_string()),
            nature: ItemNature::Goods,
            state: ItemState::Active,
            price: 0,
            category_id: cat.id.clone(),
            created_at: 0,
            updated_at: 0,
        };

        let create_command = CreateItemCommand { item: item.clone() };
        create_command.exec(&mut app_service).unwrap();

        let delete_command = DeleteItemCommand { item };
        let result = delete_command.exec(&mut app_service);
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
            price: 0,
            category_id: Uuid::now_v7().to_string(),
            created_at: 0,
            updated_at: 0,
        };

        let command = DeleteItemCommand { item };
        let result = command.exec(&mut app_service);
        assert!(matches!(result, Err(Error::NotFoundError)));
    }
}
