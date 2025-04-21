use chrono::Utc;
use sea_query::{Expr, Query};
use uuid::Uuid;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        commands::{app_service::AppService, Command},
        models::{
            catalog::{
                item_group_model::{ItemCategories, ItemGroup},
                item_model::{Item, Items, NewItem, UpdateItem},
            },
            common::tax_model::{Tax, Taxes},
        }, types::db_uuid::DbUuid,
    },
    error::{Error, Result},
};

// Commands
pub struct CreateItemCommand {
    pub item: NewItem,
}

pub struct UpdateItemCommand {
    pub item: UpdateItem,
}

pub struct DeleteItemCommand {
    pub id: DbUuid,
}

// Command Implementations
impl Command for CreateItemCommand {
    type Output = Item;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Verify category exists
        let mut select_query = Query::select();
        let category_stmt = select_query
            .from(ItemCategories::Table)
            .column(ItemCategories::Id)
            .and_where(Expr::col(ItemCategories::Id).eq(self.item.category_id.to_string()));

        let category = service.db_adapter.query_optional::<ItemGroup>(&category_stmt).await?;
        if category.is_none() {
            return Err(Error::NotFoundError);
        }

        // Verify all taxes exist if tax_ids are provided
        if let Some(tax_ids) = &self.item.tax_ids {
            for tax_id in tax_ids {
                let mut tax_select_query = Query::select();
                let tax_stmt = tax_select_query
                    .from(Taxes::Table)
                    .column(Taxes::Id)
                    .and_where(Expr::col(Taxes::Id).eq(tax_id.to_string()));

                let tax = service.db_adapter.query_optional::<Tax>(&tax_stmt).await?;
                if tax.is_none() {
                    return Err(Error::NotFoundError);
                }
            }
        }

        let now = Utc::now().naive_utc();
        let item_id: DbUuid = Uuid::now_v7().into();

        // Build the insert query
        let insert_sql = format!(
            "INSERT INTO items (id, name, description, nature, state, price, category_id, created_at, updated_at) \
             VALUES ('{}', '{}', {}, '{}', '{}', '{}', '{}', '{}', '{}')",
            item_id.to_string(),
            self.item.name.clone(),
            match &self.item.description {
                Some(desc) => format!("'{}'", desc),
                None => "NULL".to_string(),
            },
            self.item.nature.to_string(),
            self.item.state.to_string(),
            self.item.price.to_string(),
            self.item.category_id.to_string(),
            now.to_string(),
            now.to_string()
        );

        // Execute the insert query
        service.db_adapter.execute(&insert_sql).await?;

        // Create item-tax associations if tax_ids are provided
        if let Some(tax_ids) = &self.item.tax_ids {
            for tax_id in tax_ids {
                let item_tax_sql = format!(
                    "INSERT INTO item_taxes (item_id, tax_id) VALUES ('{}', '{}')",
                    item_id.to_string(),
                    tax_id.to_string()
                );

                service.db_adapter.execute(&item_tax_sql).await?;
            }
        }

        // Create and return the new item object
        let new_item = Item {
            id: item_id,
            name: self.item.name.clone(),
            description: self.item.description.clone(),
            nature: self.item.nature,
            state: self.item.state,
            price: self.item.price,
            category_id: self.item.category_id.clone(),
            created_at: now,
            updated_at: now,
        };

        Ok(new_item)
    }
}

impl Command for UpdateItemCommand {
    type Output = Item;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Verify category exists if provided
        if let Some(cat_id) = self.item.category_id {
            let mut select_query = Query::select();
            let category_stmt = select_query
                .from(ItemCategories::Table)
                .column(ItemCategories::Id)
                .and_where(Expr::col(ItemCategories::Id).eq(cat_id.to_string()));

            let category = service.db_adapter.query_optional::<ItemGroup>(category_stmt).await?;
            if category.is_none() {
                return Err(Error::NotFoundError);
            }
        }

        // Verify item exists
        let mut item_select_query = Query::select();
        let item_stmt = item_select_query
            .from(Items::Table)
            .columns([
                Items::Id,
                Items::Name,
                Items::Description,
                Items::Nature,
                Items::State,
                Items::Price,
                Items::CategoryId,
                Items::CreatedAt,
                Items::UpdatedAt,
            ])
            .and_where(Expr::col(Items::Id).eq(self.item.id.to_string()));

