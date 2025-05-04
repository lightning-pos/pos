use chrono::Utc;
use sea_query::{Expr, Func, Query};
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

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Verify all items exist if item_ids are provided
        if let Some(item_ids) = &self.tax.item_ids {
            for item_id in item_ids {
                let mut query_builder = Query::select();
                let item_query = query_builder
                    .from(Items::Table)
                    .columns([Items::Id])
                    .and_where(Expr::col(Items::Id).eq(item_id.to_string()));

                let item = service.db_adapter.query_optional::<DbUuid>(&item_query).await?;
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
        let mut insert_query = Query::insert();
        let tax_stmt = insert_query
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
            ]);

        service.db_adapter.insert_many(&tax_stmt).await?;

        // Create item-tax associations if item_ids are provided
        if let Some(item_ids) = &self.tax.item_ids {
            for item_id in item_ids {
                let item_tax = ItemTax {
                    item_id: *item_id,
                    tax_id: new_tax.id,
                };

                let mut insert_query = Query::insert();
                let item_tax_stmt = insert_query
                    .into_table(ItemTaxes::Table)
                    .columns([
                        ItemTaxes::ItemId,
                        ItemTaxes::TaxId,
                    ])
                    .values_panic([
                        item_tax.item_id.to_string().into(),
                        item_tax.tax_id.to_string().into(),
                    ]);

                service.db_adapter.insert_many(&item_tax_stmt).await?;
            }
        }

        Ok(new_tax)
    }
}

impl Command for UpdateTaxCommand {
    type Output = Tax;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Get the existing tax
        let mut query_builder = Query::select();
        let select_stmt = query_builder
            .from(Taxes::Table)
            .columns([
                Taxes::Id,
                Taxes::Name,
                Taxes::Rate,
                Taxes::Description,
                Taxes::CreatedAt,
                Taxes::UpdatedAt,
            ])
            .and_where(Expr::col(Taxes::Id).eq(self.tax.id.to_string()));

        let tax = service.db_adapter.query_optional::<Tax>(&select_stmt).await?;
        if tax.is_none() {
            return Err(Error::NotFoundError);
        }
        let tax = tax.unwrap();

        let now = Utc::now().naive_utc();

        // Build update query
        let mut update_query = Query::update();
        let update_stmt = update_query
            .table(Taxes::Table)
            .and_where(Expr::col(Taxes::Id).eq(self.tax.id.to_string()))
            .value(Taxes::UpdatedAt, now.to_string());

        if let Some(name) = &self.tax.name {
            update_stmt.value(Taxes::Name, name.clone());
        }

        if let Some(rate) = self.tax.rate {
            update_stmt.value(Taxes::Rate, rate.to_string());
        }

        if let Some(description) = &self.tax.description {
            update_stmt.value(Taxes::Description, description.clone());
        }

        service.db_adapter.update_many(&update_stmt).await?;

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
    }
}

impl Command for DeleteTaxCommand {
    type Output = i32;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let mut delete_query = Query::delete();
        let delete_stmt = delete_query
            .from_table(Taxes::Table)
            .and_where(Expr::col(Taxes::Id).eq(self.id.to_string()));

        let result = service.db_adapter.delete(&delete_stmt).await?;

        if result == 0 {
            return Err(Error::NotFoundError);
        }

        Ok(result as i32)
    }
}

impl Command for AssignTaxToItemCommand {
    type Output = i32;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Verify item exists
        let mut item_query_builder = Query::select();
        let item_query = item_query_builder
            .from(Items::Table)
            .columns([Items::Id])
            .and_where(Expr::col(Items::Id).eq(self.item_tax.item_id.to_string()));

        let item = service.db_adapter.query_optional::<DbUuid>(&item_query).await?;
        if item.is_none() {
            return Err(Error::NotFoundError);
        }

        // Verify tax exists
        let mut tax_query_builder = Query::select();
        let tax_query = tax_query_builder
            .from(Taxes::Table)
            .columns([Taxes::Id])
            .and_where(Expr::col(Taxes::Id).eq(self.item_tax.tax_id.to_string()));

        let tax = service.db_adapter.query_optional::<DbUuid>(&tax_query).await?;
        if tax.is_none() {
            return Err(Error::NotFoundError);
        }

        // Check if association already exists
        let mut exists_query_builder = Query::select();
        let exists_query = exists_query_builder
            .expr(Func::count(Expr::col(ItemTaxes::ItemId)))
            .from(ItemTaxes::Table)
            .and_where(Expr::col(ItemTaxes::ItemId).eq(self.item_tax.item_id.to_string()))
            .and_where(Expr::col(ItemTaxes::TaxId).eq(self.item_tax.tax_id.to_string()));

