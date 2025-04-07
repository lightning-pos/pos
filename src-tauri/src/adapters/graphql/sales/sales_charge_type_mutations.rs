use crate::{
    core::{
        commands::{
            sales::sales_charge_type_commands::{
                CreateSalesChargeTypeCommand, DeleteSalesChargeTypeCommand,
                UpdateSalesChargeTypeCommand,
            },
            Command,
        },
        models::sales::sales_charge_type_model::{
            SalesChargeType, SalesChargeTypeNewInput, SalesChargeTypeUpdateInput,
        },
        types::db_uuid::DbUuid,
    },
    AppState,
};
use juniper::FieldResult;

pub fn create_sales_charge_type(
    charge_type: SalesChargeTypeNewInput,
    context: &AppState,
) -> FieldResult<SalesChargeType> {
    let mut service = context.service.lock().unwrap();
    let res = CreateSalesChargeTypeCommand { charge_type }.exec(&mut service)?;
    Ok(res)
}

pub fn update_sales_charge_type(
    charge_type: SalesChargeTypeUpdateInput,
    context: &AppState,
) -> FieldResult<SalesChargeType> {
    let mut service = context.service.lock().unwrap();
    let res = UpdateSalesChargeTypeCommand { charge_type }.exec(&mut service)?;
    Ok(res)
}

pub fn delete_sales_charge_type(id: DbUuid, context: &AppState) -> FieldResult<bool> {
    let mut service = context.service.lock().unwrap();
    let res = DeleteSalesChargeTypeCommand { id }.exec(&mut service)?;
    Ok(res)
}
