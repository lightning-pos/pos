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

pub async fn create_cart(cart: CartNewInput, context: &AppState) -> FieldResult<Cart> {
    let mut service = context.service.lock().await;
    let res = CreateCartCommand { cart }.exec(&mut service).await?;
    Ok(res)
}

pub async fn update_cart(cart: CartUpdateInput, context: &AppState) -> FieldResult<Cart> {
    let mut service = context.service.lock().await;
    let res = UpdateCartCommand { cart }.exec(&mut service).await?;
    Ok(res)
}

pub async fn delete_cart(id: DbUuid, context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().await;
    let res = DeleteCartCommand { id }.exec(&mut service).await?;
    Ok(res)
}
