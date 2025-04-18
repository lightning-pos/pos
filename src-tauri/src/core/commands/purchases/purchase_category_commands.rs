use chrono::Utc;
use sea_query::{Expr, Query, SqliteQueryBuilder};
use uuid::Uuid;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        commands::{app_service::AppService, Command},
        models::purchases::purchase_category_model::{
            PurchaseCategory, PurchaseCategoryNew, PurchaseCategoryState, PurchaseCategoryUpdate, PurchaseCategories,
        },
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
};

// Commands
pub struct CreatePurchaseCategoryCommand {
    pub category: PurchaseCategoryNew,
}

pub struct UpdatePurchaseCategoryCommand {
    pub category: PurchaseCategoryUpdate,
}

pub struct DeletePurchaseCategoryCommand {
    pub id: DbUuid,
}

// Command Implementations
impl Command for CreatePurchaseCategoryCommand {
    type Output = PurchaseCategory;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Check if a category with the same name already exists
        let check_query = Query::select()
            .from(PurchaseCategories::Table)
            .columns([PurchaseCategories::Id])
            .and_where(Expr::col(PurchaseCategories::Name).eq(&self.category.name))
            .to_string(SqliteQueryBuilder);

        let existing = service.db_adapter.query_optional::<DbUuid>(&check_query, vec![])?;

        if existing.is_some() {
            return Err(Error::UniqueConstraintError);
        }

        let now = Utc::now().naive_utc();
        let new_id = Uuid::now_v7();

        let new_category = PurchaseCategory {
            id: new_id.into(),
            name: self.category.name.clone(),
            description: self.category.description.clone(),
            state: self.category.state.unwrap_or(PurchaseCategoryState::Active),
            created_at: now,
            updated_at: now,
        };

        // Build the insert query with SeaQuery
        let query = Query::insert()
            .into_table(PurchaseCategories::Table)
            .columns([
                PurchaseCategories::Id,
                PurchaseCategories::Name,
                PurchaseCategories::Description,
                PurchaseCategories::State,
                PurchaseCategories::CreatedAt,
                PurchaseCategories::UpdatedAt,
            ])
            .values_panic([
                new_id.to_string().into(),
                self.category.name.clone().into(),
                self.category.description.clone().into(),
                self.category.state.unwrap_or(PurchaseCategoryState::Active).to_string().into(),
                now.to_string().into(),
                now.to_string().into(),
            ])
            .to_string(SqliteQueryBuilder);

        // Execute the query
        service.db_adapter.execute(&query, vec![])?;

        // Return the newly created category
        Ok(new_category)
    }
}

impl Command for UpdatePurchaseCategoryCommand {
    type Output = PurchaseCategory;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Check if the category exists
        let check_query = Query::select()
            .from(PurchaseCategories::Table)
            .columns([
                PurchaseCategories::Id,
                PurchaseCategories::Name,
                PurchaseCategories::Description,
                PurchaseCategories::State,
                PurchaseCategories::CreatedAt,
                PurchaseCategories::UpdatedAt,
            ])
            .and_where(Expr::col(PurchaseCategories::Id).eq(self.category.id.to_string()))
            .to_string(SqliteQueryBuilder);

        let existing = service.db_adapter.query_optional::<PurchaseCategory>(&check_query, vec![])?;

        if existing.is_none() {
            return Err(Error::NotFoundError);
        }

        let now = Utc::now().naive_utc();

        // Build the update query with SeaQuery
        let mut update_query = Query::update();
        let query = update_query.table(PurchaseCategories::Table);

        // Only set fields that are provided in the update input
        if let Some(name) = &self.category.name {
            query.value(PurchaseCategories::Name, name.clone());
        }

        if let Some(description) = &self.category.description {
            match description {
                Some(desc) => query.value(PurchaseCategories::Description, desc.clone()),
                None => query.value(PurchaseCategories::Description, sea_query::Value::String(None)),
            };
        }

        if let Some(state) = &self.category.state {
            query.value(PurchaseCategories::State, state.to_string());
        }

        // Always update the updated_at timestamp
        query.value(PurchaseCategories::UpdatedAt, now.to_string());

        // Add the WHERE clause
        query.and_where(Expr::col(PurchaseCategories::Id).eq(self.category.id.to_string()));

        let sql = query.to_string(SqliteQueryBuilder);

        // Execute the query
        service.db_adapter.execute(&sql, vec![])?;

        // Get the updated category
        let updated_category = service.db_adapter.query_one::<PurchaseCategory>(&check_query, vec![])?;

        Ok(updated_category)
    }
}

