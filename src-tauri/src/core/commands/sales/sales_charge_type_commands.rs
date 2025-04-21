use chrono::Utc;
use sea_query::{Alias, Expr, Query};
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

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
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
        let mut insert_query = Query::insert();
        let insert_stmt = insert_query
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
            ]);

        // Execute the query
        service.db_adapter.insert_one(&insert_stmt).await?;

        // Return the newly created charge type
        Ok(new_charge_type)
    }
}

impl Command for UpdateSalesChargeTypeCommand {
    type Output = SalesChargeType;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let now = Utc::now().naive_utc();
        let charge_type_id = self.charge_type.id;

        // First, check if the charge type exists
        let mut select_query = Query::select();
        let select_stmt = select_query
            .from(SalesChargeTypes::Table)
            .columns([
                SalesChargeTypes::Id,
                SalesChargeTypes::Name,
                SalesChargeTypes::Description,
                SalesChargeTypes::CreatedAt,
                SalesChargeTypes::UpdatedAt,
            ])
            .and_where(Expr::col(SalesChargeTypes::Id).eq(charge_type_id.to_string()));

        let existing = service.db_adapter.query_optional::<SalesChargeType>(&select_stmt).await?;

        if existing.is_none() {
            return Err(Error::NotFoundError);
        }

        // Build the update query with SeaQuery
        let mut update_query = Query::update();
        let update_stmt = update_query.table(SalesChargeTypes::Table);

        // Only set fields that are provided in the update input
        if let Some(name) = &self.charge_type.name {
            update_stmt.value(SalesChargeTypes::Name, name.clone());
        }

        if let Some(description) = &self.charge_type.description {
            match description {
                Some(desc) => update_stmt.value(SalesChargeTypes::Description, desc.clone()),
                None => update_stmt.value(SalesChargeTypes::Description, sea_query::Value::String(None)),
            };
        }

        // Always update the updated_at timestamp
        update_stmt.value(SalesChargeTypes::UpdatedAt, now.to_string());

        // Add the WHERE clause
        update_stmt.and_where(Expr::col(SalesChargeTypes::Id).eq(charge_type_id.to_string()));

        // Execute the query
        service.db_adapter.update_one(&update_stmt).await?;

        // Get the updated charge type
        let updated_charge_type = service.db_adapter.query_one::<SalesChargeType>(&select_stmt).await?;

        Ok(updated_charge_type)
    }
}

impl Command for DeleteSalesChargeTypeCommand {
    type Output = bool;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Check if the charge type is used in any sales order charges
        let mut count_query = Query::select();
        let count_stmt = count_query
            .from(SalesOrderCharges::Table)
            .expr_as(Expr::col(SalesOrderCharges::Id).count(), Alias::new("count"))
            .and_where(Expr::col(SalesOrderCharges::ChargeTypeId).eq(self.id.to_string()));

        let count = service.db_adapter.query_one::<i64>(&count_stmt).await?;

        if count > 0 {
            return Err(Error::HasChildrenError);
        }

        // Build the delete query with SeaQuery
        let mut delete_query = Query::delete();
        let delete_stmt = delete_query
            .from_table(SalesChargeTypes::Table)
            .and_where(Expr::col(SalesChargeTypes::Id).eq(self.id.to_string()));

        // Execute the query
        let affected_rows = service.db_adapter.delete(&delete_stmt).await?;

        Ok(affected_rows > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::commands::tests::setup_service;
    use sea_query::{Expr, Query};
    use tokio;

    #[tokio::test]
    async fn test_create_sales_charge_type() {
        let mut service = setup_service();

        let input = SalesChargeTypeNewInput {
            name: "Service Charge".to_string(),
            description: Some("A charge for service".to_string()),
        };

        let cmd = CreateSalesChargeTypeCommand { charge_type: input };
        let result = cmd.exec(&mut service).await.unwrap();

        assert_eq!(result.name, "Service Charge");
        assert_eq!(result.description, Some("A charge for service".to_string()));
    }

    #[tokio::test]
    async fn test_update_sales_charge_type() {
        let mut service = setup_service();

        // Create first
        let input = SalesChargeTypeNewInput {
            name: "Initial Charge".to_string(),
            description: None,
        };
        let cmd = CreateSalesChargeTypeCommand { charge_type: input };
        let created = cmd.exec(&mut service).await.unwrap();

        // Update
        let update_input = SalesChargeTypeUpdateInput {
            id: created.id,
            name: Some("Updated Charge".to_string()),
            description: Some(Some("Updated Description".to_string())), // Set description
        };
        let update_cmd = UpdateSalesChargeTypeCommand {
            charge_type: update_input,
        };
        let updated = update_cmd.exec(&mut service).await.unwrap();

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
        let updated2 = update_cmd2.exec(&mut service).await.unwrap();

        assert_eq!(updated2.name, "Updated Charge");
        assert!(updated2.description.is_none());
    }

    #[tokio::test]
    async fn test_update_non_existent_charge_type() {
        let mut service = setup_service();

        let update_input = SalesChargeTypeUpdateInput {
            id: Uuid::now_v7().into(),
            name: Some("Doesn't Matter".to_string()),
            description: None,
        };
        let update_cmd = UpdateSalesChargeTypeCommand {
            charge_type: update_input,
        };
        let result = update_cmd.exec(&mut service).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_sales_charge_type() {
        let mut service = setup_service();

        // Create a charge type first
        let input = SalesChargeTypeNewInput {
            name: "Test Charge".to_string(),
            description: None,
        };
        let cmd = CreateSalesChargeTypeCommand { charge_type: input };
        let created = cmd.exec(&mut service).await.unwrap();

        // Delete it
        let delete_cmd = DeleteSalesChargeTypeCommand { id: created.id };
        let result = delete_cmd.exec(&mut service).await.unwrap();

        assert!(result);

        // Verify it's gone
        let mut check_query = Query::select();
        let count_stmt = check_query
            .from(SalesChargeTypes::Table)
            .expr_as(Expr::col(SalesChargeTypes::Id).count(), Alias::new("count"))
            .and_where(Expr::col(SalesChargeTypes::Id).eq(created.id.to_string()));

        let count = service.db_adapter.query_one::<i64>(&count_stmt).await.unwrap();
        assert_eq!(count, 0);
    }
}
