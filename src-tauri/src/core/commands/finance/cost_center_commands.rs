use chrono::Utc;
use sea_query::{Expr, Query};
use uuid::Uuid;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        commands::{app_service::AppService, Command},
        models::finance::cost_center_model::{
            CostCenter, CostCenterNewInput, CostCenterState, CostCenterUpdateInput, CostCenters,
        },
        types::db_uuid::DbUuid,
    },
    error::Result,
};

// Commands
pub struct CreateCostCenterCommand {
    pub cost_center: CostCenterNewInput,
}

pub struct UpdateCostCenterCommand {
    pub cost_center: CostCenterUpdateInput,
}

pub struct DeleteCostCenterCommand {
    pub id: DbUuid,
}

// Command Implementations
impl Command for CreateCostCenterCommand {
    type Output = CostCenter;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let now = Utc::now().naive_utc();
        let new_id = Uuid::now_v7();

        let new_cost_center = CostCenter {
            id: new_id.into(),
            name: self.cost_center.name.clone(),
            code: self.cost_center.code.clone(),
            description: self.cost_center.description.clone(),
            state: self.cost_center.state.unwrap_or(CostCenterState::Active),
            created_at: now,
            updated_at: now,
        };

        // Build the insert query with SeaQuery
        let mut insert_query = Query::insert();
        let insert_stmt = insert_query
            .into_table(CostCenters::Table)
            .columns([
                CostCenters::Id,
                CostCenters::Name,
                CostCenters::Code,
                CostCenters::Description,
                CostCenters::State,
                CostCenters::CreatedAt,
                CostCenters::UpdatedAt,
            ])
            .values_panic([
                new_id.to_string().into(),
                self.cost_center.name.clone().into(),
                self.cost_center.code.clone().into(),
                self.cost_center.description.clone().into(),
                self.cost_center.state.unwrap_or(CostCenterState::Active).to_string().into(),
                now.to_string().into(),
                now.to_string().into(),
            ]);

        // Execute the query
        service.db_adapter.insert_many(&insert_stmt).await?;

        // Return the newly created cost center
        Ok(new_cost_center)
    }
}

impl Command for UpdateCostCenterCommand {
    type Output = CostCenter;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let now = Utc::now().naive_utc();
        let cost_center_id = self.cost_center.id;

        // First, get the current cost center
        let mut select_query = Query::select();
        let get_stmt = select_query
            .from(CostCenters::Table)
            .columns([
                CostCenters::Id,
                CostCenters::Name,
                CostCenters::Code,
                CostCenters::Description,
                CostCenters::State,
                CostCenters::CreatedAt,
                CostCenters::UpdatedAt,
            ])
            .and_where(Expr::col(CostCenters::Id).eq(cost_center_id.to_string()));

        // Build the update query with SeaQuery
        let mut update_query = Query::update();
        let update_stmt = update_query.table(CostCenters::Table);

        // Only set fields that are provided in the update input
        if let Some(name) = &self.cost_center.name {
            update_stmt.value(CostCenters::Name, name.clone());
        }

        if let Some(code) = &self.cost_center.code {
            update_stmt.value(CostCenters::Code, code.clone());
        }

        if let Some(description) = &self.cost_center.description {
            match description {
                Some(desc) => update_stmt.value(CostCenters::Description, desc.clone()),
                None => update_stmt.value(CostCenters::Description, sea_query::Value::String(None)),
            };
        }

        if let Some(state) = &self.cost_center.state {
            update_stmt.value(CostCenters::State, state.to_string());
        }

        // Always update the updated_at timestamp
        update_stmt.value(CostCenters::UpdatedAt, now.to_string());

        // Add the WHERE clause
        update_stmt.and_where(Expr::col(CostCenters::Id).eq(cost_center_id.to_string()));

        // Execute the update query
        service.db_adapter.update_many(&update_stmt).await?;

        // Get the updated cost center
        let updated_cost_center = service.db_adapter.query_one::<CostCenter>(&get_stmt).await?;

        Ok(updated_cost_center)
    }
}

impl Command for DeleteCostCenterCommand {
    type Output = i32;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Build the delete query with SeaQuery
        let mut delete_query = Query::delete();
        let delete_stmt = delete_query
            .from_table(CostCenters::Table)
            .and_where(Expr::col(CostCenters::Id).eq(self.id.to_string()));

        // Execute the query
        let affected_rows = service.db_adapter.delete(&delete_stmt).await?;

        Ok(affected_rows as i32)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::commands::tests::setup_service;

    use super::*;
    use sea_query::{Expr, Func, Query};

    #[tokio::test]
    async fn test_create_cost_center() {
        let mut service = setup_service().await;

        let command = CreateCostCenterCommand {
            cost_center: CostCenterNewInput {
                name: "Operations".to_string(),
                code: "OPS".to_string(),
                description: Some("Operations Department".to_string()),
                state: Some(CostCenterState::Active),
            },
        };

        let cost_center = command.exec(&mut service).await.unwrap();
        assert_eq!(cost_center.name, "Operations");
        assert_eq!(cost_center.code, "OPS");
        assert_eq!(
            cost_center.description,
            Some("Operations Department".to_string())
        );
        assert_eq!(cost_center.state, CostCenterState::Active);
    }

