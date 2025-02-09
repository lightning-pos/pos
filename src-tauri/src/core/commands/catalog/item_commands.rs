use chrono::Utc;
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{
    core::{
        commands::{app_service::AppService, Command},
        models::{
            catalog::{
                item_group_model::ItemGroup,
                item_model::{Item, NewItem, UpdateItem},
            },
            common::tax_model::{ItemTax, Tax},
        },
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
    schema::{item_categories, item_taxes, items, taxes},
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
        service.conn.transaction(|conn| {
            // Verify category exists
            item_categories::table
                .filter(item_categories::id.eq(&self.item.category_id))
                .select(ItemGroup::as_select())
                .get_result::<ItemGroup>(conn)?;

            // Verify all taxes exist if tax_ids are provided
            if let Some(tax_ids) = &self.item.tax_ids {
                for tax_id in tax_ids {
                    taxes::table
                        .filter(taxes::id.eq(tax_id))
                        .select(Tax::as_select())
                        .get_result::<Tax>(conn)?;
                }
            }

            let now = Utc::now().naive_utc();
            let new_item = Item {
                id: Uuid::now_v7().into(),
                name: self.item.name.clone(),
                description: self.item.description.clone(),
                nature: self.item.nature,
                state: self.item.state,
                price: self.item.price,
                category_id: self.item.category_id.clone(),
                created_at: now,
                updated_at: now,
            };

            let res = diesel::insert_into(items::table)
                .values(&new_item)
                .returning(Item::as_returning())
                .get_result(conn)?;

            // Create item-tax associations if tax_ids are provided
            if let Some(tax_ids) = &self.item.tax_ids {
                for tax_id in tax_ids {
                    let item_tax = ItemTax {
                        item_id: res.id,
                        tax_id: *tax_id,
                    };
                    diesel::insert_into(item_taxes::table)
                        .values(&item_tax)
                        .execute(conn)?;
                }
            }

            Ok(res)
        })
    }
}

impl Command for UpdateItemCommand {
    type Output = Item;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Verify category exists
            if let Some(cat_id) = self.item.category_id.clone() {
                item_categories::table
                    .filter(item_categories::id.eq(&cat_id))
                    .select(ItemGroup::as_select())
                    .get_result::<ItemGroup>(conn)?;
            }

            // Verify item exists
            items::table
                .find(&self.item.id)
                .select(Item::as_select())
                .get_result::<Item>(conn)?;

            let now = Utc::now().naive_utc();

            let mut item = self.item.clone();
            item.updated_at = Some(now);

            let res = diesel::update(items::table.find(&self.item.id))
                .set(&item)
                .returning(Item::as_returning())
                .get_result(conn)?;

            Ok(res)
        })
    }
}

impl Command for DeleteItemCommand {
    type Output = i32;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let result = diesel::delete(items::table.find(&self.id)).execute(conn)?;

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
    use crate::core::models::item_model::{ItemNature, ItemState};

    fn create_test_tax(service: &mut AppService) -> Tax {
        let now = Utc::now().naive_utc();
        let tax = Tax {
            id: Uuid::now_v7().into(),
            name: "Test Tax".to_string(),
            rate: 1000,
            description: None,
            created_at: now,
            updated_at: now,
        };

        diesel::insert_into(taxes::table)
            .values(&tax)
            .execute(&mut service.conn)
            .unwrap();

        tax
    }

    #[test]
    fn test_create_item() {
        let mut service = AppService::new(":memory:");

        // Create a test category first
        let category_id = Uuid::now_v7().into();
        let now = Utc::now().naive_utc();
        let category = ItemGroup {
            id: category_id,
            name: "Test Category".to_string(),
            description: None,
            state: crate::core::models::catalog::item_group_model::ItemGroupState::Active,
            created_at: now,
            updated_at: now,
        };

        diesel::insert_into(item_categories::table)
            .values(&category)
            .execute(&mut service.conn)
            .unwrap();

        let command = CreateItemCommand {
            item: NewItem {
                name: "Test Item".to_string(),
                description: None,
                nature: ItemNature::Goods,
                state: ItemState::Active,
                price: 1000.into(),
                category_id,
                tax_ids: None,
            },
        };

        let item = command.exec(&mut service).unwrap();
        assert_eq!(item.name, "Test Item");
        assert_eq!(item.category_id, category_id);
    }

