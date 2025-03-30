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

pub fn create_brand(input: BrandNewInput, context: &AppState) -> FieldResult<Brand> {
    let mut service = context.service.lock().unwrap();
    let res = CreateBrandCommand { brand: input }.exec(&mut service)?;
    Ok(res)
}

pub fn update_brand(input: BrandUpdateInput, context: &AppState) -> FieldResult<Brand> {
    let mut service = context.service.lock().unwrap();
    let res = UpdateBrandCommand { brand: input }.exec(&mut service)?;
    Ok(res)
}

pub fn delete_brand(id: DbUuid, context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().unwrap();
    let res = DeleteBrandCommand { id }.exec(&mut service)?;
    Ok(res)
}
