use sea_query::{Expr, Query};

use crate::adapters::outgoing::database::DatabaseAdapter;
use crate::core::commands::{app_service::AppService, Command};
use crate::core::models::catalog::item_discount_model::{ItemDiscount, ItemDiscountNewInput, ItemDiscounts};
use crate::core::types::db_uuid::DbUuid;
use crate::error::Result;

// --- Command Structs ---

pub struct AddItemDiscountCommand {
    pub item_discount: ItemDiscountNewInput,
}

pub struct RemoveItemDiscountCommand {
    pub item_id: DbUuid,
    pub discount_id: DbUuid,
}

pub struct GetItemDiscountsCommand {
    pub item_id: DbUuid,
}

pub struct GetDiscountItemsCommand {
    pub discount_id: DbUuid,
}

// --- Command Implementations ---

impl Command for AddItemDiscountCommand {
    type Output = ItemDiscount;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Check if the relationship already exists
        let mut select_query = Query::select();
        let select_query = select_query
            .from(ItemDiscounts::Table)
            .columns([
                ItemDiscounts::ItemId,
                ItemDiscounts::DiscountId,
            ])
            .and_where(Expr::col(ItemDiscounts::ItemId).eq(self.item_discount.item_id.to_string()))
            .and_where(Expr::col(ItemDiscounts::DiscountId).eq(self.item_discount.discount_id.to_string()));

        let existing = service.db_adapter.query_optional::<ItemDiscount>(&select_query).await?;

        if let Some(existing_relation) = existing {
            return Ok(existing_relation);
        }

        // Create new relationship
        let new_item_discount = ItemDiscount {
            item_id: self.item_discount.item_id,
            discount_id: self.item_discount.discount_id,
        };

        let mut insert_query = Query::insert();
        let insert_stmt = insert_query
            .into_table(ItemDiscounts::Table)
            .columns([
                ItemDiscounts::ItemId,
                ItemDiscounts::DiscountId,
            ])
            .values_panic([
                self.item_discount.item_id.to_string().into(),
                self.item_discount.discount_id.to_string().into(),
            ]);

        service.db_adapter.insert_many(&insert_stmt).await?;

        Ok(new_item_discount)
    }
}

impl Command for RemoveItemDiscountCommand {
    type Output = usize;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let mut delete_query = Query::delete();
        let delete_query = delete_query
            .from_table(ItemDiscounts::Table)
            .and_where(Expr::col(ItemDiscounts::ItemId).eq(self.item_id.to_string()))
            .and_where(Expr::col(ItemDiscounts::DiscountId).eq(self.discount_id.to_string()));

        let affected_rows = service.db_adapter.delete(&delete_query).await?;

        Ok(affected_rows as usize)
    }
}

impl Command for GetItemDiscountsCommand {
    type Output = Vec<ItemDiscount>;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let mut select_query = Query::select();
        let select_query = select_query
            .from(ItemDiscounts::Table)
            .columns([
                ItemDiscounts::ItemId,
                ItemDiscounts::DiscountId,
            ])
            .and_where(Expr::col(ItemDiscounts::ItemId).eq(self.item_id.to_string()));

        let result = service.db_adapter.query_many::<ItemDiscount>(&select_query).await?;

        Ok(result)
    }
}

impl Command for GetDiscountItemsCommand {
    type Output = Vec<ItemDiscount>;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let mut select_query = Query::select();
        let select_query = select_query
            .from(ItemDiscounts::Table)
            .columns([
                ItemDiscounts::ItemId,
                ItemDiscounts::DiscountId,
            ])
            .and_where(Expr::col(ItemDiscounts::DiscountId).eq(self.discount_id.to_string()));

        let result = service.db_adapter.query_many::<ItemDiscount>(&select_query).await?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::commands::app_service::tests::setup_service;
    use crate::core::commands::catalog::discount_commands::CreateDiscountCommand;
    use crate::core::commands::catalog::item_group_commands::CreateItemGroupCommand;
    use crate::core::models::catalog::{
        discount_model::{Discount, DiscountNewInput, DiscountScope, DiscountState, DiscountType},
        item_model::{Item, ItemNature, ItemState},
        item_group_model::ItemCategoryNew,
    };
    use crate::core::types::money::Money;
    use chrono::Utc;
    use uuid::Uuid;
    use tokio;

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
        let item_id = Uuid::now_v7().into();

        let item = Item {
            id: item_id,
            category_id,
            name: "Test Item".to_string(),
            description: None,
            nature: ItemNature::Goods,
            state: ItemState::Active,
            price: Money::from_float(100.0),
            created_at: now,
            updated_at: now,
        };

