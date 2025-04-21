use juniper::FieldResult;

use crate::{
    core::{
        commands::{
            common::brand_commands::{CreateBrandCommand, DeleteBrandCommand, UpdateBrandCommand},
            Command,
        },
        models::common::brand_model::{Brand, BrandNewInput, BrandUpdateInput},
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub async fn create_brand(input: BrandNewInput, context: &AppState) -> FieldResult<Brand> {
    let mut service = context.service.lock().await;
    let res = CreateBrandCommand { brand: input }.exec(&mut service).await?;
    Ok(res)
}

pub async fn update_brand(input: BrandUpdateInput, context: &AppState) -> FieldResult<Brand> {
    let mut service = context.service.lock().await;
    let res = UpdateBrandCommand { brand: input }.exec(&mut service).await?;
    Ok(res)
}

pub async fn delete_brand(id: DbUuid, context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().await;
    let res = DeleteBrandCommand { id }.exec(&mut service).await?;
    Ok(res)
}
