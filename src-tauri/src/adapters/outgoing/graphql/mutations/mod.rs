use juniper::{graphql_object, FieldResult};

use crate::{
    core::entities::catalog::{
        item::{Item, NewItem, UpdateItem},
        item_category::{ItemCategory, NewItemCategory, UpdateItemCategory},
    },
    AppState,
};

use super::Mutation;

pub mod catalog;

#[graphql_object(context = AppState)]
impl Mutation {
    fn create_item(item: NewItem, context: &AppState) -> FieldResult<Item> {
        catalog::item::create_item(item, context)
    }

    fn update_item(item: UpdateItem, context: &AppState) -> FieldResult<Item> {
        catalog::item::update_item(item, context)
    }

    fn delete_item(id: String, context: &AppState) -> FieldResult<i32> {
        catalog::item::delete_item(id, context)
    }

    fn create_item_category(
        new_category: NewItemCategory,
        context: &AppState,
    ) -> FieldResult<ItemCategory> {
        catalog::item_category::create_item_category(new_category, context)
    }

    fn update_item_category(
        category: UpdateItemCategory,
        context: &AppState,
    ) -> FieldResult<ItemCategory> {
        catalog::item_category::update_item_category(category, context)
    }

    fn delete_item_category(id: String, context: &AppState) -> FieldResult<i32> {
        catalog::item_category::delete_item_category(id, context)
    }
}
