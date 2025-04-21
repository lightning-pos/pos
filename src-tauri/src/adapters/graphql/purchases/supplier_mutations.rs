use crate::{
    core::{
        commands::{
            purchases::supplier_commands::{
                CreateSupplierCommand, DeleteSupplierCommand, UpdateSupplierCommand,
            },
            Command,
        },
        models::purchases::supplier_model::{Supplier, SupplierNewInput, SupplierUpdateInput},
        types::db_uuid::DbUuid,
    },
    AppState,
};
use juniper::FieldResult;

pub async fn create_supplier(supplier: SupplierNewInput, context: &AppState) -> FieldResult<Supplier> {
    let mut service = context.service.lock().await;
    let res = CreateSupplierCommand { supplier }.exec(&mut service).await?;
    Ok(res)
}

pub async fn update_supplier(supplier: SupplierUpdateInput, context: &AppState) -> FieldResult<Supplier> {
    let mut service = context.service.lock().await;
    let res = UpdateSupplierCommand { supplier }.exec(&mut service).await?;
    Ok(res)
}

pub async fn delete_supplier(id: DbUuid, context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().await;
    let res = DeleteSupplierCommand { id }.exec(&mut service).await?;
    Ok(res)
}
