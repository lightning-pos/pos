use chrono::Utc;
use sea_query::{Expr, Query, SqliteQueryBuilder};
use uuid::Uuid;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        commands::{app_service::AppService, Command},
        models::{
            catalog::{
                item_group_model::{ItemCategories, ItemGroup, ItemGroupState},
                item_model::{Item, ItemNature, ItemState, Items, NewItem, UpdateItem},
            },
            common::tax_model::{ItemTax, ItemTaxes, Tax, Taxes},
        },
        types::{db_uuid::DbUuid, money::Money, percentage::Percentage},
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

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Verify category exists
        let category_query = Query::select()
            .from(ItemCategories::Table)
            .column(ItemCategories::Id)
            .and_where(Expr::col(ItemCategories::Id).eq(self.item.category_id.to_string()))
            .to_string(SqliteQueryBuilder);

        let category = service.db_adapter.query_optional::<ItemGroup>(&category_query, vec![])?;
        if category.is_none() {
            return Err(Error::NotFoundError);
        }

        // Verify all taxes exist if tax_ids are provided
        if let Some(tax_ids) = &self.item.tax_ids {
            for tax_id in tax_ids {
                let tax_query = Query::select()
                    .from(Taxes::Table)
                    .column(Taxes::Id)
                    .and_where(Expr::col(Taxes::Id).eq(tax_id.to_string()))
                    .to_string(SqliteQueryBuilder);

                let tax = service.db_adapter.query_optional::<Tax>(&tax_query, vec![])?;
                if tax.is_none() {
                    return Err(Error::NotFoundError);
                }
            }
        }

        let now = Utc::now().naive_utc();
        let item_id: DbUuid = Uuid::now_v7().into();

        // Build the insert query
        let insert_query = Query::insert()
            .into_table(Items::Table)
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
            .values_panic([
                item_id.to_string().into(),
                self.item.name.clone().into(),
                self.item.description.clone().map_or_else(|| "NULL".into(), |d| d.into()),
                self.item.nature.to_string().into(),
                self.item.state.to_string().into(),
                self.item.price.to_string().into(),
                self.item.category_id.to_string().into(),
                now.to_string().into(),
                now.to_string().into(),
            ])
            .to_string(SqliteQueryBuilder);

        // Execute the insert query
        service.db_adapter.execute(&insert_query, vec![])?;

        // Create item-tax associations if tax_ids are provided
        if let Some(tax_ids) = &self.item.tax_ids {
            for tax_id in tax_ids {
                let item_tax_query = Query::insert()
                    .into_table(ItemTaxes::Table)
                    .columns([
                        ItemTaxes::ItemId,
                        ItemTaxes::TaxId,
                    ])
                    .values_panic([
                        item_id.to_string().into(),
                        tax_id.to_string().into(),
                    ])
                    .to_string(SqliteQueryBuilder);

                service.db_adapter.execute(&item_tax_query, vec![])?;
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

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Verify category exists if provided
        if let Some(cat_id) = self.item.category_id {
            let category_query = Query::select()
                .from(ItemCategories::Table)
                .column(ItemCategories::Id)
                .and_where(Expr::col(ItemCategories::Id).eq(cat_id.to_string()))
                .to_string(SqliteQueryBuilder);

            let category = service.db_adapter.query_optional::<ItemGroup>(&category_query, vec![])?;
            if category.is_none() {
                return Err(Error::NotFoundError);
            }
        }

        // Verify item exists
        let item_query = Query::select()
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
            .and_where(Expr::col(Items::Id).eq(self.item.id.to_string()))
            .to_string(SqliteQueryBuilder);

        let existing_item = service.db_adapter.query_optional::<Item>(&item_query, vec![])?;
        if existing_item.is_none() {
            return Err(Error::NotFoundError);
        }

        let _existing_item = existing_item.unwrap();
        let now = Utc::now().naive_utc();

        // Build update query with SeaQuery
        let mut query = Query::update();
        query.table(Items::Table)
            .value(Items::UpdatedAt, now.to_string());

        // Add optional fields if they exist
        if let Some(name) = &self.item.name {
            query.value(Items::Name, name.clone());
        }

        if let Some(description) = &self.item.description {
            match description {
                Some(desc) => query.value(Items::Description, desc.clone()),
                None => query.value(Items::Description, "NULL"),
            };
        }

        if let Some(nature) = &self.item.nature {
            query.value(Items::Nature, nature.to_string());
        }

        if let Some(state) = &self.item.state {
            query.value(Items::State, state.to_string());
        }

        if let Some(price) = &self.item.price {
            query.value(Items::Price, price.to_string());
        }

        if let Some(category_id) = &self.item.category_id {
            query.value(Items::CategoryId, category_id.to_string());
        }

        // Add WHERE condition
        query.and_where(Expr::col(Items::Id).eq(self.item.id.to_string()));

        // Generate the SQL query
        let sql = query.to_string(SqliteQueryBuilder);

        // Execute the update
        service.db_adapter.execute(&sql, vec![])?;

        // Retrieve the updated item
        let select_query = Query::select()
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
            .and_where(Expr::col(Items::Id).eq(self.item.id.to_string()))
            .to_string(SqliteQueryBuilder);

        let updated_item = service.db_adapter.query_one::<Item>(&select_query, vec![])?;

        Ok(updated_item)
    }
}

