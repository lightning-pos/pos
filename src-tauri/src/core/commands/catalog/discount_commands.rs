use chrono::Utc;
use sea_query::{Expr, Query};
use uuid::Uuid;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        commands::{app_service::AppService, Command},
        models::catalog::discount_model::{
            Discount, DiscountNewInput, DiscountState, DiscountUpdateInput, Discounts,
        },
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
};

// --- Command Structs ---

pub struct CreateDiscountCommand {
    pub discount: DiscountNewInput,
}

pub struct UpdateDiscountCommand {
    pub discount: DiscountUpdateInput,
}

pub struct DeleteDiscountCommand {
    pub id: DbUuid,
}

pub struct GetDiscountCommand {
    pub id: DbUuid,
}

pub struct ListDiscountsCommand;

// --- Command Implementations ---

impl Command for CreateDiscountCommand {
    type Output = Discount;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Check for uniqueness by name using SeaQuery
        let mut select_query = Query::select();
        let check_stmt = select_query
            .from(Discounts::Table)
            .column(Discounts::Id)
            .and_where(Expr::col(Discounts::Name).eq(self.discount.name.clone()));

        let existing = service.db_adapter.query_optional::<Discount>(&check_stmt).await?;
        if existing.is_some() {
            return Err(Error::UniqueConstraintError);
        }

        // Create new discount
        let now = Utc::now().naive_utc();
        let discount_id: DbUuid = Uuid::now_v7().into();

        // Build the insert query using a SQL string
        let insert_sql = format!(
            "INSERT INTO discounts (id, name, description, discount_type, value, scope, state, start_date, end_date, created_at, updated_at) \
             VALUES ('{}', '{}', {}, '{}', '{}', '{}', '{}', {}, {}, '{}', '{}')",
            discount_id.to_string(),
            self.discount.name.clone(),
            match &self.discount.description {
                Some(desc) => format!("'{}'", desc),
                None => "NULL".to_string(),
            },
            self.discount.discount_type.to_string(),
            self.discount.value.to_string(),
            self.discount.scope.to_string(),
            self.discount.state.unwrap_or(DiscountState::Active).to_string(),
            match &self.discount.start_date {
                Some(date) => format!("'{}'", date),
                None => "NULL".to_string(),
            },
            match &self.discount.end_date {
                Some(date) => format!("'{}'", date),
                None => "NULL".to_string(),
            },
            now.to_string(),
            now.to_string()
        );

        // Execute the insert query
        service.db_adapter.execute(&insert_sql).await?;

        // Create and return the new discount object
        let new_discount = Discount {
            id: discount_id,
            name: self.discount.name.clone(),
            description: self.discount.description.clone(),
            discount_type: self.discount.discount_type,
            value: self.discount.value,
            scope: self.discount.scope,
            state: self.discount.state.unwrap_or(DiscountState::Active),
            start_date: self.discount.start_date,
            end_date: self.discount.end_date,
            created_at: now,
            updated_at: now,
        };

        Ok(new_discount)
    }
}

impl Command for UpdateDiscountCommand {
    type Output = Discount;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Check if discount exists using SeaQuery
        let mut select_query = Query::select();
        let check_stmt = select_query
            .from(Discounts::Table)
            .column(Discounts::Id)
            .and_where(Expr::col(Discounts::Id).eq(self.discount.id.to_string()));

        let existing = service.db_adapter.query_optional::<Discount>(&check_stmt).await?;
        if existing.is_none() {
            return Err(Error::NotFoundError);
        }

        // Build update query with SeaQuery
        let now = Utc::now().naive_utc();

        // Create the update query
        let mut update_query = Query::update();
        let update_stmt = update_query.table(Discounts::Table)
            .value(Discounts::UpdatedAt, now.to_string());

        // Add optional fields if they exist
        if let Some(name) = &self.discount.name {
            update_stmt.value(Discounts::Name, name.clone());
        }

        if let Some(description) = &self.discount.description {
            match description {
                Some(desc) => update_stmt.value(Discounts::Description, desc.clone()),
                None => update_stmt.value(Discounts::Description, sea_query::Value::String(None)),
            };
        }

        if let Some(discount_type) = &self.discount.discount_type {
            update_stmt.value(Discounts::DiscountType, discount_type.to_string());
        }

        if let Some(value) = &self.discount.value {
            update_stmt.value(Discounts::Value, value.to_string());
        }

        if let Some(scope) = &self.discount.scope {
            update_stmt.value(Discounts::Scope, scope.to_string());
        }

        if let Some(state) = &self.discount.state {
            update_stmt.value(Discounts::State, state.to_string());
        }

        if let Some(start_date) = &self.discount.start_date {
            match start_date {
                Some(date) => update_stmt.value(Discounts::StartDate, date.to_string()),
                None => update_stmt.value(Discounts::StartDate, sea_query::Value::String(None)),
            };
        }

