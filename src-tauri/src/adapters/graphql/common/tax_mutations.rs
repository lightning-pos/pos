use juniper::FieldResult;

use crate::{
    core::{
        commands::{
            common::tax_commands::{
                AssignTaxToItemCommand, CreateTaxCommand, DeleteTaxCommand,
                RemoveTaxFromItemCommand, UpdateTaxCommand,
            },
            Command,
        },
        models::common::tax_model::{Tax, TaxNewInput, TaxUpdateInput, ItemTaxNewInput},
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub fn create_tax(input: TaxNewInput, context: &AppState) -> FieldResult<Tax> {
    let mut service = context.service.lock().unwrap();
    let res = CreateTaxCommand { tax: input }.exec(&mut service)?;
    Ok(res)
}

pub fn update_tax(input: TaxUpdateInput, context: &AppState) -> FieldResult<Tax> {
    let mut service = context.service.lock().unwrap();
    let res = UpdateTaxCommand { tax: input }.exec(&mut service)?;
    Ok(res)
}

pub fn delete_tax(id: DbUuid, context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().unwrap();
    let res = DeleteTaxCommand { id }.exec(&mut service)?;
    Ok(res)
}

pub fn assign_tax_to_item(input: ItemTaxNewInput, context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().unwrap();
    let res = AssignTaxToItemCommand { item_tax: input }.exec(&mut service)?;
    Ok(res)
}

pub fn remove_tax_from_item(
    item_id: DbUuid,
    tax_id: DbUuid,
    context: &AppState,
) -> FieldResult<i32> {
    let mut service = context.service.lock().unwrap();
    let res = RemoveTaxFromItemCommand { item_id, tax_id }.exec(&mut service)?;
    Ok(res)
}
