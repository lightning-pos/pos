use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};

use crate::core::commands::{app_service::AppService, Command};
use crate::core::models::catalog::item_discount_model::{ItemDiscount, ItemDiscountNewInput};
use crate::core::types::db_uuid::DbUuid;
use crate::error::Result;
use crate::schema::item_discounts::dsl::*;

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

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let conn = &mut service.conn;

        // Check if the relationship already exists
        let existing = item_discounts
            .filter(item_id.eq(&self.item_discount.item_id))
            .filter(discount_id.eq(&self.item_discount.discount_id))
            .first::<ItemDiscount>(conn)
            .optional()?;

        if let Some(existing_relation) = existing {
            return Ok(existing_relation);
        }

        // Create new relationship
        let new_item_discount = ItemDiscount {
            item_id: self.item_discount.item_id,
            discount_id: self.item_discount.discount_id,
        };

        diesel::insert_into(item_discounts)
            .values(&new_item_discount)
            .execute(conn)?;

        Ok(new_item_discount)
    }
}

impl Command for RemoveItemDiscountCommand {
    type Output = usize;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let conn = &mut service.conn;

        let deleted_count = diesel::delete(
            item_discounts
                .filter(item_id.eq(&self.item_id))
                .filter(discount_id.eq(&self.discount_id)),
        )
        .execute(conn)?;

        Ok(deleted_count)
    }
}

impl Command for GetItemDiscountsCommand {
    type Output = Vec<ItemDiscount>;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let conn = &mut service.conn;

        let result = item_discounts
            .filter(item_id.eq(&self.item_id))
            .load::<ItemDiscount>(conn)?;

        Ok(result)
    }
}

impl Command for GetDiscountItemsCommand {
    type Output = Vec<ItemDiscount>;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let conn = &mut service.conn;