        if let Some(end_date) = &self.discount.end_date {
            match end_date {
                Some(date) => update_stmt.value(Discounts::EndDate, date.to_string()),
                None => update_stmt.value(Discounts::EndDate, sea_query::Value::String(None)),
            };
        }

        // Add WHERE condition
        update_stmt.and_where(Expr::col(Discounts::Id).eq(self.discount.id.to_string()));

        // Execute the update
        service.db_adapter.update_many(&update_stmt).await?;

        // Retrieve the updated discount
        let mut select_query = Query::select();
        let select_stmt = select_query
            .from(Discounts::Table)
            .columns([
                Discounts::Id,
                Discounts::Name,
                Discounts::Description,
                Discounts::DiscountType,
                Discounts::Value,
                Discounts::Scope,
                Discounts::State,
                Discounts::StartDate,
                Discounts::EndDate,
                Discounts::CreatedAt,
                Discounts::UpdatedAt,
            ])
            .and_where(Expr::col(Discounts::Id).eq(self.discount.id.to_string()));

        let discount = service.db_adapter.query_one::<Discount>(&select_stmt).await?;

        Ok(discount)
    }
}

impl Command for DeleteDiscountCommand {
    type Output = usize;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Build delete query with SeaQuery
        let mut delete_query = Query::delete();
        let delete_stmt = delete_query
            .from_table(Discounts::Table)
            .and_where(Expr::col(Discounts::Id).eq(self.id.to_string()));

        // Execute the delete query
        let affected_rows = service.db_adapter.delete(&delete_stmt).await?;

        if affected_rows == 0 {
            Err(Error::NotFoundError)
        } else {
            Ok(affected_rows as usize)
        }
    }
}

impl Command for GetDiscountCommand {
    type Output = Discount;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Build select query with SeaQuery
        let mut select_query = Query::select();
        let select_stmt = select_query
            .from(Discounts::Table)
            .columns([
                Discounts::Id,
                Discounts::Name,
                Discounts::Description,
                Discounts::DiscountType,
                Discounts::Value,
                Discounts::Scope,
                Discounts::State,
                Discounts::StartDate,
                Discounts::EndDate,
                Discounts::CreatedAt,
                Discounts::UpdatedAt,
            ])
            .and_where(Expr::col(Discounts::Id).eq(self.id.to_string()));

        // Execute the select query
        let discount = service.db_adapter.query_one::<Discount>(&select_stmt).await?;

        Ok(discount)
    }
}

impl Command for ListDiscountsCommand {
    type Output = Vec<Discount>;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Build select query with SeaQuery
        let mut select_query = Query::select();
        let select_stmt = select_query
            .from(Discounts::Table)
            .columns([
                Discounts::Id,
                Discounts::Name,
                Discounts::Description,
                Discounts::DiscountType,
                Discounts::Value,
                Discounts::Scope,
                Discounts::State,
                Discounts::StartDate,
                Discounts::EndDate,
                Discounts::CreatedAt,
                Discounts::UpdatedAt,
            ]);

        // Execute the select query
        let discounts = service.db_adapter.query_many::<Discount>(&select_stmt).await?;

        Ok(discounts)
    }
}