impl Command for DeleteItemCommand {
    type Output = i32;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Build delete query with SeaQuery
        let delete_query = Query::delete()
            .from_table(Items::Table)
            .and_where(Expr::col(Items::Id).eq(self.id.to_string()))
            .to_string(SqliteQueryBuilder);

        // Execute the delete query
        let affected_rows = service.db_adapter.execute(&delete_query, vec![])?;

        if affected_rows == 0 {
            Err(Error::NotFoundError)
        } else {
            Ok(affected_rows as i32)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a test category
    fn create_test_category(service: &mut AppService) -> ItemGroup {
        let category_id: DbUuid = Uuid::now_v7().into();
        let now = Utc::now().naive_utc();

        let insert_query = Query::insert()
            .into_table(ItemCategories::Table)
            .columns([
                ItemCategories::Id,
                ItemCategories::Name,
                ItemCategories::Description,
                ItemCategories::State,
                ItemCategories::CreatedAt,
                ItemCategories::UpdatedAt,
            ])
            .values_panic([
                category_id.to_string().into(),
                "Test Category".into(),
                "NULL".into(),
                ItemGroupState::Active.to_string().into(),
                now.to_string().into(),
                now.to_string().into(),
            ])
            .to_string(SqliteQueryBuilder);

        service.db_adapter.execute(&insert_query, vec![]).unwrap();

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
    fn create_test_tax(service: &mut AppService) -> Tax {
        let tax_id: DbUuid = Uuid::now_v7().into();
        let now = Utc::now().naive_utc();
        let rate = Percentage::from_float(10.0);

        let insert_query = Query::insert()
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
                tax_id.to_string().into(),
                "Test Tax".into(),
                rate.to_string().into(),
                "NULL".into(),
                now.to_string().into(),
                now.to_string().into(),
            ])
            .to_string(SqliteQueryBuilder);

        service.db_adapter.execute(&insert_query, vec![]).unwrap();

        Tax {
            id: tax_id,
            name: "Test Tax".to_string(),
            rate,
            description: None,
            created_at: now,
            updated_at: now,
        }
    }

    #[test]
    fn test_create_item() {
        let mut service = AppService::new(":memory:");

        // Create a test category first
        let category = create_test_category(&mut service);

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

        let item = command.exec(&mut service).unwrap();
        assert_eq!(item.name, "Test Item");
        assert_eq!(item.category_id, category.id);
    }

    #[test]
    fn test_create_item_with_taxes() {
        let mut service = AppService::new(":memory:");

        // Create test category
        let category = create_test_category(&mut service);

        // Create test taxes
        let tax1 = create_test_tax(&mut service);
        let tax2 = create_test_tax(&mut service);

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

        let item = command.exec(&mut service).unwrap();
        assert_eq!(item.name, "Test Item");

        // Verify tax associations were created
        let select_query = Query::select()
            .from(ItemTaxes::Table)
            .columns([
                ItemTaxes::ItemId,
                ItemTaxes::TaxId,
            ])
            .and_where(Expr::col(ItemTaxes::ItemId).eq(item.id.to_string()))
            .to_string(SqliteQueryBuilder);

        let associations = service.db_adapter.query_many::<ItemTax>(&select_query, vec![]).unwrap();

        assert_eq!(associations.len(), 2);
        assert!(associations.iter().any(|a| a.tax_id == tax1.id));
        assert!(associations.iter().any(|a| a.tax_id == tax2.id));
    }

    #[test]
    fn test_create_item_with_nonexistent_tax() {
        let mut service = AppService::new(":memory:");

        // Create only category
        let category = create_test_category(&mut service);

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

        let result = command.exec(&mut service);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_item() {
        let mut service = AppService::new(":memory:");

        // Create a test category first
        let category = create_test_category(&mut service);

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

        let item = command.exec(&mut service).unwrap();

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
        let result = update_command.exec(&mut service);
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_item_does_not_exist() {
        let mut service = AppService::new(":memory:");

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
        let result = command.exec(&mut service);

        assert!(result.is_err());
    }

    #[test]
    fn test_delete_item() {
        let mut service = AppService::new(":memory:");

        // Create a test category first
        let category = create_test_category(&mut service);

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

        let item = command.exec(&mut service).unwrap();

        let delete_command = DeleteItemCommand { id: item.id };
        let result = delete_command.exec(&mut service);
        assert!(result.is_ok());
    }

    #[test]
    fn test_delete_item_does_not_exist() {
        let mut service = AppService::new(":memory:");

        let command = DeleteItemCommand {
            id: Uuid::now_v7().into(),
        };
        let result = command.exec(&mut service);
        assert!(matches!(result, Err(Error::NotFoundError)));
    }
}
