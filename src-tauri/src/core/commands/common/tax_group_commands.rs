use chrono::Utc;
use sea_query::{Expr, Func, Iden, Query};
use uuid::Uuid;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        commands::{app_service::AppService, Command},
        models::common::{
            tax_group_model::{TaxGroup, TaxGroupNewInput, TaxGroupTax, TaxGroupTaxes, TaxGroupUpdateInput, TaxGroups},
            tax_model::{Tax, Taxes},
        },
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
};

// Commands
pub struct CreateTaxGroupCommand {
    pub tax_group: TaxGroupNewInput,
}

pub struct UpdateTaxGroupCommand {
    pub tax_group: TaxGroupUpdateInput,
}

pub struct DeleteTaxGroupCommand {
    pub id: DbUuid,
}

pub struct AssignTaxToGroupCommand {
    pub tax_group_id: DbUuid,
    pub tax_id: DbUuid,
}

pub struct RemoveTaxFromGroupCommand {
    pub tax_group_id: DbUuid,
    pub tax_id: DbUuid,
}

// Command Implementations
impl Command for CreateTaxGroupCommand {
    type Output = TaxGroup;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Verify all taxes exist if tax_ids are provided
        if let Some(tax_ids) = &self.tax_group.tax_ids {
            for tax_id in tax_ids {
                let mut query_builder = Query::select();
                let tax_query = query_builder
                    .from(Taxes::Table)
                    .columns([Taxes::Id])
                    .and_where(Expr::col(Taxes::Id).eq(tax_id.to_string()));

                let tax = service.db_adapter.query_optional::<DbUuid>(&tax_query).await?;
                if tax.is_none() {
                    return Err(Error::NotFoundError);
                }
            }
        }

        let now = Utc::now().naive_utc();
        let new_tax_group = TaxGroup {
            id: Uuid::now_v7().into(),
            name: self.tax_group.name.clone(),
            description: self.tax_group.description.clone(),
            created_at: now,
            updated_at: now,
        };

        // Insert the tax group
        let mut insert_query = Query::insert();
        let tax_group_stmt = insert_query
            .into_table(TaxGroups::Table)
            .columns([
                TaxGroups::Id,
                TaxGroups::Name,
                TaxGroups::Description,
                TaxGroups::CreatedAt,
                TaxGroups::UpdatedAt,
            ])
            .values_panic([
                new_tax_group.id.to_string().into(),
                new_tax_group.name.clone().into(),
                match &new_tax_group.description {
                    Some(desc) => desc.clone().into(),
                    None => sea_query::Value::String(None).into(),
                },
                new_tax_group.created_at.to_string().into(),
                new_tax_group.updated_at.to_string().into(),
            ]);

        service.db_adapter.insert_many(&tax_group_stmt).await?;

        // If tax_ids are provided, assign them to the tax group
        if let Some(tax_ids) = &self.tax_group.tax_ids {
            for tax_id in tax_ids {
                let tax_group_tax = TaxGroupTax {
                    tax_group_id: new_tax_group.id,
                    tax_id: *tax_id,
                };

                let mut insert_query = Query::insert();
                let tax_group_tax_stmt = insert_query
                    .into_table(TaxGroupTaxes::Table)
                    .columns([
                        TaxGroupTaxes::TaxGroupId,
                        TaxGroupTaxes::TaxId,
                    ])
                    .values_panic([
                        tax_group_tax.tax_group_id.to_string().into(),
                        tax_group_tax.tax_id.to_string().into(),
                    ]);

                service.db_adapter.insert_many(&tax_group_tax_stmt).await?;
            }
        }

        Ok(new_tax_group)
    }
}

impl Command for UpdateTaxGroupCommand {
    type Output = TaxGroup;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Check if the tax group exists
        let mut query_builder = Query::select();
        let select_stmt = query_builder
            .from(TaxGroups::Table)
            .columns([
                TaxGroups::Id,
                TaxGroups::Name,
                TaxGroups::Description,
                TaxGroups::CreatedAt,
                TaxGroups::UpdatedAt,
            ])
            .and_where(Expr::col(TaxGroups::Id).eq(self.tax_group.id.to_string()));

        let tax_group = service.db_adapter.query_optional::<TaxGroup>(&select_stmt).await?;
        if tax_group.is_none() {
            return Err(Error::NotFoundError);
        }
        let tax_group = tax_group.unwrap();

        let now = Utc::now().naive_utc();

        // Build update query
        let mut update_query = Query::update();
        let update_stmt = update_query
            .table(TaxGroups::Table)
            .and_where(Expr::col(TaxGroups::Id).eq(self.tax_group.id.to_string()))
            .value(TaxGroups::UpdatedAt, now.to_string());

        if let Some(name) = &self.tax_group.name {
            update_stmt.value(TaxGroups::Name, name.clone());
        }

        if let Some(description) = &self.tax_group.description {
            match description {
                Some(desc) => update_stmt.value(TaxGroups::Description, desc.clone()),
                None => update_stmt.value(TaxGroups::Description, sea_query::Value::String(None)),
            };
        }

