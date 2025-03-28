use crate::{
    core::{
        commands::{
            sales::supplier_commands::{
                CreateSupplierCommand, DeleteSupplierCommand, UpdateSupplierCommand,
            },
            Command,
        },
        models::sales::supplier_model::{Supplier, SupplierNewInput, SupplierUpdateInput},
        types::db_uuid::DbUuid,
    },
    AppState,
};
use juniper::FieldResult;

pub fn create_supplier(supplier: SupplierNewInput, context: &AppState) -> FieldResult<Supplier> {
    let mut service = context.service.lock().unwrap();
    let res = CreateSupplierCommand { supplier }.exec(&mut service)?;
    Ok(res)
}

pub fn update_supplier(supplier: SupplierUpdateInput, context: &AppState) -> FieldResult<Supplier> {
    let mut service = context.service.lock().unwrap();
    let res = UpdateSupplierCommand { supplier }.exec(&mut service)?;
    Ok(res)
}

pub fn delete_supplier(id: DbUuid, context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().unwrap();
    let res = DeleteSupplierCommand { id }.exec(&mut service)?;
    Ok(res)
}