        // Use SeaQuery to insert the item
        use crate::core::models::catalog::item_model::Items;

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
                item.description.clone().map_or_else(|| "NULL".into(), |d| d.into()),
                item.nature.to_string().into(),
                item.state.to_string().into(),
                item.price.to_string().into(),
                item.created_at.to_string().into(),
                item.updated_at.to_string().into(),
            ]);

        service.db_adapter.insert_many(&insert_stmt).await.unwrap();

        item
    }

    async fn create_test_discount(service: &mut AppService, name: Option<String>) -> Discount {
        let discount_input = DiscountNewInput {
            name: name.unwrap_or_else(|| format!("Test Discount {}", Uuid::now_v7())),
            description: Some("Test Discount Description".to_string()),
            discount_type: DiscountType::Percentage,
            value: Money::from_float(10.0),
            scope: DiscountScope::SpecificItems,
            state: Some(DiscountState::Active),
            start_date: None,
            end_date: None,
        };
        let create_discount_cmd = CreateDiscountCommand {
            discount: discount_input,
        };
        create_discount_cmd.exec(service).await.unwrap()
    }

    #[tokio::test]
    async fn test_add_item_discount() {
        let mut service = setup_service();
        let item = create_test_item(&mut service).await;
        let discount = create_test_discount(&mut service, None).await;

        let item_discount_input = ItemDiscountNewInput {
            item_id: item.id,
            discount_id: discount.id,
        };
        let add_cmd = AddItemDiscountCommand {
            item_discount: item_discount_input,
        };
        let result = add_cmd.exec(&mut service).await.unwrap();

        assert_eq!(result.item_id, item.id);
        assert_eq!(result.discount_id, discount.id);
    }

    #[tokio::test]
    async fn test_add_duplicate_item_discount() {
        let mut service = setup_service();
        let item = create_test_item(&mut service).await;
        let discount = create_test_discount(&mut service, None).await;

        // Add the relationship first time
        let item_discount_input = ItemDiscountNewInput {
            item_id: item.id,
            discount_id: discount.id,
        };
        let add_cmd = AddItemDiscountCommand {
            item_discount: item_discount_input.clone(),
        };
        add_cmd.exec(&mut service).await.unwrap();

        // Try to add the same relationship again
        let add_cmd = AddItemDiscountCommand {
            item_discount: item_discount_input,
        };
        let result = add_cmd.exec(&mut service).await.unwrap();

        // Should return the existing relationship without error
        assert_eq!(result.item_id, item.id);
        assert_eq!(result.discount_id, discount.id);

        // Verify only one relationship exists
        let get_cmd = GetItemDiscountsCommand { item_id: item.id };
        let result = get_cmd.exec(&mut service).await.unwrap();
        assert_eq!(result.len(), 1);
    }

    #[tokio::test]
    async fn test_get_item_discounts() {
        let mut service = setup_service();
        let item = create_test_item(&mut service).await;
        let discount1 = create_test_discount(&mut service, None).await;
        let discount2 = create_test_discount(&mut service, None).await;

        // Add two discounts to the same item
        let add_cmd1 = AddItemDiscountCommand {
            item_discount: ItemDiscountNewInput {
                item_id: item.id,
                discount_id: discount1.id,
            },
        };
        add_cmd1.exec(&mut service).await.unwrap();

        let add_cmd2 = AddItemDiscountCommand {
            item_discount: ItemDiscountNewInput {
                item_id: item.id,
                discount_id: discount2.id,
            },
        };
        add_cmd2.exec(&mut service).await.unwrap();

        // Get item discounts
        let get_cmd = GetItemDiscountsCommand { item_id: item.id };
        let result = get_cmd.exec(&mut service).await.unwrap();

        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|d| d.discount_id == discount1.id));
        assert!(result.iter().any(|d| d.discount_id == discount2.id));
    }

    #[tokio::test]
    async fn test_get_discount_items() {
        let mut service = setup_service();

        // Create a unique discount first
        let discount_name = format!("Discount for Items {}", Uuid::now_v7());
        let discount = create_test_discount(&mut service, Some(discount_name)).await;

        // Create items after the discount
        let item1 = create_test_item(&mut service).await;
        let item2 = create_test_item(&mut service).await;

        // Add the same discount to two items
        let add_cmd1 = AddItemDiscountCommand {
            item_discount: ItemDiscountNewInput {
                item_id: item1.id,
                discount_id: discount.id,
            },
        };
        add_cmd1.exec(&mut service).await.unwrap();

        let add_cmd2 = AddItemDiscountCommand {
            item_discount: ItemDiscountNewInput {
                item_id: item2.id,
                discount_id: discount.id,
            },
        };
        add_cmd2.exec(&mut service).await.unwrap();

        // Get discount items
        let get_cmd = GetDiscountItemsCommand {
            discount_id: discount.id,
        };
        let result = get_cmd.exec(&mut service).await.unwrap();

        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|d| d.item_id == item1.id));
        assert!(result.iter().any(|d| d.item_id == item2.id));
    }

    #[tokio::test]
    async fn test_remove_item_discount() {
        let mut service = setup_service();
        let item = create_test_item(&mut service).await;
        let discount = create_test_discount(&mut service, None).await;

        // Add the relationship
        let add_cmd = AddItemDiscountCommand {
            item_discount: ItemDiscountNewInput {
                item_id: item.id,
                discount_id: discount.id,
            },
        };
        add_cmd.exec(&mut service).await.unwrap();

        // Verify it exists
        let get_cmd = GetItemDiscountsCommand { item_id: item.id };
        let result = get_cmd.exec(&mut service).await.unwrap();
        assert_eq!(result.len(), 1);

        // Remove the relationship
        let remove_cmd = RemoveItemDiscountCommand {
            item_id: item.id,
            discount_id: discount.id,
        };
        let deleted_count = remove_cmd.exec(&mut service).await.unwrap();
        assert_eq!(deleted_count, 1);

        // Verify it's gone
        let get_cmd = GetItemDiscountsCommand { item_id: item.id };
        let result = get_cmd.exec(&mut service).await.unwrap();
        assert_eq!(result.len(), 0);
    }

    #[tokio::test]
    async fn test_remove_nonexistent_item_discount() {
        let mut service = setup_service();
        let item = create_test_item(&mut service).await;
        let discount = create_test_discount(&mut service, None).await;

        // Try to remove a relationship that doesn't exist
        let remove_cmd = RemoveItemDiscountCommand {
            item_id: item.id,
            discount_id: discount.id,
        };
        let deleted_count = remove_cmd.exec(&mut service).await.unwrap();
        assert_eq!(deleted_count, 0); // Should return 0 rows affected
    }
}
