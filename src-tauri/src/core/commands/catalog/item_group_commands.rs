use chrono::Utc;
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{
    core::{
        commands::{app_service::AppService, Command},
        models::catalog::item_group_model::{
            ItemGroup, ItemGroupNew, ItemGroupState, ItemGroupUpdate,
        },
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
    schema::{item_categories::dsl::*, items},
};

// Commands
pub struct CreateItemGroupCommand {
    pub category: ItemGroupNew,
}

pub struct UpdateItemGroupCommand {
    pub category: ItemGroupUpdate,
}

pub struct DeleteItemGroupCommand {
    pub id: DbUuid,
}

// Command Implementations
impl Command for CreateItemGroupCommand {
    type Output = ItemGroup;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let existing_cat = item_categories
                .filter(name.eq(&self.category.name))
                .select(ItemGroup::as_select())
                .get_result::<ItemGroup>(conn);

            if let Ok(_) = existing_cat {
                return Err(Error::UniqueConstraintError);
            }

            let now = Utc::now().naive_utc();
            let new_cat = ItemGroup {
                id: Uuid::now_v7().into(),
                name: self.category.name.clone(),
                description: self.category.description.clone(),
                state: ItemGroupState::Inactive,
                created_at: now,
                updated_at: now,
            };

            let cat = diesel::insert_into(item_categories)
                .values(&new_cat)
                .returning(ItemGroup::as_returning())
                .get_result(conn)?;

            Ok(cat)
        })
    }
}

impl Command for UpdateItemGroupCommand {
    type Output = ItemGroup;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            item_categories
                .filter(id.eq(&self.category.id))
                .limit(1)
                .select(ItemGroup::as_select())
                .get_result::<ItemGroup>(conn)?;

            let now = Utc::now().naive_utc();

            let mut category = self.category.clone();
            category.updated_at = Some(now);

            let cat = diesel::update(item_categories.filter(id.eq(&self.category.id)))
                .set(&category)
                .returning(ItemGroup::as_returning())
                .get_result(conn)?;

            Ok(cat)
        })
    }
}

impl Command for DeleteItemGroupCommand {
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
    use crate::core::models::catalog::item_group_model::ItemGroupState;
    use diesel::result::Error::NotFound;
    use uuid::Uuid;

    #[test]
    fn test_create_item_category() {
        let mut app_service = AppService::new(":memory:");
        let new_cat = ItemGroupNew {
            name: "test".to_string(),
            description: Some("test description".to_string()),
        };
        let command = CreateItemGroupCommand { category: new_cat };
        let result = command.exec(&mut app_service);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_item_category_already_exists() {
        let mut app_service = AppService::new(":memory:");
        let new_cat = ItemGroupNew {
            name: "test".to_string(),
            description: Some("test description".to_string()),
        };
        let command = CreateItemGroupCommand {
            category: new_cat.clone(),
        };
        let result = command.exec(&mut app_service);

        assert!(result.is_ok());

        let command = CreateItemGroupCommand { category: new_cat };
        let result = command.exec(&mut app_service);

        assert!(matches!(result, Err(Error::UniqueConstraintError)));
    }

    #[test]
    fn test_update_item_category() {
        let mut app_service = AppService::new(":memory:");
        let new_cat = ItemGroupNew {
            name: "test".to_string(),
            description: Some("test description".to_string()),
        };
        let create_command = CreateItemGroupCommand { category: new_cat };
        let category = create_command.exec(&mut app_service).unwrap();

        let updated_category = ItemGroupUpdate {
            id: category.id,
            name: Some("updated test".to_string()),
            description: None,
            state: None,
            updated_at: None,
        };

        let update_command = UpdateItemGroupCommand {
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
        let category = ItemGroupUpdate {
            id: Uuid::now_v7().into(),
            name: Some("test".to_string()),
            description: Some(Some("test description".to_string())),
            state: Some(ItemGroupState::Active),
            updated_at: Some(now),
        };

        let command = UpdateItemGroupCommand { category };
        let result = command.exec(&mut app_service);
        assert!(matches!(result, Err(Error::DieselError(NotFound))))
    }

    #[test]
    fn test_delete_item_category() {
        let mut app_service = AppService::new(":memory:");
        let new_cat = ItemGroupNew {
            name: "test".to_string(),
            description: Some("test description".to_string()),
        };

        let create_command = CreateItemGroupCommand { category: new_cat };
        let cat = create_command.exec(&mut app_service).unwrap();

        let delete_command = DeleteItemGroupCommand { id: cat.id };
        let result = delete_command.exec(&mut app_service);
        assert!(result.is_ok());
    }
}
