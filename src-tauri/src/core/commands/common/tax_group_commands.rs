use chrono::Utc;
use sea_query::{Expr, Query, SqliteQueryBuilder};
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

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.db_adapter.transaction(|db| {
            // Verify all taxes exist if tax_ids are provided
            if let Some(tax_ids) = &self.tax_group.tax_ids {
                for tax_id in tax_ids {
                    let tax_query = Query::select()
                        .from(Taxes::Table)
                        .columns([Taxes::Id])
                        .and_where(Expr::col(Taxes::Id).eq(tax_id.to_string()))
                        .to_string(SqliteQueryBuilder);

                    let tax = db.query_optional::<Tax>(&tax_query, vec![])?;
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
            let tax_group_query = Query::insert()
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
                ])
                .to_string(SqliteQueryBuilder);

            db.execute(&tax_group_query, vec![])?;

            // If tax_ids are provided, assign them to the tax group
            if let Some(tax_ids) = &self.tax_group.tax_ids {
                for tax_id in tax_ids {
                    let tax_group_tax = TaxGroupTax {
                        tax_group_id: new_tax_group.id,
                        tax_id: *tax_id,
                    };

                    let tax_group_tax_query = Query::insert()
                        .into_table(TaxGroupTaxes::Table)
                        .columns([
                            TaxGroupTaxes::TaxGroupId,
                            TaxGroupTaxes::TaxId,
                        ])
                        .values_panic([
                            tax_group_tax.tax_group_id.to_string().into(),
                            tax_group_tax.tax_id.to_string().into(),
                        ])
                        .to_string(SqliteQueryBuilder);

                    db.execute(&tax_group_tax_query, vec![])?;
                }
            }

            Ok(new_tax_group)
        })
    }
}

impl Command for UpdateTaxGroupCommand {
    type Output = TaxGroup;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.db_adapter.transaction(|db| {
            // Check if the tax group exists
            let query = Query::select()
                .from(TaxGroups::Table)
                .columns([
                    TaxGroups::Id,
                    TaxGroups::Name,
                    TaxGroups::Description,
                    TaxGroups::CreatedAt,
                    TaxGroups::UpdatedAt,
                ])
                .and_where(Expr::col(TaxGroups::Id).eq(self.tax_group.id.to_string()))
                .to_string(SqliteQueryBuilder);

            let tax_group = db.query_optional::<TaxGroup>(&query, vec![])?;
            if tax_group.is_none() {
                return Err(Error::NotFoundError);
            }
            let tax_group = tax_group.unwrap();

            let now = Utc::now().naive_utc();

            // Build update query
            let mut update_query = Query::update();
            let update = update_query
                .table(TaxGroups::Table)
                .and_where(Expr::col(TaxGroups::Id).eq(self.tax_group.id.to_string()))
                .value(TaxGroups::UpdatedAt, now.to_string());

            if let Some(name) = &self.tax_group.name {
                update.value(TaxGroups::Name, name.clone());
            }

            if let Some(description) = &self.tax_group.description {
                match description {
                    Some(desc) => update.value(TaxGroups::Description, desc.clone()),
                    None => update.value(TaxGroups::Description, sea_query::Value::String(None)),
                };
            }

            let sql = update.to_string(SqliteQueryBuilder);
            db.execute(&sql, vec![])?;

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
        })
    }
}

impl Command for DeleteTaxGroupCommand {
    type Output = i32;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.db_adapter.transaction(|db| {
            // Check if the tax group is used in any sales order charges
            let exists_query = format!(
                "SELECT COUNT(*) FROM sales_order_charges WHERE tax_group_id = '{}'",
                self.id.to_string()
            );

            let count: i64 = db.query_one(&exists_query, vec![])?;

            if count > 0 {
                return Err(Error::HasChildrenError);
            }

            // Delete all tax group tax associations
            let delete_associations_query = Query::delete()
                .from_table(TaxGroupTaxes::Table)
                .and_where(Expr::col(TaxGroupTaxes::TaxGroupId).eq(self.id.to_string()))
                .to_string(SqliteQueryBuilder);

            db.execute(&delete_associations_query, vec![])?;

            // Delete the tax group
            let delete_group_query = Query::delete()
                .from_table(TaxGroups::Table)
                .and_where(Expr::col(TaxGroups::Id).eq(self.id.to_string()))
                .to_string(SqliteQueryBuilder);

            let deleted_count = db.execute(&delete_group_query, vec![])?;

            Ok(deleted_count as i32)
        })
    }
}

