use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    core::{
        command::{app_service::AppService, Command},
        entities::catalog::item_category::ItemCategory,
    },
    error::{Error, Result},
    schema::item_categories::dsl::*,
};

// Commands
pub struct CreateItemCategoryCommand {
    pub category: ItemCategory,
}

pub struct UpdateItemCategoryCommand {
    pub category: ItemCategory,
}

pub struct DeleteItemCategoryCommand {
    pub category: ItemCategory,
}

// Command Implementations
impl Command for CreateItemCategoryCommand {
    fn exec(&self, service: &mut AppService) -> Result<()> {
        service.conn.transaction(|conn| {
            let existing_cat = item_categories
                .filter(name.eq(&self.category.name))
                .select(ItemCategory::as_select())
                .get_result::<ItemCategory>(conn);

            if let Ok(_) = existing_cat {
                return Err(Error::UniqueConstraintError);
            }

            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;

            let mut category = self.category.clone();
            category.created_at = now;
            category.updated_at = now;

            diesel::insert_into(item_categories)
                .values(&category)
                .execute(conn)?;

            Ok(())
        })
    }
}

impl Command for UpdateItemCategoryCommand {
    fn exec(&self, service: &mut AppService) -> Result<()> {
        service.conn.transaction(|conn| {
            let existing_cat = item_categories
                .filter(id.eq(&self.category.id))
                .select(ItemCategory::as_select())
                .get_result::<ItemCategory>(conn);

            if let Err(_) = existing_cat {
                return Err(Error::NotFoundError);
            }

            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;

            let mut category = self.category.clone();
            category.updated_at = now;

            diesel::update(item_categories.filter(id.eq(&self.category.id)))
                .set(&category)
                .execute(conn)?;

            Ok(())
        })
    }
}

impl Command for DeleteItemCategoryCommand {
    fn exec(&self, service: &mut AppService) -> Result<()> {
        service.conn.transaction(|conn| {
            // Check if category has items
            let items = crate::schema::items::dsl::items
                .filter(crate::schema::items::dsl::category_id.eq(&self.category.id))
                .count()
                .get_result::<i64>(conn)?;

            if items > 0 {
                return Err(Error::ForeignKeyConstraintError);
            }

            diesel::delete(item_categories.filter(id.eq(&self.category.id))).execute(conn)?;

            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::entities::catalog::item_category::ItemCategoryState;
    use uuid::Uuid;

    #[test]
    fn test_create_item_category() {
        let mut app_service = AppService::new(":memory:");
        let category = ItemCategory {
            id: Uuid::now_v7().to_string(),
            name: "test".to_string(),
            description: Some("test description".to_string()),
            state: ItemCategoryState::Active,
            created_at: 0,
            updated_at: 0,
        };
        let command = CreateItemCategoryCommand { category };
        let result = command.exec(&mut app_service);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_item_category_already_exists() {
        let mut app_service = AppService::new(":memory:");
        let category = ItemCategory {
            id: Uuid::now_v7().to_string(),
            name: "test".to_string(),
            description: Some("test description".to_string()),
            state: ItemCategoryState::Active,
            created_at: 0,
            updated_at: 0,
        };

        let command = CreateItemCategoryCommand {
            category: category.clone(),
        };
        let result = command.exec(&mut app_service);
        assert!(result.is_ok());

        let command = CreateItemCategoryCommand { category };
        let result = command.exec(&mut app_service);
        assert!(matches!(result, Err(Error::UniqueConstraintError)));
    }

    #[test]
    fn test_update_item_category() {
        let mut app_service = AppService::new(":memory:");
        let category = ItemCategory {
            id: Uuid::now_v7().to_string(),
            name: "test".to_string(),
            description: Some("test description".to_string()),
            state: ItemCategoryState::Active,
            created_at: 0,
            updated_at: 0,
        };

        let create_command = CreateItemCategoryCommand {
            category: category.clone(),
        };
        create_command.exec(&mut app_service).unwrap();

        let mut updated_category = category;
        updated_category.name = "updated test".to_string();

        let update_command = UpdateItemCategoryCommand {
            category: updated_category,
        };
        let result = update_command.exec(&mut app_service);
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_item_category_does_not_exist() {
        let mut app_service = AppService::new(":memory:");
        let category = ItemCategory {
            id: Uuid::now_v7().to_string(),
            name: "test".to_string(),
            description: Some("test description".to_string()),
            state: ItemCategoryState::Active,
            created_at: 0,
            updated_at: 0,
        };

        let command = UpdateItemCategoryCommand { category };
        let result = command.exec(&mut app_service);
        assert!(matches!(result, Err(Error::NotFoundError)));
    }

    #[test]
    fn test_delete_item_category() {
        let mut app_service = AppService::new(":memory:");
        let category = ItemCategory {
            id: Uuid::now_v7().to_string(),
            name: "test".to_string(),
            description: Some("test description".to_string()),
            state: ItemCategoryState::Active,
            created_at: 0,
            updated_at: 0,
        };

        let create_command = CreateItemCategoryCommand {
            category: category.clone(),
        };
        create_command.exec(&mut app_service).unwrap();

        let delete_command = DeleteItemCategoryCommand { category };
        let result = delete_command.exec(&mut app_service);
        assert!(result.is_ok());
    }
}
