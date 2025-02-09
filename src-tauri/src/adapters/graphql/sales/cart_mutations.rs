use crate::{
    core::{
        commands::{
            sales::cart_commands::{CreateCartCommand, DeleteCartCommand, UpdateCartCommand},
            Command,
        },
        models::sales::cart_model::{Cart, CartNewInput, CartUpdateInput},
        types::db_uuid::DbUuid,
    },
    AppState,
};
use juniper::FieldResult;

pub fn create_cart(cart: CartNewInput, context: &AppState) -> FieldResult<Cart> {
    let mut service = context.service.lock().unwrap();
    let res = CreateCartCommand { cart }.exec(&mut service)?;
    Ok(res)
}

pub fn update_cart(cart: CartUpdateInput, context: &AppState) -> FieldResult<Cart> {
    let mut service = context.service.lock().unwrap();
    let res = UpdateCartCommand { cart }.exec(&mut service)?;
    Ok(res)
}

pub fn delete_cart(id: DbUuid, context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().unwrap();
    let res = DeleteCartCommand { id }.exec(&mut service)?;
    Ok(res)
}
