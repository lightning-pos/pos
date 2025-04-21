use chrono::Utc;
use sea_query::{Expr, Query};
use uuid::Uuid;

use crate::{
    adapters::outgoing::database::DatabaseAdapter, core::{
        commands::{app_service::AppService, Command},
        models::catalog::item_group_model::{
            ItemCategories, ItemGroup, ItemGroupNew, ItemGroupState, ItemGroupUpdate,
        },
        types::db_uuid::DbUuid,
    }, error::{Error, Result}
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

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Check if a category with the same name already exists
        let mut select_query = Query::select();
        let select_stmt = select_query
            .from(ItemCategories::Table)
            .column(ItemCategories::Id)
            .and_where(Expr::col(ItemCategories::Name).eq(self.category.name.clone()));

        let existing = service.db_adapter.query_optional::<ItemGroup>(&select_stmt).await?;

        if existing.is_some() {
            return Err(Error::UniqueConstraintError);
        }

        let now = Utc::now().naive_utc();
        let item_id = Uuid::now_v7().into();

        let new_cat = ItemGroup {
            id: item_id,
            name: self.category.name.clone(),
            description: self.category.description.clone(),
            state: ItemGroupState::Inactive,
            created_at: now,
            updated_at: now,
        };

        // Insert the new category
        let mut insert_query = Query::insert();
        let insert_stmt = insert_query
            .into_table(ItemCategories::Table)
            .columns([
                ItemCategories::Id,
                ItemCategories::Name,
                ItemCategories::Description,
                ItemCategories::State,
                ItemCategories::CreatedAt,
                ItemCategories::UpdatedAt,
            ])
            .values_panic([
                new_cat.id.to_string().into(),
                new_cat.name.clone().into(),
                new_cat.description.clone().map_or_else(|| "NULL".into(), |d| d.into()),
                new_cat.state.to_string().into(),
                new_cat.created_at.to_string().into(),
                new_cat.updated_at.to_string().into(),
            ]);

        service.db_adapter.insert_many(&insert_stmt).await?;

        Ok(new_cat)
    }
}

impl Command for UpdateItemGroupCommand {
    type Output = ItemGroup;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Check if the category exists
        let mut select_query = Query::select();
        let select_stmt = select_query
            .from(ItemCategories::Table)
            .columns([
                ItemCategories::Id,
                ItemCategories::Name,
                ItemCategories::Description,
                ItemCategories::State,
                ItemCategories::CreatedAt,
                ItemCategories::UpdatedAt,
            ])
            .and_where(Expr::col(ItemCategories::Id).eq(self.category.id.to_string()));

        let existing = service.db_adapter.query_optional::<ItemGroup>(&select_stmt).await?;

        if existing.is_none() {
            return Err(Error::NotFoundError);
        }

        let now = Utc::now().naive_utc();

        // Build the update query
        let mut update_query = Query::update();
        let update_stmt = update_query
            .table(ItemCategories::Table)
            .and_where(Expr::col(ItemCategories::Id).eq(self.category.id.to_string()))
            .value(ItemCategories::UpdatedAt, now.to_string());

        // Add optional fields if they exist
        if let Some(name) = &self.category.name {
            update_stmt.value(ItemCategories::Name, name.clone());
        }

        if let Some(description) = &self.category.description {
            match description {
                Some(desc) => update_stmt.value(ItemCategories::Description, desc.clone()),
                None => update_stmt.value(ItemCategories::Description, "NULL"),
            };
        }

        if let Some(state) = &self.category.state {
            update_stmt.value(ItemCategories::State, state.to_string());
        }

        service.db_adapter.update_many(&update_stmt).await?;

        // Retrieve the updated category
        let mut updated_query = Query::select();
        let updated_stmt = updated_query
            .from(ItemCategories::Table)
            .columns([
                ItemCategories::Id,
                ItemCategories::Name,
                ItemCategories::Description,
                ItemCategories::State,
                ItemCategories::CreatedAt,
                ItemCategories::UpdatedAt,
            ])
            .and_where(Expr::col(ItemCategories::Id).eq(self.category.id.to_string()));

        let updated_cat = service.db_adapter.query_one::<ItemGroup>(&updated_stmt).await?;
        Ok(updated_cat)
    }
}

impl Command for DeleteItemGroupCommand {
    type Output = i32;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Check if category has items
        use crate::core::models::catalog::item_model::Items;

        let mut count_query = Query::select();
        let count_stmt = count_query
            .from(Items::Table)
            .expr(Expr::count(Expr::col(Items::Id)))
            .and_where(Expr::col(Items::CategoryId).eq(self.id.to_string()));

        // Execute the count query
        let count_result = service.db_adapter.query_one::<i64>(&count_stmt).await?;

        if count_result > 0 {
            return Err(Error::ForeignKeyConstraintError);
        }

        // Delete the category
        let mut delete_query = Query::delete();
        let delete_stmt = delete_query
            .from_table(ItemCategories::Table)
            .and_where(Expr::col(ItemCategories::Id).eq(self.id.to_string()));

        let affected_rows = service.db_adapter.delete(&delete_stmt).await?;

        Ok(affected_rows as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{commands::tests::setup_service, models::catalog::item_group_model::ItemGroupState};
    use uuid::Uuid;
    use tokio;

    #[tokio::test]
    async fn test_create_item_category() {
        let mut app_service = setup_service();
        let new_cat = ItemGroupNew {
            name: "test".to_string(),
            description: Some("test description".to_string()),
        };
        let command = CreateItemGroupCommand { category: new_cat };
        let result = command.exec(&mut app_service).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_item_category_already_exists() {
        let mut app_service = setup_service();
        let new_cat = ItemGroupNew {
            name: "test".to_string(),
            description: Some("test description".to_string()),
        };
        let command = CreateItemGroupCommand {
            category: new_cat.clone(),
        };
        let result = command.exec(&mut app_service).await;

        assert!(result.is_ok());

        let command = CreateItemGroupCommand { category: new_cat };
        let result = command.exec(&mut app_service).await;

        assert!(matches!(result, Err(Error::UniqueConstraintError)));
    }

    #[tokio::test]
    async fn test_update_item_category() {
        let mut app_service = setup_service();
        let new_cat = ItemGroupNew {
            name: "test".to_string(),
            description: Some("test description".to_string()),
        };
        let create_command = CreateItemGroupCommand { category: new_cat };
        let category = create_command.exec(&mut app_service).await.unwrap();

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
        let result = update_command.exec(&mut app_service).await;
        assert!(result.is_ok());
        assert!(result.unwrap().name == "updated test");
    }

    #[tokio::test]
    async fn test_update_item_category_does_not_exist() {
        let mut app_service = setup_service();
        let now = Utc::now().naive_utc();
        let category = ItemGroupUpdate {
            id: Uuid::now_v7().into(),
            name: Some("test".to_string()),
            description: Some(Some("test description".to_string())),
            state: Some(ItemGroupState::Active),
            updated_at: Some(now),
        };

        let command = UpdateItemGroupCommand { category };
        let result = command.exec(&mut app_service).await;
        assert!(matches!(result, Err(Error::NotFoundError)))
    }

    #[tokio::test]
    async fn test_delete_item_category() {
        let mut app_service = setup_service();
        let new_cat = ItemGroupNew {
            name: "test".to_string(),
            description: Some("test description".to_string()),
        };

        let create_command = CreateItemGroupCommand { category: new_cat };
        let cat = create_command.exec(&mut app_service).await.unwrap();

        let delete_command = DeleteItemGroupCommand { id: cat.id };
        let result = delete_command.exec(&mut app_service).await;
        assert!(result.is_ok());
    }
}
