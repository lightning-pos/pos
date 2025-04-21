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

pub async fn create_sales_order_payment(
    context: &AppState,
    payment: SalesOrderPaymentNewInput,
) -> FieldResult<SalesOrderPayment> {
    let mut service = context.service.lock().await;
    let cmd = CreateSalesOrderPaymentCommand { payment };
    let result = cmd.exec(&mut service).await?;
    Ok(result)
}

pub async fn update_sales_order_payment(
    context: &AppState,
    payment: SalesOrderPaymentUpdateInput,
) -> FieldResult<SalesOrderPayment> {
    let mut service = context.service.lock().await;
    let cmd = UpdateSalesOrderPaymentCommand { payment };
    let result = cmd.exec(&mut service).await?;
    Ok(result)
}

pub async fn void_sales_order_payment(context: &AppState, id: DbUuid) -> FieldResult<SalesOrderPayment> {
    let mut service = context.service.lock().await;
    let cmd = VoidSalesOrderPaymentCommand { id };
    let result = cmd.exec(&mut service).await?;
    Ok(result)
}