        let count: i64 = service.db_adapter.query_one(&exists_query).await?;

        if count > 0 {
            return Err(Error::AlreadyExistsError);
        }

        let item_tax = ItemTax {
            item_id: self.item_tax.item_id,
            tax_id: self.item_tax.tax_id,
        };

        let mut insert_query_builder = Query::insert();
        let insert_stmt = insert_query_builder
            .into_table(ItemTaxes::Table)
            .columns([
                ItemTaxes::ItemId,
                ItemTaxes::TaxId,
            ])
            .values_panic([
                item_tax.item_id.to_string().into(),
                item_tax.tax_id.to_string().into(),
            ]);

        let rows_affected = service.db_adapter.insert_many(&insert_stmt).await?;

        Ok(rows_affected as i32)
    }
}

impl Command for RemoveTaxFromItemCommand {
    type Output = i32;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let mut delete_query = Query::delete();
        let delete_stmt = delete_query
            .from_table(ItemTaxes::Table)
            .and_where(Expr::col(ItemTaxes::ItemId).eq(self.item_id.to_string()))
            .and_where(Expr::col(ItemTaxes::TaxId).eq(self.tax_id.to_string()));

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
    use crate::core::{
        commands::{app_service::AppService, catalog::item_group_commands::CreateItemGroupCommand, tests::setup_service},
        models::catalog::{item_group_model::ItemCategoryNew, item_model::{ItemNature, ItemState}},
        types::percentage::Percentage,
    };

    #[tokio::test]
    async fn test_create_tax() {
        let mut service = setup_service().await;

        let command = CreateTaxCommand {
            tax: TaxNewInput {
                name: "GST".to_string(),
                rate: Percentage::from_float(18.0),
                description: Some("Goods and Services Tax".to_string()),
                item_ids: None,
            },
        };

        let tax = command.exec(&mut service).await.unwrap();
        assert_eq!(tax.name, "GST");
        assert_eq!(tax.rate, Percentage::from_float(18.0));
        assert_eq!(tax.description, Some("Goods and Services Tax".to_string()));
    }

    #[tokio::test]
    async fn test_create_tax_with_items() {
        let mut service = setup_service().await;

        // Create test items
        let item1 = create_test_item(&mut service).await;
        let item2 = create_test_item(&mut service).await;

        let command = CreateTaxCommand {
            tax: TaxNewInput {
                name: "GST".to_string(),
                rate: Percentage::from_float(18.0),
                description: Some("Goods and Services Tax".to_string()),
                item_ids: Some(vec![item1.id, item2.id]),
            },
        };

        let tax = command.exec(&mut service).await.unwrap();

        // Verify tax was created
        assert_eq!(tax.name, "GST");
        assert_eq!(tax.rate, Percentage::from_float(18.0));

        // Verify item-tax associations were created
        let mut query_builder = Query::select();
        let query = query_builder
            .from(ItemTaxes::Table)
            .columns([ItemTaxes::ItemId, ItemTaxes::TaxId])
            .and_where(Expr::col(ItemTaxes::TaxId).eq(tax.id.to_string()));

        let associations = service.db_adapter.query_many::<ItemTax>(&query).await.unwrap();

        assert_eq!(associations.len(), 2);
        assert!(associations.iter().any(|a| a.item_id == item1.id));
        assert!(associations.iter().any(|a| a.item_id == item2.id));
    }

