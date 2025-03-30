use chrono::Utc;
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{
    core::{
        commands::{app_service::AppService, Command},
        models::purchases::purchase_category_model::{
            PurchaseCategory, PurchaseCategoryNew, PurchaseCategoryState, PurchaseCategoryUpdate,
        },
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
    schema::purchase_categories::dsl::*,
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
        service.conn.transaction(|conn| {
            let existing_cat = purchase_categories
                .filter(name.eq(&self.category.name))
                .select(PurchaseCategory::as_select())
                .get_result::<PurchaseCategory>(conn);

            if let Ok(_) = existing_cat {
                return Err(Error::UniqueConstraintError);
            }

            let now = Utc::now().naive_utc();
            let new_cat = PurchaseCategory {
                id: Uuid::now_v7().into(),
                name: self.category.name.clone(),
                description: self.category.description.clone(),
                state: self.category.state.unwrap_or(PurchaseCategoryState::Active),
                created_at: now,
                updated_at: now,
            };

            let cat = diesel::insert_into(purchase_categories)
                .values(&new_cat)
                .returning(PurchaseCategory::as_returning())
                .get_result(conn)?;

            Ok(cat)
        })
    }
}

impl Command for UpdatePurchaseCategoryCommand {
    type Output = PurchaseCategory;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            purchase_categories
                .filter(id.eq(&self.category.id))
                .limit(1)
                .select(PurchaseCategory::as_select())
                .get_result::<PurchaseCategory>(conn)?;

            let now = Utc::now().naive_utc();

            let mut category = self.category.clone();
            category.updated_at = Some(now);

            let cat = diesel::update(purchase_categories.filter(id.eq(&self.category.id)))
                .set(&category)
                .returning(PurchaseCategory::as_returning())
                .get_result(conn)?;

            Ok(cat)
        })
    }
}

impl Command for DeletePurchaseCategoryCommand {
    type Output = i32;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Just check if the category exists
            purchase_categories
                .filter(id.eq(&self.id))
                .limit(1)
                .select(PurchaseCategory::as_select())
                .get_result::<PurchaseCategory>(conn)?;

            let res = diesel::delete(purchase_categories.filter(id.eq(&self.id))).execute(conn)?;

            Ok(res as i32)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::purchase_categories;
    use diesel::{QueryDsl, RunQueryDsl};

    #[test]
    fn test_create_purchase_category() {
        let mut service = AppService::new(":memory:");

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
        let mut service = AppService::new(":memory:");

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
        let mut service = AppService::new(":memory:");

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
        let mut service = AppService::new(":memory:");

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
        let mut service = AppService::new(":memory:");

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
        let count: i64 = purchase_categories::table
            .filter(purchase_categories::dsl::id.eq(category.id))
            .count()
            .get_result(&mut service.conn)
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_delete_nonexistent_category() {
        let mut service = AppService::new(":memory:");

        let delete_command = DeletePurchaseCategoryCommand {
            id: Uuid::now_v7().into(),
        };

        let result = delete_command.exec(&mut service);
        assert!(result.is_err());
    }
}
