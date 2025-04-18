use chrono::Utc;
use sea_query::{Alias, Expr, Query, SqliteQueryBuilder};
use uuid::Uuid;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        commands::{app_service::AppService, Command},
        models::sales::{
            sales_charge_type_model::{SalesChargeType, SalesChargeTypeNewInput, SalesChargeTypeUpdateInput, SalesChargeTypes},
            sales_order_charge_model::SalesOrderCharges,
        },
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
};

// Commands
pub struct CreateSalesChargeTypeCommand {
    pub charge_type: SalesChargeTypeNewInput,
}

pub struct UpdateSalesChargeTypeCommand {
    pub charge_type: SalesChargeTypeUpdateInput,
}

pub struct DeleteSalesChargeTypeCommand {
    pub id: DbUuid,
}

// Command Implementations
impl Command for CreateSalesChargeTypeCommand {
    type Output = SalesChargeType;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let now = Utc::now().naive_utc();
        let new_id = Uuid::now_v7();

        let new_charge_type = SalesChargeType {
            id: new_id.into(),
            name: self.charge_type.name.clone(),
            description: self.charge_type.description.clone(),
            created_at: now,
            updated_at: now,
        };

        // Build the insert query with SeaQuery
        let query = Query::insert()
            .into_table(SalesChargeTypes::Table)
            .columns([
                SalesChargeTypes::Id,
                SalesChargeTypes::Name,
                SalesChargeTypes::Description,
                SalesChargeTypes::CreatedAt,
                SalesChargeTypes::UpdatedAt,
            ])
            .values_panic([
                new_id.to_string().into(),
                self.charge_type.name.clone().into(),
                self.charge_type.description.clone().into(),
                now.to_string().into(),
                now.to_string().into(),
            ])
            .to_string(SqliteQueryBuilder);

        // Execute the query
        service.db_adapter.execute(&query, vec![])?;

        // Return the newly created charge type
        Ok(new_charge_type)
    }
}

impl Command for UpdateSalesChargeTypeCommand {
    type Output = SalesChargeType;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let now = Utc::now().naive_utc();
        let charge_type_id = self.charge_type.id;

        // First, check if the charge type exists
        let check_query = Query::select()
            .from(SalesChargeTypes::Table)
            .columns([
                SalesChargeTypes::Id,
                SalesChargeTypes::Name,
                SalesChargeTypes::Description,
                SalesChargeTypes::CreatedAt,
                SalesChargeTypes::UpdatedAt,
            ])
            .and_where(Expr::col(SalesChargeTypes::Id).eq(charge_type_id.to_string()))
            .to_string(SqliteQueryBuilder);

        let existing = service.db_adapter.query_optional::<SalesChargeType>(&check_query, vec![])?;

        if existing.is_none() {
            return Err(Error::NotFoundError);
        }

        // Build the update query with SeaQuery
        let mut update_query = Query::update();
        let query = update_query.table(SalesChargeTypes::Table);

        // Only set fields that are provided in the update input
        if let Some(name) = &self.charge_type.name {
            query.value(SalesChargeTypes::Name, name.clone());
        }

        if let Some(description) = &self.charge_type.description {
            match description {
                Some(desc) => query.value(SalesChargeTypes::Description, desc.clone()),
                None => query.value(SalesChargeTypes::Description, sea_query::Value::String(None)),
            };
        }

        // Always update the updated_at timestamp
        query.value(SalesChargeTypes::UpdatedAt, now.to_string());

        // Add the WHERE clause
        query.and_where(Expr::col(SalesChargeTypes::Id).eq(charge_type_id.to_string()));

        let sql = query.to_string(SqliteQueryBuilder);

        // Execute the query
        service.db_adapter.execute(&sql, vec![])?;

        // Get the updated charge type
        let updated_charge_type = service.db_adapter.query_one::<SalesChargeType>(&check_query, vec![])?;

        Ok(updated_charge_type)
    }
}

impl Command for DeleteSalesChargeTypeCommand {
    type Output = bool;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Check if the charge type is used in any sales order charges
        let count_query = Query::select()
            .from(SalesOrderCharges::Table)
            .expr_as(Expr::col(SalesOrderCharges::Id).count(), Alias::new("count"))
            .and_where(Expr::col(SalesOrderCharges::ChargeTypeId).eq(self.id.to_string()))
            .to_string(SqliteQueryBuilder);