    #[tokio::test]
    async fn test_create_tax_with_nonexistent_item() {
        let mut service = setup_service().await;

        let command = CreateTaxCommand {
            tax: TaxNewInput {
                name: "GST".to_string(),
                rate: Percentage::from_float(18.0),
                description: Some("Goods and Services Tax".to_string()),
                item_ids: Some(vec![Uuid::now_v7().into()]),
            },
        };

        let result = command.exec(&mut service).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_tax() {
        let mut service = setup_service().await;

        // First create a tax
        let create_command = CreateTaxCommand {
            tax: TaxNewInput {
                name: "GST".to_string(),
                rate: Percentage::from_float(18.0),
                description: Some("Goods and Services Tax".to_string()),
                item_ids: None,
            },
        };
        let created_tax = create_command.exec(&mut service).await.unwrap();

        // Then update it
        let update_command = UpdateTaxCommand {
            tax: TaxUpdateInput {
                id: created_tax.id,
                name: Some("Updated GST".to_string()),
                rate: Some(Percentage::from_float(20.0)),
                description: Some("Updated GST Description".to_string()),
            },
        };

        let updated_tax = update_command.exec(&mut service).await.unwrap();
        assert_eq!(updated_tax.name, "Updated GST");
        assert_eq!(updated_tax.rate, Percentage::from_float(20.0));
        assert_eq!(
            updated_tax.description,
            Some("Updated GST Description".to_string())
        );
    }

    #[tokio::test]
    async fn test_update_tax_does_not_exist() {
        let mut service = setup_service().await;

        let command = UpdateTaxCommand {
            tax: TaxUpdateInput {
                id: Uuid::now_v7().into(),
                name: Some("Updated GST".to_string()),
                rate: Some(Percentage::from_float(20.0)),
                description: Some("Updated GST Description".to_string()),
            },
        };

        let result = command.exec(&mut service).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_tax() {
        let mut service = setup_service().await;

        // First create a tax
        let create_command = CreateTaxCommand {
            tax: TaxNewInput {
                name: "GST".to_string(),
                rate: Percentage::from_float(18.0),
                description: Some("Goods and Services Tax".to_string()),
                item_ids: None,
            },
        };
        let created_tax = create_command.exec(&mut service).await.unwrap();

        // Then delete it
        let delete_command = DeleteTaxCommand { id: created_tax.id };

        let result = delete_command.exec(&mut service).await.unwrap();
        assert_eq!(result, 1);
    }

    #[tokio::test]
    async fn test_delete_tax_does_not_exist() {
        let mut service = setup_service().await;

        let command = DeleteTaxCommand {
            id: Uuid::now_v7().into(),
        };

        let result = command.exec(&mut service).await;
        assert!(result.is_err());
    }

    async fn create_test_item_category(service: &mut AppService) -> DbUuid {
        let category_name = format!("Test Category {}", Uuid::now_v7());
        let command = CreateItemGroupCommand {
            category: ItemCategoryNew {
                name: category_name,
                description: None,
            },
        };
        let category = command.exec(service).await.unwrap();
        category.id
    }

    async fn create_test_item(service: &mut AppService) -> Item {
        let now = Utc::now().naive_utc();
        let category_id = create_test_item_category(service).await;

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

        let mut insert_query = Query::insert();
        let insert_stmt = insert_query
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
            ]);

        service.db_adapter.insert_many(&insert_stmt).await.unwrap();

        item
    }

    #[tokio::test]
    async fn test_assign_tax_to_item() {
        let mut service = setup_service().await;

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
        .await
        .unwrap();

        let item = create_test_item(&mut service).await;

        // Assign tax to item
        let command = AssignTaxToItemCommand {
            item_tax: ItemTaxNewInput {
                item_id: item.id,
                tax_id: tax.id,
            },
        };

        let result = command.exec(&mut service).await.unwrap();
        assert_eq!(result, 1);
    }

    #[tokio::test]
    async fn test_assign_tax_to_nonexistent_item() {
        let mut service = setup_service().await;

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
        .await
        .unwrap();

        // Try to assign tax to non-existent item
        let command = AssignTaxToItemCommand {
            item_tax: ItemTaxNewInput {
                item_id: Uuid::now_v7().into(),
                tax_id: tax.id,
            },
        };

        let result = command.exec(&mut service).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_assign_nonexistent_tax_to_item() {
        let mut service = setup_service().await;

        // Create only an item
        let item = create_test_item(&mut service).await;

        // Try to assign non-existent tax to item
        let command = AssignTaxToItemCommand {
            item_tax: ItemTaxNewInput {
                item_id: item.id,
                tax_id: Uuid::now_v7().into(),
            },
        };

        let result = command.exec(&mut service).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_assign_same_tax_twice() {
        let mut service = setup_service().await;

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
        .await
        .unwrap();

        let item = create_test_item(&mut service).await;

        // Assign tax to item first time
        let command = AssignTaxToItemCommand {
            item_tax: ItemTaxNewInput {
                item_id: item.id,
                tax_id: tax.id,
            },
        };
        command.exec(&mut service).await.unwrap();

        // Try to assign same tax again
        let result = command.exec(&mut service).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_tax_from_item() {
        let mut service = setup_service().await;

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
        .await
        .unwrap();

        let item = create_test_item(&mut service).await;

        // First assign tax to item
        AssignTaxToItemCommand {
            item_tax: ItemTaxNewInput {
                item_id: item.id,
                tax_id: tax.id,
            },
        }
        .exec(&mut service)
        .await
        .unwrap();

        // Then remove it
        let command = RemoveTaxFromItemCommand {
            item_id: item.id,
            tax_id: tax.id,
        };

        let result = command.exec(&mut service).await.unwrap();
        assert_eq!(result, 1);
    }

    #[tokio::test]
    async fn test_remove_nonexistent_tax_assignment() {
        let mut service = setup_service().await;

        let command = RemoveTaxFromItemCommand {
            item_id: Uuid::now_v7().into(),
            tax_id: Uuid::now_v7().into(),
        };

        let result = command.exec(&mut service).await;
        assert!(result.is_err());
    }
}
