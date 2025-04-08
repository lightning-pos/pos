use juniper::FieldResult;

use crate::{
    core::{
        commands::{
            catalog::{
                discount_commands::GetDiscountCommand,
                item_commands::UpdateItemCommand,
                item_discount_commands::{
                    AddItemDiscountCommand, GetDiscountItemsCommand, GetItemDiscountsCommand,
                    RemoveItemDiscountCommand,
                },
            },
            Command,
        },
        models::catalog::item_discount_model::{ItemDiscount, ItemDiscountNewInput},
        types::db_uuid::DbUuid,
    },
    AppState,
};

#[derive(juniper::GraphQLObject)]
#[graphql(description = "A relationship between an item and a discount")]
pub struct ItemDiscountObject {
    pub item_id: DbUuid,
    pub discount_id: DbUuid,
}

impl From<ItemDiscount> for ItemDiscountObject {
    fn from(item_discount: ItemDiscount) -> Self {
        Self {
            item_id: item_discount.item_id,
            discount_id: item_discount.discount_id,
        }
    }
}

// Query resolvers
pub struct ItemDiscountQuery;

#[juniper::graphql_object(context = AppState)]
impl ItemDiscountQuery {
    #[graphql(description = "Get all discounts for an item")]
    pub fn item_discounts(
        context: &AppState,
        item_id: DbUuid,
    ) -> FieldResult<Vec<ItemDiscountObject>> {
        let mut service = context.service.lock().unwrap();
        let cmd = GetItemDiscountsCommand { item_id };
        let item_discounts = cmd.exec(&mut service)?;
        Ok(item_discounts.into_iter().map(Into::into).collect())
    }

    #[graphql(description = "Get all items for a discount")]
    pub fn discount_items(
        context: &AppState,
        discount_id: DbUuid,
    ) -> FieldResult<Vec<ItemDiscountObject>> {
        let mut service = context.service.lock().unwrap();
        let cmd = GetDiscountItemsCommand { discount_id };
        let discount_items = cmd.exec(&mut service)?;
        Ok(discount_items.into_iter().map(Into::into).collect())
    }
}

// Mutation resolvers
pub struct ItemDiscountMutation;

#[juniper::graphql_object(context = AppState)]
impl ItemDiscountMutation {
    #[graphql(description = "Add a discount to an item")]
    pub fn add_item_discount(
        context: &AppState,
        item_discount: ItemDiscountNewInput,
    ) -> FieldResult<ItemDiscountObject> {
        let mut service = context.service.lock().unwrap();

        // Verify that the item exists
        let get_item_cmd = UpdateItemCommand {
            item: crate::core::models::catalog::item_model::UpdateItem {
                id: item_discount.item_id,
                name: None,
                description: None,
                nature: None,
                state: None,
                price: None,
                category_id: None,
                updated_at: None,
            },
        };
        get_item_cmd.exec(&mut service)?;

        // Verify that the discount exists
        let get_discount_cmd = GetDiscountCommand {
            id: item_discount.discount_id,
        };
        get_discount_cmd.exec(&mut service)?;

        // Add the relationship
        let cmd = AddItemDiscountCommand { item_discount };
        let item_discount = cmd.exec(&mut service)?;
        Ok(item_discount.into())
    }

    #[graphql(description = "Remove a discount from an item")]
    pub fn remove_item_discount(
        context: &AppState,
        item_id: DbUuid,
        discount_id: DbUuid,
    ) -> FieldResult<bool> {
        let mut service = context.service.lock().unwrap();
        let cmd = RemoveItemDiscountCommand {
            item_id,
            discount_id,
        };
        let deleted_count = cmd.exec(&mut service)?;
        Ok(deleted_count > 0)
    }
}