        let count = service.db_adapter.query_one::<i64>(&count_query, vec![])?;

        if count > 0 {
            return Err(Error::HasChildrenError);
        }

        // Build the delete query with SeaQuery
        let query = Query::delete()
            .from_table(SalesChargeTypes::Table)
            .and_where(Expr::col(SalesChargeTypes::Id).eq(self.id.to_string()))
            .to_string(SqliteQueryBuilder);

        // Execute the query
        let affected_rows = service.db_adapter.execute(&query, vec![])?;

        Ok(affected_rows > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::commands::tests::setup_service;
    use sea_query::{Expr, Query, SqliteQueryBuilder};

    #[test]
    fn test_create_sales_charge_type() {
        let mut service = setup_service();

        let input = SalesChargeTypeNewInput {
            name: "Service Charge".to_string(),
            description: Some("A charge for service".to_string()),
        };

        let cmd = CreateSalesChargeTypeCommand { charge_type: input };
        let result = cmd.exec(&mut service).unwrap();

        assert_eq!(result.name, "Service Charge");
        assert_eq!(result.description, Some("A charge for service".to_string()));
    }

    #[test]
    fn test_update_sales_charge_type() {
        let mut service = setup_service();

        // Create first
        let input = SalesChargeTypeNewInput {
            name: "Initial Charge".to_string(),
            description: None,
        };
        let cmd = CreateSalesChargeTypeCommand { charge_type: input };
        let created = cmd.exec(&mut service).unwrap();

        // Update
        let update_input = SalesChargeTypeUpdateInput {
            id: created.id,
            name: Some("Updated Charge".to_string()),
            description: Some(Some("Updated Description".to_string())), // Set description
        };
        let update_cmd = UpdateSalesChargeTypeCommand {
            charge_type: update_input,
        };
        let updated = update_cmd.exec(&mut service).unwrap();

        assert_eq!(updated.id, created.id);
        assert_eq!(updated.name, "Updated Charge");
        assert_eq!(updated.description, Some("Updated Description".to_string()));

        // Update again - remove description
        let update_input2 = SalesChargeTypeUpdateInput {
            id: created.id,
            name: None,              // Keep name the same
            description: Some(None), // Remove description
        };
        let update_cmd2 = UpdateSalesChargeTypeCommand {
            charge_type: update_input2,
        };
        let updated2 = update_cmd2.exec(&mut service).unwrap();

        assert_eq!(updated2.name, "Updated Charge");
        assert!(updated2.description.is_none());
    }

    #[test]
    fn test_update_non_existent_charge_type() {
        let mut service = setup_service();

        let update_input = SalesChargeTypeUpdateInput {
            id: Uuid::now_v7().into(),
            name: Some("Doesn't Matter".to_string()),
            description: None,
        };
        let update_cmd = UpdateSalesChargeTypeCommand {
            charge_type: update_input,
        };
        let result = update_cmd.exec(&mut service);

        assert!(result.is_err());
    }

    #[test]
    fn test_delete_sales_charge_type() {
        let mut service = setup_service();

        // Create a charge type first
        let input = SalesChargeTypeNewInput {
            name: "Test Charge".to_string(),
            description: None,
        };
        let cmd = CreateSalesChargeTypeCommand { charge_type: input };
        let created = cmd.exec(&mut service).unwrap();

        // Delete it
        let delete_cmd = DeleteSalesChargeTypeCommand { id: created.id };
        let result = delete_cmd.exec(&mut service).unwrap();

        assert!(result);

        // Verify it's gone
        let check_query = Query::select()
            .from(SalesChargeTypes::Table)
            .expr_as(Expr::col(SalesChargeTypes::Id).count(), Alias::new("count"))
            .and_where(Expr::col(SalesChargeTypes::Id).eq(created.id.to_string()))
            .to_string(SqliteQueryBuilder);

        let count = service.db_adapter.query_one::<i64>(&check_query, vec![]).unwrap();
        assert_eq!(count, 0);
    }
}