    #[tokio::test]
    async fn test_create_cost_center_minimal() {
        let mut service = setup_service().await;

        let command = CreateCostCenterCommand {
            cost_center: CostCenterNewInput {
                name: "IT".to_string(),
                code: "IT".to_string(),
                description: None,
                state: None,
            },
        };

        let cost_center = command.exec(&mut service).await.unwrap();
        assert_eq!(cost_center.name, "IT");
        assert_eq!(cost_center.code, "IT");
        assert_eq!(cost_center.description, None);
        assert_eq!(cost_center.state, CostCenterState::Active); // Default
    }

    #[tokio::test]
    async fn test_update_cost_center() {
        let mut service = setup_service().await;

        // Create cost center
        let create_command = CreateCostCenterCommand {
            cost_center: CostCenterNewInput {
                name: "Sales".to_string(),
                code: "SLS".to_string(),
                description: Some("Sales Department".to_string()),
                state: Some(CostCenterState::Active),
            },
        };

        let cost_center = create_command.exec(&mut service).await.unwrap();

        // Update cost center
        let update_command = UpdateCostCenterCommand {
            cost_center: CostCenterUpdateInput {
                id: cost_center.id,
                name: Some("Sales & Marketing".to_string()),
                code: Some("S&M".to_string()),
                description: None,
                state: None,
            },
        };

        let updated_cost_center = update_command.exec(&mut service).await.unwrap();
        assert_eq!(updated_cost_center.name, "Sales & Marketing");
        assert_eq!(updated_cost_center.code, "S&M");
        assert_eq!(
            updated_cost_center.description,
            Some("Sales Department".to_string())
        ); // Unchanged
        assert_eq!(updated_cost_center.state, CostCenterState::Active); // Unchanged
    }

    #[tokio::test]
    async fn test_update_cost_center_remove_field() {
        let mut service = setup_service().await;

        // Create cost center
        let create_command = CreateCostCenterCommand {
            cost_center: CostCenterNewInput {
                name: "Marketing".to_string(),
                code: "MKT".to_string(),
                description: Some("Marketing Department".to_string()),
                state: Some(CostCenterState::Active),
            },
        };

        let cost_center = create_command.exec(&mut service).await.unwrap();

        // Update cost center - remove description
        let update_command = UpdateCostCenterCommand {
            cost_center: CostCenterUpdateInput {
                id: cost_center.id,
                name: None,
                code: None,
                description: Some(None), // Remove description
                state: None,
            },
        };

        let updated_cost_center = update_command.exec(&mut service).await.unwrap();
        assert_eq!(updated_cost_center.name, "Marketing"); // Unchanged
        assert_eq!(updated_cost_center.code, "MKT"); // Unchanged
        assert_eq!(updated_cost_center.description, None); // Removed
        assert_eq!(updated_cost_center.state, CostCenterState::Active); // Unchanged
    }

    #[tokio::test]
    async fn test_update_nonexistent_cost_center() {
        let mut service = setup_service().await;

        let update_command = UpdateCostCenterCommand {
            cost_center: CostCenterUpdateInput {
                id: Uuid::now_v7().into(),
                name: Some("Updated Name".to_string()),
                code: None,
                description: None,
                state: None,
            },
        };

        let result = update_command.exec(&mut service).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_cost_center() {
        let mut service = setup_service().await;

        // Create cost center
        let create_command = CreateCostCenterCommand {
            cost_center: CostCenterNewInput {
                name: "Admin".to_string(),
                code: "ADM".to_string(),
                description: None,
                state: None,
            },
        };

        let cost_center = create_command.exec(&mut service).await.unwrap();

        // Delete cost center
        let delete_command = DeleteCostCenterCommand { id: cost_center.id };
        let result = delete_command.exec(&mut service).await.unwrap();
        assert_eq!(result, 1);

        // Verify cost center no longer exists
        let mut count_query_builder = Query::select();
        let count_stmt = count_query_builder
            .from(CostCenters::Table)
            .expr(Func::count(Expr::col(CostCenters::Id)))
            .and_where(Expr::col(CostCenters::Id).eq(cost_center.id.to_string()));

        let count: i64 = service.db_adapter.query_one(&count_stmt).await.unwrap();
        assert_eq!(count, 0);
    }

    #[tokio::test]
    async fn test_delete_nonexistent_cost_center() {
        let mut service = setup_service().await;

        // Delete non-existent cost center
        let delete_command = DeleteCostCenterCommand {
            id: Uuid::now_v7().into(),
        };
        let result = delete_command.exec(&mut service).await.unwrap();
        assert_eq!(result, 0); // No rows affected
    }
}