    #[test]
    fn test_create_item_with_taxes() {
        let mut service = AppService::new(":memory:");

        // Create test category
        let category_id = Uuid::now_v7().into();
        let now = Utc::now().naive_utc();
        let category = ItemGroup {
            id: category_id,
            name: "Test Category".to_string(),
            description: None,
            state: crate::core::models::catalog::item_group_model::ItemGroupState::Active,
            created_at: now,
            updated_at: now,
        };

        diesel::insert_into(item_categories::table)
            .values(&category)
            .execute(&mut service.conn)
            .unwrap();

        // Create test taxes
        let tax1 = create_test_tax(&mut service);
        let tax2 = create_test_tax(&mut service);

        let command = CreateItemCommand {
            item: NewItem {
                name: "Test Item".to_string(),
                description: None,
                nature: ItemNature::Goods,
                state: ItemState::Active,
                price: 1000.into(),
                category_id,
                tax_ids: Some(vec![tax1.id, tax2.id]),
            },
        };

        let item = command.exec(&mut service).unwrap();
        assert_eq!(item.name, "Test Item");

        // Verify tax associations were created
        let associations = item_taxes::table
            .filter(item_taxes::item_id.eq(item.id))
            .load::<ItemTax>(&mut service.conn)
            .unwrap();

        assert_eq!(associations.len(), 2);
        assert!(associations.iter().any(|a| a.tax_id == tax1.id));
        assert!(associations.iter().any(|a| a.tax_id == tax2.id));
    }

    #[test]
    fn test_create_item_with_nonexistent_tax() {
        let mut service = AppService::new(":memory:");

        // Create only category
        let category_id = Uuid::now_v7().into();
        let now = Utc::now().naive_utc();
        let category = ItemGroup {
            id: category_id,
            name: "Test Category".to_string(),
            description: None,
            state: crate::core::models::catalog::item_group_model::ItemGroupState::Active,
            created_at: now,
            updated_at: now,
        };

        diesel::insert_into(item_categories::table)
            .values(&category)
            .execute(&mut service.conn)
            .unwrap();

        let command = CreateItemCommand {
            item: NewItem {
                name: "Test Item".to_string(),
                description: None,
                nature: ItemNature::Goods,
                state: ItemState::Active,
                price: 1000.into(),
                category_id,
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
        let category_id = Uuid::now_v7().into();
        let now = Utc::now().naive_utc();
        let category = ItemGroup {
            id: category_id,
            name: "Test Category".to_string(),
            description: None,
            state: crate::core::models::catalog::item_group_model::ItemGroupState::Active,
            created_at: now,
            updated_at: now,
        };

        diesel::insert_into(item_categories::table)
            .values(&category)
            .execute(&mut service.conn)
            .unwrap();

        let command = CreateItemCommand {
            item: NewItem {
                name: "Test Item".to_string(),
                description: None,
                nature: ItemNature::Goods,
                state: ItemState::Active,
                price: 1000.into(),
                category_id,
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

        assert!(matches!(result, Err(Error::DieselError(_))));
    }

    #[test]
    fn test_delete_item() {
        let mut service = AppService::new(":memory:");

        // Create a test category first
        let category_id = Uuid::now_v7().into();
        let now = Utc::now().naive_utc();
        let category = ItemGroup {
            id: category_id,
            name: "Test Category".to_string(),
            description: None,
            state: crate::core::models::catalog::item_group_model::ItemGroupState::Active,
            created_at: now,
            updated_at: now,
        };

        diesel::insert_into(item_categories::table)
            .values(&category)
            .execute(&mut service.conn)
            .unwrap();

        let command = CreateItemCommand {
            item: NewItem {
                name: "Test Item".to_string(),
                description: None,
                nature: ItemNature::Goods,
                state: ItemState::Active,
                price: 1000.into(),
                category_id,
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
