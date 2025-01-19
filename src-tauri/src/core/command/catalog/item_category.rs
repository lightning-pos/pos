use chrono::Utc;
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{
    core::{
        command::{app_service::AppService, Command},
        entities::catalog::item_category::{
            ItemCategory, ItemCategoryState, NewItemCategory, UpdateItemCategory,
        },
    },
    error::{Error, Result},
    schema::{item_categories::dsl::*, items},
};

// Commands
pub struct CreateItemCategoryCommand {
    pub category: NewItemCategory,
}

pub struct UpdateItemCategoryCommand {
    pub category: UpdateItemCategory,
}

pub struct DeleteItemCategoryCommand {
    pub id: String,
}

// Command Implementations
impl Command for CreateItemCategoryCommand {
    type Output = ItemCategory;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let existing_cat = item_categories
                .filter(name.eq(&self.category.name))
                .select(ItemCategory::as_select())
                .get_result::<ItemCategory>(conn);

            if let Ok(_) = existing_cat {
                return Err(Error::UniqueConstraintError);
            }

            let now = Utc::now().naive_utc();
            let new_cat = ItemCategory {
                id: Uuid::now_v7().to_string(),
                name: self.category.name.clone(),
                description: self.category.description.clone(),
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
            item_categories
                .filter(id.eq(&self.category.id))
                .limit(1)
                .select(ItemCategory::as_select())
                .get_result::<ItemCategory>(conn)?;

            let now = Utc::now().naive_utc();

            let mut category = self.category.clone();
            category.updated_at = Some(now);

            let cat = diesel::update(item_categories.filter(id.eq(&self.category.id)))
                .set(&category)
                .returning(ItemCategory::as_returning())
                .get_result(conn)?;

            Ok(cat)
        })
    }
}

impl Command for DeleteItemCategoryCommand {
    type Output = i32;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Check if category has items
            let items = items::table
                .filter(items::category_id.eq(&self.id))
                .count()
                .get_result::<i64>(conn)?;

            if items > 0 {
                return Err(Error::ForeignKeyConstraintError);
            }

            let res = diesel::delete(item_categories.filter(id.eq(&self.id))).execute(conn)?;

            Ok(res as i32)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::entities::catalog::item_category::ItemCategoryState;
    use diesel::result::Error::NotFound;
    use uuid::Uuid;

    #[test]
    fn test_create_item_category() {
        let mut app_service = AppService::new(":memory:");
        let new_cat = NewItemCategory {
            name: "test".to_string(),
            description: Some("test description".to_string()),
        };
        let command = CreateItemCategoryCommand { category: new_cat };
        let result = command.exec(&mut app_service);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_item_category_already_exists() {
        let mut app_service = AppService::new(":memory:");
        let new_cat = NewItemCategory {
            name: "test".to_string(),
            description: Some("test description".to_string()),
        };
        let command = CreateItemCategoryCommand {
            category: new_cat.clone(),
        };
        let result = command.exec(&mut app_service);

        assert!(result.is_ok());

        let command = CreateItemCategoryCommand { category: new_cat };
        let result = command.exec(&mut app_service);

        assert!(matches!(result, Err(Error::UniqueConstraintError)));
    }

    #[test]
    fn test_update_item_category() {
        let mut app_service = AppService::new(":memory:");
        let new_cat = NewItemCategory {
            name: "test".to_string(),
            description: Some("test description".to_string()),
        };
        let create_command = CreateItemCategoryCommand { category: new_cat };
        let category = create_command.exec(&mut app_service).unwrap();

        let updated_category = UpdateItemCategory {
            id: category.id.clone(),
            name: Some("updated test".to_string()),
            description: None,
            state: None,
            updated_at: None,
        };

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
        let now = Utc::now().naive_utc();
        let category = UpdateItemCategory {
            id: Uuid::now_v7().to_string(),
            name: Some("test".to_string()),
            description: Some("test description".to_string()),
            state: Some(ItemCategoryState::Active),
            updated_at: Some(now),
        };

        let command = UpdateItemCategoryCommand { category };
        let result = command.exec(&mut app_service);
        assert!(matches!(result, Err(Error::DieselError(NotFound))));
    }

    #[test]
    fn test_delete_item_category() {
        let mut app_service = AppService::new(":memory:");
        let new_cat = NewItemCategory {
            name: "test".to_string(),
            description: Some("test description".to_string()),
        };

        let create_command = CreateItemCategoryCommand { category: new_cat };
        let cat = create_command.exec(&mut app_service).unwrap();

        let delete_command = DeleteItemCategoryCommand { id: cat.id };
        let result = delete_command.exec(&mut app_service);
        assert!(result.is_ok());
    }
}
