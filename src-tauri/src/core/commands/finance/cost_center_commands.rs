use chrono::Utc;
use sea_query::{Expr, Query, SqliteQueryBuilder};
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

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
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
        let query = Query::insert()
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
            ])
            .to_string(SqliteQueryBuilder);

        // Execute the query
        service.db_adapter.execute(&query, vec![])?;

        // Return the newly created cost center
        Ok(new_cost_center)
    }
}

impl Command for UpdateCostCenterCommand {
    type Output = CostCenter;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let now = Utc::now().naive_utc();
        let cost_center_id = self.cost_center.id;

        // First, get the current cost center
        let get_query = Query::select()
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
            .and_where(Expr::col(CostCenters::Id).eq(cost_center_id.to_string()))
            .to_string(SqliteQueryBuilder);

        let current_cost_center = service.db_adapter.query_one::<CostCenter>(&get_query, vec![])?;

        // Build the update query with SeaQuery
        let mut update_query = Query::update();
        let query = update_query.table(CostCenters::Table);

        // Only set fields that are provided in the update input
        if let Some(name) = &self.cost_center.name {
            query.value(CostCenters::Name, name.clone());
        }

        if let Some(code) = &self.cost_center.code {
            query.value(CostCenters::Code, code.clone());
        }

        if let Some(description) = &self.cost_center.description {
            match description {
                Some(desc) => query.value(CostCenters::Description, desc.clone()),
                None => query.value(CostCenters::Description, sea_query::Value::String(None)),
            };
        }

        if let Some(state) = &self.cost_center.state {
            query.value(CostCenters::State, state.to_string());
        }

        // Always update the updated_at timestamp
        query.value(CostCenters::UpdatedAt, now.to_string());

        // Add the WHERE clause
        query.and_where(Expr::col(CostCenters::Id).eq(cost_center_id.to_string()));

        let sql = query.to_string(SqliteQueryBuilder);

        // Execute the query
        service.db_adapter.execute(&sql, vec![])?;

        // Get the updated cost center
        let updated_cost_center = service.db_adapter.query_one::<CostCenter>(&get_query, vec![])?;

        Ok(updated_cost_center)
    }
}

impl Command for DeleteCostCenterCommand {
    type Output = i32;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Build the delete query with SeaQuery
        let query = Query::delete()
            .from_table(CostCenters::Table)
            .and_where(Expr::col(CostCenters::Id).eq(self.id.to_string()))
            .to_string(SqliteQueryBuilder);

        // Execute the query
        let affected_rows = service.db_adapter.execute(&query, vec![])?;

        Ok(affected_rows as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_query::{Alias, Expr, Query, SqliteQueryBuilder};

    #[test]
    fn test_create_cost_center() {
        let mut service = AppService::new(":memory:");

        let command = CreateCostCenterCommand {
            cost_center: CostCenterNewInput {
                name: "Operations".to_string(),
                code: "OPS".to_string(),
                description: Some("Operations Department".to_string()),
                state: Some(CostCenterState::Active),
            },
        };

        let cost_center = command.exec(&mut service).unwrap();
        assert_eq!(cost_center.name, "Operations");
        assert_eq!(cost_center.code, "OPS");
        assert_eq!(
            cost_center.description,
            Some("Operations Department".to_string())
        );
        assert_eq!(cost_center.state, CostCenterState::Active);
    }

    #[test]
    fn test_create_cost_center_minimal() {
        let mut service = AppService::new(":memory:");

        let command = CreateCostCenterCommand {
            cost_center: CostCenterNewInput {
                name: "IT".to_string(),
                code: "IT".to_string(),
                description: None,
                state: None,
            },
        };

        let cost_center = command.exec(&mut service).unwrap();
        assert_eq!(cost_center.name, "IT");
        assert_eq!(cost_center.code, "IT");
        assert_eq!(cost_center.description, None);
        assert_eq!(cost_center.state, CostCenterState::Active); // Default
    }

    #[test]
    fn test_update_cost_center() {
        let mut service = AppService::new(":memory:");

        // Create cost center
        let create_command = CreateCostCenterCommand {
            cost_center: CostCenterNewInput {
                name: "Sales".to_string(),
                code: "SLS".to_string(),
                description: Some("Sales Department".to_string()),
                state: Some(CostCenterState::Active),
            },
        };

        let cost_center = create_command.exec(&mut service).unwrap();

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

        let updated_cost_center = update_command.exec(&mut service).unwrap();
        assert_eq!(updated_cost_center.name, "Sales & Marketing");
        assert_eq!(updated_cost_center.code, "S&M");
        assert_eq!(
            updated_cost_center.description,
            Some("Sales Department".to_string())
        ); // Unchanged
        assert_eq!(updated_cost_center.state, CostCenterState::Active); // Unchanged
    }

    #[test]
    fn test_update_cost_center_remove_field() {
        let mut service = AppService::new(":memory:");

        // Create cost center
        let create_command = CreateCostCenterCommand {
            cost_center: CostCenterNewInput {
                name: "Marketing".to_string(),
                code: "MKT".to_string(),
                description: Some("Marketing Department".to_string()),
                state: Some(CostCenterState::Active),
            },
        };

        let cost_center = create_command.exec(&mut service).unwrap();

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

        let updated_cost_center = update_command.exec(&mut service).unwrap();
        assert_eq!(updated_cost_center.name, "Marketing"); // Unchanged
        assert_eq!(updated_cost_center.code, "MKT"); // Unchanged
        assert_eq!(updated_cost_center.description, None); // Removed
        assert_eq!(updated_cost_center.state, CostCenterState::Active); // Unchanged
    }

    #[test]
    fn test_update_nonexistent_cost_center() {
        let mut service = AppService::new(":memory:");

        let update_command = UpdateCostCenterCommand {
            cost_center: CostCenterUpdateInput {
                id: Uuid::now_v7().into(),
                name: Some("Updated Name".to_string()),
                code: None,
                description: None,
                state: None,
            },
        };

        let result = update_command.exec(&mut service);
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_cost_center() {
        let mut service = AppService::new(":memory:");

        // Create cost center
        let create_command = CreateCostCenterCommand {
            cost_center: CostCenterNewInput {
                name: "Admin".to_string(),
                code: "ADM".to_string(),
                description: None,
                state: None,
            },
        };

        let cost_center = create_command.exec(&mut service).unwrap();

        // Delete cost center
        let delete_command = DeleteCostCenterCommand { id: cost_center.id };
        let result = delete_command.exec(&mut service).unwrap();
        assert_eq!(result, 1);

        // Verify cost center no longer exists
        let count_query = Query::select()
            .from(CostCenters::Table)
            .expr_as(Expr::col(CostCenters::Id).count(), Alias::new("count"))
            .and_where(Expr::col(CostCenters::Id).eq(cost_center.id.to_string()))
            .to_string(SqliteQueryBuilder);
            
        let count = service.db_adapter.query_one::<i64>(&count_query, vec![]).unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_delete_nonexistent_cost_center() {
        let mut service = AppService::new(":memory:");

        // Delete non-existent cost center
        let delete_command = DeleteCostCenterCommand {
            id: Uuid::now_v7().into(),
        };
        let result = delete_command.exec(&mut service).unwrap();
        assert_eq!(result, 0); // No rows affected
    }
}