        let existing_item = service.db_adapter.query_optional::<Item>(&item_stmt).await?;
        if existing_item.is_none() {
            return Err(Error::NotFoundError);
        }

        let _existing_item = existing_item.unwrap();
        let now = Utc::now().naive_utc();

        // Build update query with SeaQuery
        let mut update_query = Query::update();
        let update_stmt = update_query.table(Items::Table)
            .value(Items::UpdatedAt, now.to_string());

        // Add optional fields if they exist
        if let Some(name) = &self.item.name {
            update_stmt.value(Items::Name, name.clone());
        }

        if let Some(description) = &self.item.description {
            match description {
                Some(desc) => update_stmt.value(Items::Description, desc.clone()),
                None => update_stmt.value(Items::Description, sea_query::Value::String(None)),
            };
        }

        if let Some(nature) = &self.item.nature {
            update_stmt.value(Items::Nature, nature.to_string());
        }

        if let Some(state) = &self.item.state {
            update_stmt.value(Items::State, state.to_string());
        }

        if let Some(price) = &self.item.price {
            update_stmt.value(Items::Price, price.to_string());
        }

        if let Some(category_id) = &self.item.category_id {
            update_stmt.value(Items::CategoryId, category_id.to_string());
        }

        // Add WHERE condition
        update_stmt.and_where(Expr::col(Items::Id).eq(self.item.id.to_string()));

        // Execute the update
        service.db_adapter.update_many(&update_stmt).await?;

        // Retrieve the updated item
        let mut select_query = Query::select();
        let select_stmt = select_query
            .from(Items::Table)
            .columns([
                Items::Id,
                Items::Name,
                Items::Description,
                Items::Nature,
                Items::State,
                Items::Price,
                Items::CategoryId,
                Items::CreatedAt,
                Items::UpdatedAt,
            ])
            .and_where(Expr::col(Items::Id).eq(self.item.id.to_string()));

        let updated_item = service.db_adapter.query_one::<Item>(&select_stmt).await?;

        Ok(updated_item)
    }
}

impl Command for DeleteItemCommand {
    type Output = i32;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let mut delete_query = Query::delete();
        let delete_stmt = delete_query
            .from_table(Items::Table)
            .and_where(Expr::col(Items::Id).eq(self.id.to_string()));

        let affected_rows = service.db_adapter.delete(&delete_stmt).await?;