impl Command for AssignTaxToGroupCommand {
    type Output = i32;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.db_adapter.transaction(|db| {
            // Verify the tax group exists
            let tax_group_query = Query::select()
                .from(TaxGroups::Table)
                .columns([TaxGroups::Id])
                .and_where(Expr::col(TaxGroups::Id).eq(self.tax_group_id.to_string()))
                .to_string(SqliteQueryBuilder);

            let tax_group = db.query_optional::<TaxGroup>(&tax_group_query, vec![])?;
            if tax_group.is_none() {
                return Err(Error::NotFoundError);
            }

            // Verify the tax exists
            let tax_query = Query::select()
                .from(Taxes::Table)
                .columns([Taxes::Id])
                .and_where(Expr::col(Taxes::Id).eq(self.tax_id.to_string()))
                .to_string(SqliteQueryBuilder);

            let tax = db.query_optional::<Tax>(&tax_query, vec![])?;
            if tax.is_none() {
                return Err(Error::NotFoundError);
            }

            // Check if the association already exists
            let exists_query = Query::select()
                .from(TaxGroupTaxes::Table)
                .expr(Expr::count(Expr::col(TaxGroupTaxes::TaxGroupId)))
                .and_where(Expr::col(TaxGroupTaxes::TaxGroupId).eq(self.tax_group_id.to_string()))
                .and_where(Expr::col(TaxGroupTaxes::TaxId).eq(self.tax_id.to_string()))
                .to_string(SqliteQueryBuilder);

            let count: i64 = db.query_one(&exists_query, vec![])?;

            if count > 0 {
                return Ok(0); // Association already exists
            }

            // Create the association
            let tax_group_tax = TaxGroupTax {
                tax_group_id: self.tax_group_id,
                tax_id: self.tax_id,
            };

            let insert_query = Query::insert()
                .into_table(TaxGroupTaxes::Table)
                .columns([
                    TaxGroupTaxes::TaxGroupId,
                    TaxGroupTaxes::TaxId,
                ])
                .values_panic([
                    tax_group_tax.tax_group_id.to_string().into(),
                    tax_group_tax.tax_id.to_string().into(),
                ])
                .to_string(SqliteQueryBuilder);

            let rows_affected = db.execute(&insert_query, vec![])?;

            Ok(rows_affected as i32)
        })
    }
}

impl Command for RemoveTaxFromGroupCommand {
    type Output = i32;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.db_adapter.transaction(|db| {
            let query = Query::delete()
                .from_table(TaxGroupTaxes::Table)
                .and_where(Expr::col(TaxGroupTaxes::TaxGroupId).eq(self.tax_group_id.to_string()))
                .and_where(Expr::col(TaxGroupTaxes::TaxId).eq(self.tax_id.to_string()))
                .to_string(SqliteQueryBuilder);

            let result = db.execute(&query, vec![])?;

            if result == 0 {
                return Err(Error::NotFoundError);
            }

            Ok(result as i32)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::commands::common::tax_commands::CreateTaxCommand;
    use crate::core::models::common::tax_model::TaxNewInput;
    use crate::core::types::percentage::Percentage;

    #[test]
    fn test_create_tax_group() {
        let mut service = AppService::new(":memory:");

        // Create a tax first
        let tax_input = TaxNewInput {
            name: "CGST".to_string(),
            rate: Percentage::from_float(9.0),
            description: Some("Central GST".to_string()),
            item_ids: None,
        };
        let tax = CreateTaxCommand { tax: tax_input }
            .exec(&mut service)
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
        .unwrap();

        assert_eq!(tax_group.name, "GST 18%");
        assert_eq!(
            tax_group.description,
            Some("GST with CGST 9% and SGST 9%".to_string())
        );
    }

    #[test]
    fn test_update_tax_group() {
        let mut service = AppService::new(":memory:");

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
        .unwrap();

        assert_eq!(updated_tax_group.name, "GST 18% (Updated)");
        assert_eq!(
            updated_tax_group.description,
            Some("GST with CGST 9% and SGST 9%".to_string())
        );
    }

    #[test]
    fn test_delete_tax_group() {
        let mut service = AppService::new(":memory:");

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
        .unwrap();

        // Delete the tax group
        let result = DeleteTaxGroupCommand { id: tax_group.id }
            .exec(&mut service)
            .unwrap();

        assert_eq!(result, 1); // 1 row affected
    }

    #[test]
    fn test_assign_tax_to_group() {
        let mut service = AppService::new(":memory:");

        // Create a tax
        let tax_input = TaxNewInput {
            name: "SGST".to_string(),
            rate: Percentage::from_float(9.0),
            description: Some("State GST".to_string()),
            item_ids: None,
        };
        let tax = CreateTaxCommand { tax: tax_input }
            .exec(&mut service)
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
        .unwrap();

        // Assign the tax to the group
        let result = AssignTaxToGroupCommand {
            tax_group_id: tax_group.id,
            tax_id: tax.id,
        }
        .exec(&mut service)
        .unwrap();

        assert_eq!(result, 1); // 1 row affected
    }

    #[test]
    fn test_remove_tax_from_group() {
        let mut service = AppService::new(":memory:");

        // Create a tax
        let tax_input = TaxNewInput {
            name: "SGST".to_string(),
            rate: Percentage::from_float(9.0),
            description: Some("State GST".to_string()),
            item_ids: None,
        };
        let tax = CreateTaxCommand { tax: tax_input }
            .exec(&mut service)
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
        .unwrap();

        // Remove the tax from the group
        let result = RemoveTaxFromGroupCommand {
            tax_group_id: tax_group.id,
            tax_id: tax.id,
        }
        .exec(&mut service)
        .unwrap();

        assert_eq!(result, 1); // 1 row affected
    }
}