// --- Tests ---
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        commands::tests::setup_service, models::catalog::discount_model::{DiscountScope, DiscountType}, types::money::Money
    };

    async fn create_basic_discount(service: &mut AppService) -> Discount {
        let new_discount_info = DiscountNewInput {
            name: "Test Discount".to_string(),
            description: Some("Basic test discount".to_string()),
            discount_type: DiscountType::Percentage,
            value: Money::from_float(10.0),
            scope: DiscountScope::AllItems,
            state: Some(DiscountState::Active),
            start_date: None,
            end_date: None,
        };
        let create_cmd = CreateDiscountCommand {
            discount: new_discount_info,
        };
        create_cmd
            .exec(service)
            .await
            .expect("Failed to create test discount")
    }

    #[tokio::test]
    async fn test_create_discount() {
        let mut service = setup_service();
        let created = create_basic_discount(&mut service).await;

        assert_eq!(created.name, "Test Discount");
        assert_eq!(created.discount_type, DiscountType::Percentage);
        assert_eq!(created.value, Money::from_float(10.0));
        assert_eq!(created.state, DiscountState::Active);
    }

    #[tokio::test]
    async fn test_create_discount_unique_constraint() {
        let mut service = setup_service();
        create_basic_discount(&mut service).await; // Create first one

        // Try creating another with the same name
        let new_discount_info = DiscountNewInput {
            name: "Test Discount".to_string(), // Same name
            description: None,
            discount_type: DiscountType::FixedAmount,
            value: Money::from_float(5.0),
            scope: DiscountScope::AllItems,
            state: None,
            start_date: None,
            end_date: None,
        };
        let create_cmd = CreateDiscountCommand {
            discount: new_discount_info,
        };
        let result = create_cmd.exec(&mut service).await;

        assert!(matches!(result, Err(Error::UniqueConstraintError)));
    }

    #[tokio::test]
    async fn test_get_discount() {
        let mut service = setup_service();
        let created = create_basic_discount(&mut service).await;

        let get_cmd = GetDiscountCommand { id: created.id };
        let fetched = get_cmd.exec(&mut service).await.expect("Failed to get discount");

        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.name, "Test Discount");
    }

    #[tokio::test]
    async fn test_get_discount_not_found() {
        let mut service = setup_service();
        let non_existent_id = Uuid::now_v7().into();
        let get_cmd = GetDiscountCommand {
            id: non_existent_id,
        };
        let result = get_cmd.exec(&mut service).await;

        assert!(matches!(result, Err(Error::NotFoundError)));
    }

    #[tokio::test]
    async fn test_list_discounts() {
        let mut service = setup_service();
        let created1 = create_basic_discount(&mut service).await;

        let list_cmd = ListDiscountsCommand;
        let list1 = list_cmd
            .exec(&mut service)
            .await
            .expect("Failed to list discounts");
        assert_eq!(list1.len(), 1);
        assert_eq!(list1[0].id, created1.id);

        // Add another discount
        let new_discount_info = DiscountNewInput {
            name: "Another Discount".to_string(),
            description: None,
            discount_type: DiscountType::FixedAmount,
            value: Money::from_float(5.0),
            scope: DiscountScope::AllItems,
            state: Some(DiscountState::Inactive),
            start_date: None,
            end_date: None,
        };
        let create_cmd = CreateDiscountCommand {
            discount: new_discount_info,
        };
        let created2 = create_cmd
            .exec(&mut service)
            .await
            .expect("Failed to create second discount");

        let list2 = list_cmd
            .exec(&mut service)
            .await
            .expect("Failed to list discounts again");
        assert_eq!(list2.len(), 2);
        // Check if both IDs are present (order might vary)
        assert!(list2.iter().any(|d| d.id == created1.id));
        assert!(list2.iter().any(|d| d.id == created2.id));
    }

    #[tokio::test]
    async fn test_update_discount() {
        let mut service = setup_service();
        let created = create_basic_discount(&mut service).await;

        let update_info = DiscountUpdateInput {
            id: created.id,
            name: Some("Updated Discount Name".to_string()),
            description: Some(Some("Updated description".to_string())), // Update description
            discount_type: Some(DiscountType::FixedAmount),
            value: Some(Money::from_float(25.50)),
            scope: None, // Keep scope same
            state: Some(DiscountState::Inactive),
            start_date: None,
            end_date: None,
        };
        let update_cmd = UpdateDiscountCommand {
            discount: update_info,
        };
        let updated = update_cmd
            .exec(&mut service)
            .await
            .expect("Failed to update discount");

        assert_eq!(updated.id, created.id);
        assert_eq!(updated.name, "Updated Discount Name");
        assert_eq!(updated.description, Some("Updated description".to_string()));
        assert_eq!(updated.discount_type, DiscountType::FixedAmount);
        assert_eq!(updated.value, Money::from_float(25.50));
        assert_eq!(updated.state, DiscountState::Inactive);
        assert!(updated.updated_at > created.updated_at); // Check timestamp updated
    }

    #[tokio::test]
    async fn test_update_discount_not_found() {
        let mut service = setup_service();
        let non_existent_id = Uuid::now_v7().into();
        let update_info = DiscountUpdateInput {
            id: non_existent_id,
            name: Some("Does not matter".to_string()),
            description: None,
            discount_type: None,
            value: None,
            scope: None,
            state: None,
            start_date: None,
            end_date: None,
        };
        let update_cmd = UpdateDiscountCommand {
            discount: update_info,
        };
        let result = update_cmd.exec(&mut service).await;

        // This should fail because the initial query in the command fails
        assert!(matches!(result, Err(Error::DieselError(_)))); // Or potentially NotFoundError depending on how DieselError maps
    }

    #[tokio::test]
    async fn test_delete_discount() {
        let mut service = setup_service();
        let created = create_basic_discount(&mut service).await;

        // Delete
        let delete_cmd = DeleteDiscountCommand { id: created.id };
        let delete_result = delete_cmd.exec(&mut service).await.expect("Delete failed");
        assert_eq!(delete_result, 1); // 1 row deleted

        // Verify deletion
        let get_cmd = GetDiscountCommand { id: created.id };
        let get_result = get_cmd.exec(&mut service).await;
        assert!(matches!(get_result, Err(Error::NotFoundError)));

        // Verify list is empty
        let list_cmd = ListDiscountsCommand;
        let list = list_cmd
            .exec(&mut service)
            .await
            .expect("List failed after delete");
        assert!(list.is_empty());
    }

    #[tokio::test]
    async fn test_delete_discount_not_found() {
        let mut service = setup_service();
        let non_existent_id = Uuid::now_v7().into();

        let delete_cmd = DeleteDiscountCommand {
            id: non_existent_id,
        };
        let result = delete_cmd.exec(&mut service).await;

        assert!(matches!(result, Err(Error::NotFoundError)));
    }
}