        service.db_adapter.update_many(&update_stmt).await?;

        // Return the updated tax group
        let updated_tax_group = TaxGroup {
            id: tax_group.id,
            name: self.tax_group.name.clone().unwrap_or(tax_group.name),
            description: match &self.tax_group.description {
                Some(Some(desc)) => Some(desc.clone()),
                Some(None) => None,
                None => tax_group.description,
            },
            created_at: tax_group.created_at,
            updated_at: now,
        };

        Ok(updated_tax_group)
    }
}

impl Command for DeleteTaxGroupCommand {
    type Output = i32;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Check if the tax group is used in any sales order charges
        // Define a temporary Iden for the sales_order_charges table
        #[derive(Iden)]
        enum SalesOrderCharges {
            Table,
            TaxGroupId,
        }

        let mut count_query_builder = Query::select();
        let count_query = count_query_builder
            .expr(Func::count(Expr::col(SalesOrderCharges::TaxGroupId)))
            .from(SalesOrderCharges::Table)
            .and_where(Expr::col(SalesOrderCharges::TaxGroupId).eq(self.id.to_string()));

        let count: i64 = service.db_adapter.query_one(&count_query).await?;

        if count > 0 {
            return Err(Error::HasChildrenError);
        }

        // Delete all tax group tax associations
        let mut delete_assoc_query = Query::delete();
        let delete_associations_stmt = delete_assoc_query
            .from_table(TaxGroupTaxes::Table)
            .and_where(Expr::col(TaxGroupTaxes::TaxGroupId).eq(self.id.to_string()));

        service.db_adapter.delete(&delete_associations_stmt).await?;

        // Delete the tax group
        let mut delete_group_query = Query::delete();
        let delete_group_stmt = delete_group_query
            .from_table(TaxGroups::Table)
            .and_where(Expr::col(TaxGroups::Id).eq(self.id.to_string()));

        let deleted_count = service.db_adapter.delete(&delete_group_stmt).await?;

        Ok(deleted_count as i32)
    }
}

impl Command for AssignTaxToGroupCommand {
    type Output = i32;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Verify the tax group exists
        let mut tax_group_query_builder = Query::select();
        let tax_group_stmt = tax_group_query_builder
            .from(TaxGroups::Table)
            .columns([TaxGroups::Id])
            .and_where(Expr::col(TaxGroups::Id).eq(self.tax_group_id.to_string()));

        let tax_group = service.db_adapter.query_optional::<DbUuid>(&tax_group_stmt).await?;
        if tax_group.is_none() {
            return Err(Error::NotFoundError);
        }

        // Verify the tax exists
        let mut tax_query_builder = Query::select();
        let tax_stmt = tax_query_builder
            .from(Taxes::Table)
            .columns([Taxes::Id])
            .and_where(Expr::col(Taxes::Id).eq(self.tax_id.to_string()));

        let tax = service.db_adapter.query_optional::<DbUuid>(&tax_stmt).await?;
        if tax.is_none() {
            return Err(Error::NotFoundError);
        }

        // Check if the association already exists
        let mut exists_query_builder = Query::select();
        let exists_stmt = exists_query_builder
            .from(TaxGroupTaxes::Table)
            .expr(Func::count(Expr::col(TaxGroupTaxes::TaxGroupId)))
            .and_where(Expr::col(TaxGroupTaxes::TaxGroupId).eq(self.tax_group_id.to_string()))
            .and_where(Expr::col(TaxGroupTaxes::TaxId).eq(self.tax_id.to_string()));

        let count: i64 = service.db_adapter.query_one(&exists_stmt).await?;

        if count > 0 {
            return Ok(0); // Association already exists
        }

        // Create the association
        let tax_group_tax = TaxGroupTax {
            tax_group_id: self.tax_group_id,
            tax_id: self.tax_id,
        };

        let mut insert_query_builder = Query::insert();
        let insert_stmt = insert_query_builder
            .into_table(TaxGroupTaxes::Table)
            .columns([
                TaxGroupTaxes::TaxGroupId,
                TaxGroupTaxes::TaxId,
            ])
            .values_panic([
                tax_group_tax.tax_group_id.to_string().into(),
                tax_group_tax.tax_id.to_string().into(),
            ]);

        let rows_affected = service.db_adapter.insert_many(&insert_stmt).await?;

        Ok(rows_affected as i32)
    }
}

impl Command for RemoveTaxFromGroupCommand {
    type Output = i32;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let mut delete_query = Query::delete();
        let delete_stmt = delete_query
            .from_table(TaxGroupTaxes::Table)
            .and_where(Expr::col(TaxGroupTaxes::TaxGroupId).eq(self.tax_group_id.to_string()))
            .and_where(Expr::col(TaxGroupTaxes::TaxId).eq(self.tax_id.to_string()));

        let result = service.db_adapter.delete(&delete_stmt).await?;

        if result == 0 {
            return Err(Error::NotFoundError);
        }

