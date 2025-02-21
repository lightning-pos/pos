use chrono::Utc;
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{
    core::{
        commands::{app_service::AppService, Command},
        models::{
            catalog::item_model::Item,
            common::tax_model::{ItemTax, ItemTaxNewInput, Tax, TaxNewInput, TaxUpdateInput},
        },
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
    schema::{item_taxes, items, taxes},
};

// Commands
pub struct CreateTaxCommand {
    pub tax: TaxNewInput,
}

pub struct UpdateTaxCommand {
    pub tax: TaxUpdateInput,
}

pub struct DeleteTaxCommand {
    pub id: DbUuid,
}

pub struct AssignTaxToItemCommand {
    pub item_tax: ItemTaxNewInput,
}

pub struct RemoveTaxFromItemCommand {
    pub item_id: DbUuid,
    pub tax_id: DbUuid,
}

// Command Implementations
impl Command for CreateTaxCommand {
    type Output = Tax;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Verify all items exist if item_ids are provided
            if let Some(item_ids) = &self.tax.item_ids {
                for item_id in item_ids {
                    items::table
                        .filter(items::id.eq(item_id))
                        .select(Item::as_select())
                        .get_result::<Item>(conn)?;
                }
            }

            let now = Utc::now().naive_utc();
            let new_tax = Tax {
                id: Uuid::now_v7().into(),
                name: self.tax.name.clone(),
                rate: self.tax.rate,
                description: self.tax.description.clone(),
                created_at: now,
                updated_at: now,
            };

            // Insert the tax
            diesel::insert_into(taxes::table)
                .values(&new_tax)
                .execute(conn)?;

            // Create item-tax associations if item_ids are provided
            if let Some(item_ids) = &self.tax.item_ids {
                for item_id in item_ids {
                    let item_tax = ItemTax {
                        item_id: *item_id,
                        tax_id: new_tax.id,
                    };
                    diesel::insert_into(item_taxes::table)
                        .values(&item_tax)
                        .execute(conn)?;
                }
            }

            Ok(new_tax)
        })
    }
}

impl Command for UpdateTaxCommand {
    type Output = Tax;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let now = Utc::now().naive_utc();
            let tax = taxes::table
                .filter(taxes::id.eq(&self.tax.id))
                .select(Tax::as_select())
                .get_result::<Tax>(conn)?;

            let updated_tax = Tax {
                id: tax.id,
                name: self.tax.name.clone().unwrap_or(tax.name),
                rate: self.tax.rate.unwrap_or(tax.rate),
                description: self.tax.description.clone().or(tax.description),
                created_at: tax.created_at,
                updated_at: now,
            };

            diesel::update(taxes::table)
                .filter(taxes::id.eq(&self.tax.id))
                .set((
                    taxes::name.eq(&updated_tax.name),
                    taxes::rate.eq(updated_tax.rate),
                    taxes::description.eq(&updated_tax.description),
                    taxes::updated_at.eq(updated_tax.updated_at),
                ))
                .execute(conn)?;

            Ok(updated_tax)
        })
    }
}

impl Command for DeleteTaxCommand {
    type Output = i32;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let result = diesel::delete(taxes::table)
                .filter(taxes::id.eq(&self.id))
                .execute(conn)?;

            if result == 0 {
                return Err(Error::NotFoundError);
            }

            Ok(result as i32)
        })
    }
}

impl Command for AssignTaxToItemCommand {
    type Output = i32;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Verify item exists
            items::table
                .filter(items::id.eq(&self.item_tax.item_id))
                .select(Item::as_select())
                .get_result::<Item>(conn)?;

            // Verify tax exists
            taxes::table
                .filter(taxes::id.eq(&self.item_tax.tax_id))
                .select(Tax::as_select())
                .get_result::<Tax>(conn)?;

            // Check if association already exists
            let existing = item_taxes::table
                .filter(item_taxes::item_id.eq(&self.item_tax.item_id))
                .filter(item_taxes::tax_id.eq(&self.item_tax.tax_id))
                .count()
                .get_result::<i64>(conn)?;

            if existing > 0 {
                return Err(Error::AlreadyExistsError);
            }

            let item_tax = ItemTax {
                item_id: self.item_tax.item_id,
                tax_id: self.item_tax.tax_id,
            };

            let rows_affected = diesel::insert_into(item_taxes::table)
                .values(&item_tax)
                .execute(conn)?;

            Ok(rows_affected as i32)
        })
    }
}

