use chrono::Utc;
use sea_query::{Expr, Query, SqliteQueryBuilder};
use uuid::Uuid;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        commands::{app_service::AppService, Command},
        models::{
            catalog::item_model::{Item, Items},
            common::tax_model::{ItemTax, ItemTaxNewInput, ItemTaxes, Tax, TaxNewInput, TaxUpdateInput, Taxes},
        },
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
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
        service.db_adapter.transaction(|db| {
            // Verify all items exist if item_ids are provided
            if let Some(item_ids) = &self.tax.item_ids {
                for item_id in item_ids {
                    let item_query = Query::select()
                        .from(Items::Table)
                        .columns([Items::Id])
                        .and_where(Expr::col(Items::Id).eq(item_id.to_string()))
                        .to_string(SqliteQueryBuilder);

                    let item = db.query_optional::<Item>(&item_query, vec![])?;
                    if item.is_none() {
                        return Err(Error::NotFoundError);
                    }
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
            let tax_query = Query::insert()
                .into_table(Taxes::Table)
                .columns([
                    Taxes::Id,
                    Taxes::Name,
                    Taxes::Rate,
                    Taxes::Description,
                    Taxes::CreatedAt,
                    Taxes::UpdatedAt,
                ])
                .values_panic([
                    new_tax.id.to_string().into(),
                    new_tax.name.clone().into(),
                    new_tax.rate.to_string().into(),
                    match &new_tax.description {
                        Some(desc) => desc.clone().into(),
                        None => sea_query::Value::String(None).into(),
                    },
                    new_tax.created_at.to_string().into(),
                    new_tax.updated_at.to_string().into(),
                ])
                .to_string(SqliteQueryBuilder);

            db.execute(&tax_query, vec![])?;

            // Create item-tax associations if item_ids are provided
            if let Some(item_ids) = &self.tax.item_ids {
                for item_id in item_ids {
                    let item_tax = ItemTax {
                        item_id: *item_id,
                        tax_id: new_tax.id,
                    };

                    let item_tax_query = Query::insert()
                        .into_table(ItemTaxes::Table)
                        .columns([
                            ItemTaxes::ItemId,
                            ItemTaxes::TaxId,
                        ])
                        .values_panic([
                            item_tax.item_id.to_string().into(),
                            item_tax.tax_id.to_string().into(),
                        ])
                        .to_string(SqliteQueryBuilder);

                    db.execute(&item_tax_query, vec![])?;
                }
            }

            Ok(new_tax)
        })
    }
}

impl Command for UpdateTaxCommand {
    type Output = Tax;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.db_adapter.transaction(|db| {
            // Get the existing tax
            let query = Query::select()
                .from(Taxes::Table)
                .columns([
                    Taxes::Id,
                    Taxes::Name,
                    Taxes::Rate,
                    Taxes::Description,
                    Taxes::CreatedAt,
                    Taxes::UpdatedAt,
                ])
                .and_where(Expr::col(Taxes::Id).eq(self.tax.id.to_string()))
                .to_string(SqliteQueryBuilder);

            let tax = db.query_optional::<Tax>(&query, vec![])?;
            if tax.is_none() {
                return Err(Error::NotFoundError);
            }
            let tax = tax.unwrap();

            let now = Utc::now().naive_utc();

            // Build update query
            let mut update_query = Query::update();
            let update = update_query
                .table(Taxes::Table)
                .and_where(Expr::col(Taxes::Id).eq(self.tax.id.to_string()))
                .value(Taxes::UpdatedAt, now.to_string());

            if let Some(name) = &self.tax.name {
                update.value(Taxes::Name, name.clone());
            }

            if let Some(rate) = self.tax.rate {
                update.value(Taxes::Rate, rate.to_string());
            }

            if let Some(description) = &self.tax.description {
                update.value(Taxes::Description, description.clone());
            }

            let sql = update.to_string(SqliteQueryBuilder);
            db.execute(&sql, vec![])?;

            // Return the updated tax
            let updated_tax = Tax {
                id: tax.id,
                name: self.tax.name.clone().unwrap_or(tax.name),
                rate: self.tax.rate.unwrap_or(tax.rate),
                description: self.tax.description.clone().or(tax.description),
                created_at: tax.created_at,
                updated_at: now,
            };

            Ok(updated_tax)
        })
    }
}