        Ok(result as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::commands::common::tax_commands::CreateTaxCommand;
    use crate::core::commands::tests::setup_service;
    use crate::core::models::common::tax_model::TaxNewInput;
    use crate::core::types::percentage::Percentage;

    #[tokio::test]
    async fn test_create_tax_group() {
        let mut service = setup_service().await;

        // Create a tax first
        let tax_input = TaxNewInput {
            name: "CGST".to_string(),
            rate: Percentage::from_float(9.0),
            description: Some("Central GST".to_string()),
            item_ids: None,
        };
        let tax = CreateTaxCommand { tax: tax_input }
            .exec(&mut service)
            .await
            .unwrap();

        // Now create a tax group
        let tax_group_input = TaxGroupNewInput {
            name: "GST 18%".to_string(),
            description: Some("GST with CGST 9% and SGST 9%".to_string()),
            tax_ids: Some(vec![tax.id]),
        };

        let tax_group = CreateTaxGroupCommand {
            tax_group: tax_group_input,
        }
        .exec(&mut service)
        .await
        .unwrap();

        assert_eq!(tax_group.name, "GST 18%");
        assert_eq!(
            tax_group.description,
            Some("GST with CGST 9% and SGST 9%".to_string())
        );
    }

    #[tokio::test]
    async fn test_update_tax_group() {
        let mut service = setup_service().await;

        // Create a tax group first
        let tax_group_input = TaxGroupNewInput {
            name: "GST 18%".to_string(),
            description: Some("GST with CGST 9% and SGST 9%".to_string()),
            tax_ids: None,
        };

        let tax_group = CreateTaxGroupCommand {
            tax_group: tax_group_input,
        }
        .exec(&mut service)
        .await
        .unwrap();

        // Update the tax group
        let update_input = TaxGroupUpdateInput {
            id: tax_group.id,
            name: Some("GST 18% (Updated)".to_string()),
            description: None, // Don't change description
        };

        let updated_tax_group = UpdateTaxGroupCommand {
            tax_group: update_input,
        }
        .exec(&mut service)
        .await
        .unwrap();

        assert_eq!(updated_tax_group.name, "GST 18% (Updated)");
        assert_eq!(
            updated_tax_group.description,
            Some("GST with CGST 9% and SGST 9%".to_string())
        );
    }

    #[tokio::test]
    async fn test_delete_tax_group() {
        let mut service = setup_service().await;

        // Create a tax group first
        let tax_group_input = TaxGroupNewInput {
            name: "GST 18%".to_string(),
            description: Some("GST with CGST 9% and SGST 9%".to_string()),
            tax_ids: None,
        };

        let tax_group = CreateTaxGroupCommand {
            tax_group: tax_group_input,
        }
        .exec(&mut service)
        .await
        .unwrap();

        // Delete the tax group
        let result = DeleteTaxGroupCommand { id: tax_group.id }
            .exec(&mut service)
            .await
            .unwrap();

        assert_eq!(result, 1); // 1 row affected
    }

    #[tokio::test]
    async fn test_assign_tax_to_group() {
        let mut service = setup_service().await;

        // Create a tax
        let tax_input = TaxNewInput {
            name: "SGST".to_string(),
            rate: Percentage::from_float(9.0),
            description: Some("State GST".to_string()),
            item_ids: None,
        };
        let tax = CreateTaxCommand { tax: tax_input }
            .exec(&mut service)
            .await
            .unwrap();

        // Create a tax group
        let tax_group_input = TaxGroupNewInput {
            name: "GST 18%".to_string(),
            description: Some("GST with CGST 9% and SGST 9%".to_string()),
            tax_ids: None, // No taxes initially
        };

        let tax_group = CreateTaxGroupCommand {
            tax_group: tax_group_input,
        }
        .exec(&mut service)
        .await
        .unwrap();

        // Assign the tax to the group
        let result = AssignTaxToGroupCommand {
            tax_group_id: tax_group.id,
            tax_id: tax.id,
        }
        .exec(&mut service)
        .await
        .unwrap();

        assert_eq!(result, 1); // 1 row affected
    }

    #[tokio::test]
    async fn test_remove_tax_from_group() {
        let mut service = setup_service().await;

        // Create a tax
        let tax_input = TaxNewInput {
            name: "SGST".to_string(),
            rate: Percentage::from_float(9.0),
            description: Some("State GST".to_string()),
            item_ids: None,
        };
        let tax = CreateTaxCommand { tax: tax_input }
            .exec(&mut service)
            .await
            .unwrap();

        // Create a tax group with the tax
        let tax_group_input = TaxGroupNewInput {
            name: "GST 18%".to_string(),
            description: Some("GST with CGST 9% and SGST 9%".to_string()),
            tax_ids: Some(vec![tax.id]),
        };

        let tax_group = CreateTaxGroupCommand {
            tax_group: tax_group_input,
        }
        .exec(&mut service)
        .await
        .unwrap();

        // Remove the tax from the group
        let result = RemoveTaxFromGroupCommand {
            tax_group_id: tax_group.id,
            tax_id: tax.id,
        }
        .exec(&mut service)
        .await
        .unwrap();

        assert_eq!(result, 1); // 1 row affected
    }
}
