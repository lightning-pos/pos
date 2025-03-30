use chrono::Utc;
use diesel::{Connection, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{
    core::{
        commands::{app_service::AppService, Command},
        models::finance::cost_center_model::{
            CostCenter, CostCenterNewInput, CostCenterState, CostCenterUpdateChangeset,
            CostCenterUpdateInput,
        },
        types::db_uuid::DbUuid,
    },
    error::Result,
    schema::cost_centers,
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
        service.conn.transaction(|conn| {
            let now = Utc::now().naive_utc();
            let new_cost_center = CostCenter {
                id: Uuid::now_v7().into(),
                name: self.cost_center.name.clone(),
                code: self.cost_center.code.clone(),
                description: self.cost_center.description.clone(),
                state: self.cost_center.state.unwrap_or(CostCenterState::Active),
                created_at: now,
                updated_at: now,
            };

            let res = diesel::insert_into(cost_centers::table)
                .values(&new_cost_center)
                .returning(CostCenter::as_returning())
                .get_result(conn)?;

            Ok(res)
        })
    }
}

impl Command for UpdateCostCenterCommand {
    type Output = CostCenter;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let now = Utc::now().naive_utc();
            let cost_center_id = self.cost_center.id;

            let changeset = CostCenterUpdateChangeset {
                id: cost_center_id,
                name: self.cost_center.name.clone(),
                code: self.cost_center.code.clone(),
                description: self.cost_center.description.clone(),
                state: self.cost_center.state,
                updated_at: now,
            };

            let res = diesel::update(cost_centers::table.find(cost_center_id))
                .set(changeset)
                .returning(CostCenter::as_returning())
                .get_result(conn)?;

            Ok(res)
        })
    }
}

impl Command for DeleteCostCenterCommand {
    type Output = i32;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let res = diesel::delete(cost_centers::table.find(self.id)).execute(conn)?;
            Ok(res as i32)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

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
        let count: i64 = cost_centers::table
            .filter(cost_centers::id.eq(cost_center.id))
            .count()
            .get_result(&mut service.conn)
            .unwrap();
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