impl Command for DeleteTaxCommand {
    type Output = i32;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.db_adapter.transaction(|db| {
            let query = Query::delete()
                .from_table(Taxes::Table)
                .and_where(Expr::col(Taxes::Id).eq(self.id.to_string()))
                .to_string(SqliteQueryBuilder);

            let result = db.execute(&query, vec![])?;

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
        service.db_adapter.transaction(|db| {
            // Verify item exists
            let item_query = Query::select()
                .from(Items::Table)
                .columns([Items::Id])
                .and_where(Expr::col(Items::Id).eq(self.item_tax.item_id.to_string()))
                .to_string(SqliteQueryBuilder);

            let item = db.query_optional::<Item>(&item_query, vec![])?;
            if item.is_none() {
                return Err(Error::NotFoundError);
            }

            // Verify tax exists
            let tax_query = Query::select()
                .from(Taxes::Table)
                .columns([Taxes::Id])
                .and_where(Expr::col(Taxes::Id).eq(self.item_tax.tax_id.to_string()))
                .to_string(SqliteQueryBuilder);

            let tax = db.query_optional::<Tax>(&tax_query, vec![])?;
            if tax.is_none() {
                return Err(Error::NotFoundError);
            }

            // Check if association already exists
            let exists_query = Query::select()
                .from(ItemTaxes::Table)
                .expr(Expr::count(Expr::col(ItemTaxes::ItemId)))
                .and_where(Expr::col(ItemTaxes::ItemId).eq(self.item_tax.item_id.to_string()))
                .and_where(Expr::col(ItemTaxes::TaxId).eq(self.item_tax.tax_id.to_string()))
                .to_string(SqliteQueryBuilder);

            let count: i64 = db.query_one(&exists_query, vec![])?;

            if count > 0 {
                return Err(Error::AlreadyExistsError);
            }

            let item_tax = ItemTax {
                item_id: self.item_tax.item_id,
                tax_id: self.item_tax.tax_id,
            };

            let insert_query = Query::insert()
                .into_table(ItemTaxes::Table)
                .columns([
                    ItemTaxes::ItemId,
                    ItemTaxes::TaxId,
                ])
                .values_panic([
                    item_tax.item_id.to_string().into(),
                    item_tax.tax_id.to_string().into(),
                ])
                .to_string(SqliteQueryBuilder);

            let rows_affected = db.execute(&insert_query, vec![])?;

            Ok(rows_affected as i32)
        })
    }
}

impl Command for RemoveTaxFromItemCommand {
    type Output = i32;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.db_adapter.transaction(|db| {
            let query = Query::delete()
                .from_table(ItemTaxes::Table)
                .and_where(Expr::col(ItemTaxes::ItemId).eq(self.item_id.to_string()))
                .and_where(Expr::col(ItemTaxes::TaxId).eq(self.tax_id.to_string()))
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
    use crate::core::{
        commands::app_service::AppService,
        commands::catalog::item_group_commands::CreateItemGroupCommand,
        models::catalog::item_model::{ItemNature, ItemState},
        models::catalog::item_group_model::ItemGroupNew,
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
        let query = Query::select()
            .from(ItemTaxes::Table)
            .columns([ItemTaxes::ItemId, ItemTaxes::TaxId])
            .and_where(Expr::col(ItemTaxes::TaxId).eq(tax.id.to_string()))
            .to_string(SqliteQueryBuilder);

        let associations = service.db_adapter.query_many::<ItemTax>(&query, vec![]).unwrap();

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

    fn create_test_item_category(service: &mut AppService) -> DbUuid {
        let category_name = format!("Test Category {}", Uuid::now_v7());
        let command = CreateItemGroupCommand {
            category: ItemGroupNew {
                name: category_name,
                description: None,
            },
        };
        let category = command.exec(service).unwrap();
        category.id
    }

    fn create_test_item(service: &mut AppService) -> Item {
        let now = Utc::now().naive_utc();
        let category_id = create_test_item_category(service);

        let item = Item {
            id: Uuid::now_v7().into(),
            category_id,
            name: "Test Item".to_string(),
            description: None,
            nature: ItemNature::Goods,
            state: ItemState::Active,
            price: 1000.into(),
            created_at: now,
            updated_at: now,
        };

        let query = Query::insert()
            .into_table(Items::Table)
            .columns([
                Items::Id,
                Items::CategoryId,
                Items::Name,
                Items::Description,
                Items::Nature,
                Items::State,
                Items::Price,
                Items::CreatedAt,
                Items::UpdatedAt,
            ])
            .values_panic([
                item.id.to_string().into(),
                item.category_id.to_string().into(),
                item.name.clone().into(),
                match &item.description {
                    Some(desc) => desc.clone().into(),
                    None => sea_query::Value::String(None).into(),
                },
                item.nature.to_string().into(),
                item.state.to_string().into(),
                item.price.to_string().into(),
                item.created_at.to_string().into(),
                item.updated_at.to_string().into(),
            ])
            .to_string(SqliteQueryBuilder);

        service.db_adapter.execute(&query, vec![]).unwrap();

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