impl Command for RemoveTaxFromItemCommand {
    type Output = i32;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let result = diesel::delete(item_taxes::table)
                .filter(item_taxes::item_id.eq(&self.item_id))
                .filter(item_taxes::tax_id.eq(&self.tax_id))
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
    use crate::core::{
        commands::app_service::AppService,
        models::catalog::item_model::{ItemNature, ItemState},
        types::percentage::Percentage,
    };

    #[test]
    fn test_create_tax() {
        let mut service = AppService::new(":memory:");

        let command = CreateTaxCommand {
            tax: TaxNewInput {
                name: "GST".to_string(),
                rate: Percentage::from_float(18.0),
                description: Some("Goods and Services Tax".to_string()),
                item_ids: None,
            },
        };

        let tax = command.exec(&mut service).unwrap();
        assert_eq!(tax.name, "GST");
        assert_eq!(tax.rate, Percentage::from_float(18.0));
        assert_eq!(tax.description, Some("Goods and Services Tax".to_string()));
    }

    #[test]
    fn test_create_tax_with_items() {
        let mut service = AppService::new(":memory:");

        // Create test items
        let item1 = create_test_item(&mut service);
        let item2 = create_test_item(&mut service);

        let command = CreateTaxCommand {
            tax: TaxNewInput {
                name: "GST".to_string(),
                rate: Percentage::from_float(18.0),
                description: Some("Goods and Services Tax".to_string()),
                item_ids: Some(vec![item1.id, item2.id]),
            },
        };

        let tax = command.exec(&mut service).unwrap();

        // Verify tax was created
        assert_eq!(tax.name, "GST");
        assert_eq!(tax.rate, Percentage::from_float(18.0));

        // Verify item-tax associations were created
        let associations = item_taxes::table
            .filter(item_taxes::tax_id.eq(tax.id))
            .load::<ItemTax>(&mut service.conn)
            .unwrap();

        assert_eq!(associations.len(), 2);
        assert!(associations.iter().any(|a| a.item_id == item1.id));
        assert!(associations.iter().any(|a| a.item_id == item2.id));
    }

