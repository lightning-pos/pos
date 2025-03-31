use juniper::FieldResult;

use crate::{
    core::{
        commands::{finance::sales_order_payment_commands::GetSalesOrderPaymentsCommand, Command},
        models::finance::sales_order_payment_model::SalesOrderPayment,
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub fn sales_order_payments(
    context: &AppState,
    order_id: DbUuid,
) -> FieldResult<Vec<SalesOrderPayment>> {
    let mut service = context.service.lock().unwrap();

    let cmd = GetSalesOrderPaymentsCommand { order_id };
    let result = cmd.exec(&mut service)?;

    Ok(result)
}