        let result = item_discounts
            .filter(discount_id.eq(&self.discount_id))
            .load::<ItemDiscount>(conn)?;

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
        item_group_model::ItemGroupNew,
    };
    use crate::core::types::money::Money;
    use crate::schema::items;
    use chrono::Utc;
    use diesel::RunQueryDsl;
    use uuid::Uuid;

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
            price: Money::from_float(100.0),
            created_at: now,
            updated_at: now,
        };

        diesel::insert_into(items::table)
            .values(&item)
            .execute(&mut service.conn)
            .unwrap();

        item
    }

    fn create_test_discount(service: &mut AppService, name: Option<String>) -> Discount {
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
        create_discount_cmd.exec(service).unwrap()
    }

    #[test]
    fn test_add_item_discount() {
        let mut service = setup_service();
        let item = create_test_item(&mut service);
        let discount = create_test_discount(&mut service, None);

        let item_discount_input = ItemDiscountNewInput {
            item_id: item.id,
            discount_id: discount.id,
        };
        let add_cmd = AddItemDiscountCommand {
            item_discount: item_discount_input,
        };
        let result = add_cmd.exec(&mut service).unwrap();

        assert_eq!(result.item_id, item.id);
        assert_eq!(result.discount_id, discount.id);
    }

    #[test]
    fn test_add_duplicate_item_discount() {
        let mut service = setup_service();
        let item = create_test_item(&mut service);
        let discount = create_test_discount(&mut service, None);

        // Add the relationship first time
        let item_discount_input = ItemDiscountNewInput {
            item_id: item.id,
            discount_id: discount.id,
        };
        let add_cmd = AddItemDiscountCommand {
            item_discount: item_discount_input.clone(),
        };
        add_cmd.exec(&mut service).unwrap();

        // Try to add the same relationship again
        let add_cmd = AddItemDiscountCommand {
            item_discount: item_discount_input,
        };
        let result = add_cmd.exec(&mut service).unwrap();

        // Should return the existing relationship without error
        assert_eq!(result.item_id, item.id);
        assert_eq!(result.discount_id, discount.id);

        // Verify only one relationship exists
        let get_cmd = GetItemDiscountsCommand { item_id: item.id };
        let result = get_cmd.exec(&mut service).unwrap();
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_get_item_discounts() {
        let mut service = setup_service();
        let item = create_test_item(&mut service);
        let discount1 = create_test_discount(&mut service, None);
        let discount2 = create_test_discount(&mut service, None);

        // Add two discounts to the same item
        let add_cmd1 = AddItemDiscountCommand {
            item_discount: ItemDiscountNewInput {
                item_id: item.id,
                discount_id: discount1.id,
            },
        };
        add_cmd1.exec(&mut service).unwrap();

        let add_cmd2 = AddItemDiscountCommand {
            item_discount: ItemDiscountNewInput {
                item_id: item.id,
                discount_id: discount2.id,
            },
        };
        add_cmd2.exec(&mut service).unwrap();

        // Get item discounts
        let get_cmd = GetItemDiscountsCommand { item_id: item.id };
        let result = get_cmd.exec(&mut service).unwrap();

        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|d| d.discount_id == discount1.id));
        assert!(result.iter().any(|d| d.discount_id == discount2.id));
    }

    #[test]
    fn test_get_discount_items() {
        let mut service = setup_service();

        // Create a unique discount first
        let discount_name = format!("Discount for Items {}", Uuid::now_v7());
        let discount = create_test_discount(&mut service, Some(discount_name));

        // Create items after the discount
        let item1 = create_test_item(&mut service);
        let item2 = create_test_item(&mut service);

        // Add the same discount to two items
        let add_cmd1 = AddItemDiscountCommand {
            item_discount: ItemDiscountNewInput {
                item_id: item1.id,
                discount_id: discount.id,
            },
        };
        add_cmd1.exec(&mut service).unwrap();

        let add_cmd2 = AddItemDiscountCommand {
            item_discount: ItemDiscountNewInput {
                item_id: item2.id,
                discount_id: discount.id,
            },
        };
        add_cmd2.exec(&mut service).unwrap();

        // Get discount items
        let get_cmd = GetDiscountItemsCommand {
            discount_id: discount.id,
        };
        let result = get_cmd.exec(&mut service).unwrap();

        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|d| d.item_id == item1.id));
        assert!(result.iter().any(|d| d.item_id == item2.id));
    }

    #[test]
    fn test_remove_item_discount() {
        let mut service = setup_service();
        let item = create_test_item(&mut service);
        let discount = create_test_discount(&mut service, None);

        // Add the relationship
        let add_cmd = AddItemDiscountCommand {
            item_discount: ItemDiscountNewInput {
                item_id: item.id,
                discount_id: discount.id,
            },
        };
        add_cmd.exec(&mut service).unwrap();

        // Verify it exists
        let get_cmd = GetItemDiscountsCommand { item_id: item.id };
        let result = get_cmd.exec(&mut service).unwrap();
        assert_eq!(result.len(), 1);

        // Remove the relationship
        let remove_cmd = RemoveItemDiscountCommand {
            item_id: item.id,
            discount_id: discount.id,
        };
        let deleted_count = remove_cmd.exec(&mut service).unwrap();
        assert_eq!(deleted_count, 1);

        // Verify it's gone
        let get_cmd = GetItemDiscountsCommand { item_id: item.id };
        let result = get_cmd.exec(&mut service).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_remove_nonexistent_item_discount() {
        let mut service = setup_service();
        let item = create_test_item(&mut service);
        let discount = create_test_discount(&mut service, None);

        // Try to remove a relationship that doesn't exist
        let remove_cmd = RemoveItemDiscountCommand {
            item_id: item.id,
            discount_id: discount.id,
        };
        let deleted_count = remove_cmd.exec(&mut service).unwrap();
        assert_eq!(deleted_count, 0); // Should return 0 rows affected
    }
}