impl Command for DeletePurchaseCategoryCommand {
    type Output = i32;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Check if the category exists
        let check_query = Query::select()
            .from(PurchaseCategories::Table)
            .columns([PurchaseCategories::Id])
            .and_where(Expr::col(PurchaseCategories::Id).eq(self.id.to_string()))
            .to_string(SqliteQueryBuilder);

        let existing = service.db_adapter.query_optional::<DbUuid>(&check_query, vec![])?;

        if existing.is_none() {
            return Err(Error::NotFoundError);
        }

        // Build the delete query with SeaQuery
        let query = Query::delete()
            .from_table(PurchaseCategories::Table)
            .and_where(Expr::col(PurchaseCategories::Id).eq(self.id.to_string()))
            .to_string(SqliteQueryBuilder);

        // Execute the query
        let affected_rows = service.db_adapter.execute(&query, vec![])?;

        Ok(affected_rows as i32)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::commands::tests::setup_service;

    use super::*;
    use sea_query::{Alias, Expr, Query, SqliteQueryBuilder};

    #[test]
    fn test_create_purchase_category() {
        let mut service = setup_service();

        let command = CreatePurchaseCategoryCommand {
            category: PurchaseCategoryNew {
                name: "Test Category".to_string(),
                description: Some("This is a test category".to_string()),
                state: None,
            },
        };

        let category = command.exec(&mut service).unwrap();
        assert_eq!(category.name, "Test Category");
        assert_eq!(
            category.description,
            Some("This is a test category".to_string())
        );
        assert_eq!(category.state, PurchaseCategoryState::Active);
    }

    #[test]
    fn test_create_duplicate_category() {
        let mut service = setup_service();

        // Create first category
        let command1 = CreatePurchaseCategoryCommand {
            category: PurchaseCategoryNew {
                name: "Test Category".to_string(),
                description: None,
                state: None,
            },
        };
        command1.exec(&mut service).unwrap();

        // Try to create duplicate
        let command2 = CreatePurchaseCategoryCommand {
            category: PurchaseCategoryNew {
                name: "Test Category".to_string(),
                description: None,
                state: None,
            },
        };
        let result = command2.exec(&mut service);
        assert!(matches!(result, Err(Error::UniqueConstraintError)));
    }

    #[test]
    fn test_update_purchase_category() {
        let mut service = setup_service();

        // Create category
        let command = CreatePurchaseCategoryCommand {
            category: PurchaseCategoryNew {
                name: "Test Category".to_string(),
                description: None,
                state: None,
            },
        };
        let category = command.exec(&mut service).unwrap();

        // Update category
        let update_command = UpdatePurchaseCategoryCommand {
            category: PurchaseCategoryUpdate {
                id: category.id,
                name: Some("Updated Category".to_string()),
                description: Some(Some("Updated description".to_string())),
                state: Some(PurchaseCategoryState::Inactive),
                updated_at: None,
            },
        };

        let updated_category = update_command.exec(&mut service).unwrap();
        assert_eq!(updated_category.name, "Updated Category");
        assert_eq!(
            updated_category.description,
            Some("Updated description".to_string())
        );
        assert_eq!(updated_category.state, PurchaseCategoryState::Inactive);
    }

    #[test]
    fn test_update_nonexistent_category() {
        let mut service = setup_service();

        let update_command = UpdatePurchaseCategoryCommand {
            category: PurchaseCategoryUpdate {
                id: Uuid::now_v7().into(),
                name: Some("Updated Category".to_string()),
                description: None,
                state: None,
                updated_at: None,
            },
        };

        let result = update_command.exec(&mut service);
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_purchase_category() {
        let mut service = setup_service();

        // Create category
        let command = CreatePurchaseCategoryCommand {
            category: PurchaseCategoryNew {
                name: "Test Category".to_string(),
                description: None,
                state: None,
            },
        };
        let category = command.exec(&mut service).unwrap();

        // Delete category
        let delete_command = DeletePurchaseCategoryCommand { id: category.id };
        let result = delete_command.exec(&mut service).unwrap();
        assert_eq!(result, 1);

        // Verify category no longer exists
        let count_query = Query::select()
            .from(PurchaseCategories::Table)
            .expr_as(Expr::col(PurchaseCategories::Id).count(), Alias::new("count"))
            .and_where(Expr::col(PurchaseCategories::Id).eq(category.id.to_string()))
            .to_string(SqliteQueryBuilder);

        let count = service.db_adapter.query_one::<i64>(&count_query, vec![]).unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_delete_nonexistent_category() {
        let mut service = setup_service();

        let delete_command = DeletePurchaseCategoryCommand {
            id: Uuid::now_v7().into(),
        };

        let result = delete_command.exec(&mut service);
        assert!(result.is_err());
    }
}