    #[test]
    fn test_create_tax_with_nonexistent_item() {
        let mut service = AppService::new(":memory:");

        let command = CreateTaxCommand {
            tax: TaxNewInput {
                name: "GST".to_string(),
                rate: Percentage::from_float(18.0),
                description: Some("Goods and Services Tax".to_string()),
                item_ids: Some(vec![Uuid::now_v7().into()]),
            },
        };

        let result = command.exec(&mut service);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_tax() {
        let mut service = AppService::new(":memory:");

        // First create a tax
        let create_command = CreateTaxCommand {
            tax: TaxNewInput {
                name: "GST".to_string(),
                rate: Percentage::from_float(18.0),
                description: Some("Goods and Services Tax".to_string()),
                item_ids: None,
            },
        };
        let created_tax = create_command.exec(&mut service).unwrap();

        // Then update it
        let update_command = UpdateTaxCommand {
            tax: TaxUpdateInput {
                id: created_tax.id,
                name: Some("Updated GST".to_string()),
                rate: Some(Percentage::from_float(20.0)),
                description: Some("Updated GST Description".to_string()),
            },
        };

        let updated_tax = update_command.exec(&mut service).unwrap();
        assert_eq!(updated_tax.name, "Updated GST");
        assert_eq!(updated_tax.rate, Percentage::from_float(20.0));
        assert_eq!(
            updated_tax.description,
            Some("Updated GST Description".to_string())
        );
    }

    #[test]
    fn test_update_tax_does_not_exist() {
        let mut service = AppService::new(":memory:");

        let command = UpdateTaxCommand {
            tax: TaxUpdateInput {
                id: Uuid::now_v7().into(),
                name: Some("Updated GST".to_string()),
                rate: Some(Percentage::from_float(20.0)),
                description: Some("Updated GST Description".to_string()),
            },
        };

        let result = command.exec(&mut service);
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_tax() {
        let mut service = AppService::new(":memory:");

        // First create a tax
        let create_command = CreateTaxCommand {
            tax: TaxNewInput {
                name: "GST".to_string(),
                rate: Percentage::from_float(18.0),
                description: Some("Goods and Services Tax".to_string()),
                item_ids: None,
            },
        };
        let created_tax = create_command.exec(&mut service).unwrap();

        // Then delete it
        let delete_command = DeleteTaxCommand { id: created_tax.id };

        let result = delete_command.exec(&mut service).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_delete_tax_does_not_exist() {
        let mut service = AppService::new(":memory:");

        let command = DeleteTaxCommand {
            id: Uuid::now_v7().into(),
        };

        let result = command.exec(&mut service);
        assert!(result.is_err());
    }

    fn create_test_item(service: &mut AppService) -> Item {
        let now = Utc::now().naive_utc();
        let item = Item {
            id: Uuid::now_v7().into(),
            category_id: Uuid::now_v7().into(),
            name: "Test Item".to_string(),
            description: None,
            nature: ItemNature::Goods,
            state: ItemState::Active,
            price: 1000.into(),
            created_at: now,
            updated_at: now,
        };

        diesel::insert_into(items::table)
            .values(&item)
            .execute(&mut service.conn)
            .unwrap();

        item
    }

    #[test]
    fn test_assign_tax_to_item() {
        let mut service = AppService::new(":memory:");

        // Create a tax and an item
        let tax = CreateTaxCommand {
            tax: TaxNewInput {
                name: "GST".to_string(),
                rate: Percentage::from_float(18.0),
                description: Some("Goods and Services Tax".to_string()),
                item_ids: None,
            },
        }
        .exec(&mut service)
        .unwrap();

        let item = create_test_item(&mut service);

        // Assign tax to item
        let command = AssignTaxToItemCommand {
            item_tax: ItemTaxNewInput {
                item_id: item.id,
                tax_id: tax.id,
            },
        };

        let result = command.exec(&mut service).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_assign_tax_to_nonexistent_item() {
        let mut service = AppService::new(":memory:");

        // Create only a tax
        let tax = CreateTaxCommand {
            tax: TaxNewInput {
                name: "GST".to_string(),
                rate: Percentage::from_float(18.0),
                description: Some("Goods and Services Tax".to_string()),
                item_ids: None,
            },
        }
        .exec(&mut service)
        .unwrap();

        // Try to assign tax to non-existent item
        let command = AssignTaxToItemCommand {
            item_tax: ItemTaxNewInput {
                item_id: Uuid::now_v7().into(),
                tax_id: tax.id,
            },
        };

        let result = command.exec(&mut service);
        assert!(result.is_err());
    }

    #[test]
    fn test_assign_nonexistent_tax_to_item() {
        let mut service = AppService::new(":memory:");

        // Create only an item
        let item = create_test_item(&mut service);

        // Try to assign non-existent tax to item
        let command = AssignTaxToItemCommand {
            item_tax: ItemTaxNewInput {
                item_id: item.id,
                tax_id: Uuid::now_v7().into(),
            },
        };

        let result = command.exec(&mut service);
        assert!(result.is_err());
    }

    #[test]
    fn test_assign_same_tax_twice() {
        let mut service = AppService::new(":memory:");

        // Create a tax and an item
        let tax = CreateTaxCommand {
            tax: TaxNewInput {
                name: "GST".to_string(),
                rate: Percentage::from_float(18.0),
                description: Some("Goods and Services Tax".to_string()),
                item_ids: None,
            },
        }
        .exec(&mut service)
        .unwrap();

        let item = create_test_item(&mut service);

        // Assign tax to item first time
        let command = AssignTaxToItemCommand {
            item_tax: ItemTaxNewInput {
                item_id: item.id,
                tax_id: tax.id,
            },
        };
        command.exec(&mut service).unwrap();

        // Try to assign same tax again
        let result = command.exec(&mut service);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_tax_from_item() {
        let mut service = AppService::new(":memory:");

        // Create a tax and an item
        let tax = CreateTaxCommand {
            tax: TaxNewInput {
                name: "GST".to_string(),
                rate: Percentage::from_float(18.0),
                description: Some("Goods and Services Tax".to_string()),
                item_ids: None,
            },
        }
        .exec(&mut service)
        .unwrap();

        let item = create_test_item(&mut service);

        // First assign tax to item
        AssignTaxToItemCommand {
            item_tax: ItemTaxNewInput {
                item_id: item.id,
                tax_id: tax.id,
            },
        }
        .exec(&mut service)
        .unwrap();

        // Then remove it
        let command = RemoveTaxFromItemCommand {
            item_id: item.id,
            tax_id: tax.id,
        };

        let result = command.exec(&mut service).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_remove_nonexistent_tax_assignment() {
        let mut service = AppService::new(":memory:");

        let command = RemoveTaxFromItemCommand {
            item_id: Uuid::now_v7().into(),
            tax_id: Uuid::now_v7().into(),
        };

        let result = command.exec(&mut service);
        assert!(result.is_err());
    }
}
