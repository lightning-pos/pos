use chrono::Utc;
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{
    core::{
        commands::{app_service::AppService, Command},
        models::common::{
            tax_group_model::{TaxGroup, TaxGroupNewInput, TaxGroupTax, TaxGroupUpdateInput},
            tax_model::Tax,
        },
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
    schema::{tax_group_taxes, tax_groups, taxes},
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
        service.conn.transaction(|conn| {
            // Verify all taxes exist if tax_ids are provided
            if let Some(tax_ids) = &self.tax_group.tax_ids {
                for tax_id in tax_ids {
                    taxes::table
                        .filter(taxes::id.eq(tax_id))
                        .select(Tax::as_select())
                        .get_result::<Tax>(conn)?;
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
            diesel::insert_into(tax_groups::table)
                .values(&new_tax_group)
                .execute(conn)?;

            // If tax_ids are provided, assign them to the tax group
            if let Some(tax_ids) = &self.tax_group.tax_ids {
                for tax_id in tax_ids {
                    let tax_group_tax = TaxGroupTax {
                        tax_group_id: new_tax_group.id,
                        tax_id: *tax_id,
                    };

                    diesel::insert_into(tax_group_taxes::table)
                        .values(&tax_group_tax)
                        .execute(conn)?;
                }
            }

            Ok(new_tax_group)
        })
    }
}

impl Command for UpdateTaxGroupCommand {
    type Output = TaxGroup;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Check if the tax group exists
            let _tax_group = tax_groups::table
                .filter(tax_groups::id.eq(self.tax_group.id))
                .select(TaxGroup::as_select())
                .get_result::<TaxGroup>(conn)?;

            // Create changeset
            let changeset = self
                .tax_group
                .clone()
                .into_changeset(Utc::now().naive_utc());

            // Update the tax group
            let updated_tax_group = diesel::update(tax_groups::table)
                .filter(tax_groups::id.eq(self.tax_group.id))
                .set(changeset)
                .returning(TaxGroup::as_returning())
                .get_result(conn)?;

            Ok(updated_tax_group)
        })
    }
}

impl Command for DeleteTaxGroupCommand {
    type Output = i32;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Check if the tax group is used in any sales order charges
            let is_used = diesel::dsl::select(diesel::dsl::exists(
                crate::schema::sales_order_charges::table
                    .filter(crate::schema::sales_order_charges::tax_group_id.eq(self.id)),
            ))
            .get_result::<bool>(conn)?;

            if is_used {
                return Err(Error::HasChildrenError);
            }

            // Delete all tax group tax associations
            let _deleted_associations = diesel::delete(tax_group_taxes::table)
                .filter(tax_group_taxes::tax_group_id.eq(self.id))
                .execute(conn)?;

            // Delete the tax group
            let deleted_count = diesel::delete(tax_groups::table)
                .filter(tax_groups::id.eq(self.id))
                .execute(conn)?;

            Ok(deleted_count as i32)
        })
    }
}

impl Command for AssignTaxToGroupCommand {
    type Output = i32;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Verify the tax group exists
            tax_groups::table
                .filter(tax_groups::id.eq(self.tax_group_id))
                .select(TaxGroup::as_select())
                .get_result::<TaxGroup>(conn)?;

            // Verify the tax exists
            taxes::table
                .filter(taxes::id.eq(self.tax_id))
                .select(Tax::as_select())
                .get_result::<Tax>(conn)?;

            // Check if the association already exists
            let exists = diesel::dsl::select(diesel::dsl::exists(
                tax_group_taxes::table
                    .filter(tax_group_taxes::tax_group_id.eq(self.tax_group_id))
                    .filter(tax_group_taxes::tax_id.eq(self.tax_id)),
            ))
            .get_result::<bool>(conn)?;

            if exists {
                return Ok(0); // Association already exists
            }

            // Create the association
            let tax_group_tax = TaxGroupTax {
                tax_group_id: self.tax_group_id,
                tax_id: self.tax_id,
            };

            let rows_affected = diesel::insert_into(tax_group_taxes::table)
                .values(&tax_group_tax)
                .execute(conn)?;

            Ok(rows_affected as i32)
        })
    }
}

impl Command for RemoveTaxFromGroupCommand {
    type Output = i32;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let result = diesel::delete(tax_group_taxes::table)
                .filter(tax_group_taxes::tax_group_id.eq(&self.tax_group_id))
                .filter(tax_group_taxes::tax_id.eq(&self.tax_id))
                .execute(conn)?;

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
