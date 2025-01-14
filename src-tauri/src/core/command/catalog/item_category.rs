use chrono::Utc;
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{
    core::{
        command::{app_service::AppService, Command},
        entities::catalog::item_category::{ItemCategory, ItemCategoryState},
    },
    error::{Error, Result},
    schema::item_categories::dsl::*,
};

// Commands
pub struct CreateItemCategoryCommand {
    pub name: String,
    pub description: Option<String>,
}

pub struct UpdateItemCategoryCommand {
    pub category: ItemCategory,
}

pub struct DeleteItemCategoryCommand {
    pub category: ItemCategory,
}

// Command Implementations
impl Command for CreateItemCategoryCommand {
    type Output = ItemCategory;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let existing_cat = item_categories
                .filter(name.eq(&self.name))
                .select(ItemCategory::as_select())
                .get_result::<ItemCategory>(conn);

            if let Ok(_) = existing_cat {
                return Err(Error::UniqueConstraintError);
            }

            let now = Utc::now().timestamp();
            let new_cat = ItemCategory {
                id: Uuid::now_v7().to_string(),
                name: self.name.clone(),
                description: self.description.clone(),
                state: ItemCategoryState::Inactive,
                created_at: now,
                updated_at: now,
            };

            let cat = diesel::insert_into(item_categories)
                .values(&new_cat)
                .returning(ItemCategory::as_returning())
                .get_result(conn)?;

            Ok(cat)
        })
    }
}

impl Command for UpdateItemCategoryCommand {
    type Output = ItemCategory;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let existing_cat = item_categories
                .filter(id.eq(&self.category.id))
                .select(ItemCategory::as_select())
                .get_result::<ItemCategory>(conn);

            if let Err(_) = existing_cat {
                return Err(Error::NotFoundError);
            }

            let now = Utc::now().timestamp();

            let mut category = self.category.clone();
            category.updated_at = now;

            let cat = diesel::update(item_categories.filter(id.eq(&self.category.id)))
                .set(&category)
                .returning(ItemCategory::as_returning())
                .get_result(conn)?;

            Ok(cat)
        })
    }
}

impl Command for DeleteItemCategoryCommand {
    type Output = ();

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
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
        let command = CreateItemCategoryCommand {
            name: "test".to_string(),
            description: Some("test description".to_string()),
        };
        let result = command.exec(&mut app_service);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_item_category_already_exists() {
        let mut app_service = AppService::new(":memory:");
        let command = CreateItemCategoryCommand {
            name: "test".to_string(),
            description: Some("test description".to_string()),
        };
        let result = command.exec(&mut app_service);

        assert!(result.is_ok());

        let command = CreateItemCategoryCommand {
            name: "test".to_string(),
            description: Some("test description".to_string()),
        };
        let result = command.exec(&mut app_service);

        assert!(matches!(result, Err(Error::UniqueConstraintError)));
    }

    #[test]
    fn test_update_item_category() {
        let mut app_service = AppService::new(":memory:");
        let create_command = CreateItemCategoryCommand {
            name: "test".to_string(),
            description: Some("test description".to_string()),
        };
        let category = create_command.exec(&mut app_service).unwrap();

        let mut updated_category = category;
        updated_category.name = "updated test".to_string();

        let update_command = UpdateItemCategoryCommand {
            category: updated_category,
        };
        let result = update_command.exec(&mut app_service);
        assert!(result.is_ok());
        assert!(result.unwrap().name == "updated test");
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
            name: "test".to_string(),
            description: Some("test description".to_string()),
        };
        create_command.exec(&mut app_service).unwrap();

        let delete_command = DeleteItemCategoryCommand { category };
        let result = delete_command.exec(&mut app_service);
        assert!(result.is_ok());
    }
}
