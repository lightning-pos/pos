use juniper::FieldResult;

use crate::{
    core::{
        commands::{
            finance::sales_order_payment_commands::{
                CreateSalesOrderPaymentCommand, UpdateSalesOrderPaymentCommand,
                VoidSalesOrderPaymentCommand,
            },
            Command,
        },
        models::finance::sales_order_payment_model::{
            SalesOrderPayment, SalesOrderPaymentNewInput, SalesOrderPaymentUpdateInput,
        },
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub fn create_sales_order_payment(
    context: &AppState,
    payment: SalesOrderPaymentNewInput,
) -> FieldResult<SalesOrderPayment> {
    let mut service = context.service.lock().unwrap();
    let cmd = CreateSalesOrderPaymentCommand { payment };
    let result = cmd.exec(&mut service)?;
    Ok(result)
}

pub fn update_sales_order_payment(
    context: &AppState,
    payment: SalesOrderPaymentUpdateInput,
) -> FieldResult<SalesOrderPayment> {
    let mut service = context.service.lock().unwrap();
    let cmd = UpdateSalesOrderPaymentCommand { payment };
    let result = cmd.exec(&mut service)?;
    Ok(result)
}

pub fn void_sales_order_payment(context: &AppState, id: DbUuid) -> FieldResult<SalesOrderPayment> {
    let mut service = context.service.lock().unwrap();
    let cmd = VoidSalesOrderPaymentCommand { id };
    let result = cmd.exec(&mut service)?;
    Ok(result)
}