        if affected_rows == 0 {
            Err(Error::NotFoundError)
        } else {
            Ok(affected_rows as i32)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{commands::tests::setup_service, models::{catalog::{item_group_model::ItemGroupState, item_model::{ItemNature, ItemState}}, common::tax_model::{ItemTax, ItemTaxes}}, types::{money::Money, percentage::Percentage}};

    use super::*;

    // Helper function to create a test category
    async fn create_test_category(service: &mut AppService) -> ItemGroup {
        let category_id: DbUuid = Uuid::now_v7().into();
        let now = Utc::now().naive_utc();

        let insert_sql = format!(
            "INSERT INTO item_categories (id, name, description, state, created_at, updated_at) \
             VALUES ('{}', '{}', NULL, '{}', '{}', '{}')",
            category_id.to_string(),
            "Test Category",
            ItemGroupState::Active.to_string(),
            now.to_string(),
            now.to_string()
        );

        service.db_adapter.execute(&insert_sql).await.unwrap();

        ItemGroup {
            id: category_id,
            name: "Test Category".to_string(),
            description: None,
            state: ItemGroupState::Active,
            created_at: now,
            updated_at: now,
        }
    }

    // Helper function to create a test tax
    async fn create_test_tax(service: &mut AppService) -> Tax {
        let tax_id: DbUuid = Uuid::now_v7().into();
        let now = Utc::now().naive_utc();
        let rate = Percentage::from_float(10.0);

        let insert_sql = format!(
            "INSERT INTO taxes (id, name, rate, description, created_at, updated_at) \
             VALUES ('{}', '{}', '{}', NULL, '{}', '{}')",
            tax_id.to_string(),
            "Test Tax",
            rate.to_string(),
            now.to_string(),
            now.to_string()
        );

        service.db_adapter.execute(&insert_sql).await.unwrap();

        Tax {
            id: tax_id,
            name: "Test Tax".to_string(),
            rate,
            description: None,
            created_at: now,
            updated_at: now,
        }
    }

    #[tokio::test]
    async fn test_create_item() {
        let mut service = setup_service();

        // Create a test category first
        let category = create_test_category(&mut service).await;

        let command = CreateItemCommand {
            item: NewItem {
                name: "Test Item".to_string(),
                description: None,
                nature: ItemNature::Goods,
                state: ItemState::Active,
                price: Money::from(1000),
                category_id: category.id,
                tax_ids: None,
            },
        };

        let item = command.exec(&mut service).await.unwrap();
        assert_eq!(item.name, "Test Item");
        assert_eq!(item.category_id, category.id);
    }

    #[tokio::test]
    async fn test_create_item_with_taxes() {
        let mut service = setup_service();

        // Create test category
        let category = create_test_category(&mut service).await;

        // Create test taxes
        let tax1 = create_test_tax(&mut service).await;
        let tax2 = create_test_tax(&mut service).await;

        let command = CreateItemCommand {
            item: NewItem {
                name: "Test Item".to_string(),
                description: None,
                nature: ItemNature::Goods,
                state: ItemState::Active,
                price: Money::from(1000),
                category_id: category.id,
                tax_ids: Some(vec![tax1.id, tax2.id]),
            },
        };

        let item = command.exec(&mut service).await.unwrap();
        assert_eq!(item.name, "Test Item");

        // Verify tax associations were created
        let mut select_query = Query::select();
        let select_stmt = select_query
            .from(ItemTaxes::Table)
            .columns([
                ItemTaxes::ItemId,
                ItemTaxes::TaxId,
            ])
            .and_where(Expr::col(ItemTaxes::ItemId).eq(item.id.to_string()));

        let associations = service.db_adapter.query_many::<ItemTax>(&select_stmt).await.unwrap();

        assert_eq!(associations.len(), 2);
        assert!(associations.iter().any(|a| a.tax_id == tax1.id));
        assert!(associations.iter().any(|a| a.tax_id == tax2.id));
    }

    #[tokio::test]
    async fn test_create_item_with_nonexistent_tax() {
        let mut service = setup_service();

        // Create only category
        let category = create_test_category(&mut service).await;

        let command = CreateItemCommand {
            item: NewItem {
                name: "Test Item".to_string(),
                description: None,
                nature: ItemNature::Goods,
                state: ItemState::Active,
                price: Money::from(1000),
                category_id: category.id,
                tax_ids: Some(vec![Uuid::now_v7().into()]),
            },
        };

        let result = command.exec(&mut service).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_item() {
        let mut service = setup_service();

        // Create a test category first
        let category = create_test_category(&mut service).await;

        let command = CreateItemCommand {
            item: NewItem {
                name: "Test Item".to_string(),
                description: None,
                nature: ItemNature::Goods,
                state: ItemState::Active,
                price: Money::from(1000),
                category_id: category.id,
                tax_ids: None,
            },
        };

        let item = command.exec(&mut service).await.unwrap();

        let updated_item = UpdateItem {
            id: item.id,
            name: Some("Test Item 2".to_string()),
            description: None,
            nature: None,
            state: None,
            price: None,
            category_id: None,
            updated_at: None,
        };

        let update_command = UpdateItemCommand { item: updated_item };
        let result = update_command.exec(&mut service).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_item_does_not_exist() {
        let mut service = setup_service();

        let now = Utc::now().naive_utc();
        let item = UpdateItem {
            id: Uuid::now_v7().into(),
            name: Some("Test Item".to_string()),
            description: None,
            nature: None,
            state: None,
            price: None,
            category_id: None,
            updated_at: Some(now),
        };

        let command = UpdateItemCommand { item };
        let result = command.exec(&mut service).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_item() {
        let mut service = setup_service();

        // Create a test category first
        let category = create_test_category(&mut service).await;

        let command = CreateItemCommand {
            item: NewItem {
                name: "Test Item".to_string(),
                description: None,
                nature: ItemNature::Goods,
                state: ItemState::Active,
                price: Money::from(1000),
                category_id: category.id,
                tax_ids: None,
            },
        };

        let item = command.exec(&mut service).await.unwrap();

        let delete_command = DeleteItemCommand { id: item.id };
        let result = delete_command.exec(&mut service).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_item_does_not_exist() {
        let mut service = setup_service();

        let command = DeleteItemCommand {
            id: Uuid::now_v7().into(),
        };
        let result = command.exec(&mut service).await;
        assert!(matches!(result, Err(Error::NotFoundError)));
    }
}
