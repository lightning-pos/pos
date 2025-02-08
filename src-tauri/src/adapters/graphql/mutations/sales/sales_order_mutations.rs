use crate::{
    core::{
        commands::{
            sales::sales_order_commands::{CreateSalesOrderCommand, VoidSalesOrderCommand},
            Command,
        },
        models::sales::sales_order_model::{SalesOrder, SalesOrderNewInput},
        types::db_uuid::DbUuid,
    },
    AppState,
};
use juniper::FieldResult;

pub fn create_sales_order(sales_order: SalesOrderNewInput, context: &AppState) -> FieldResult<SalesOrder> {
    let mut service = context.service.lock().unwrap();
    let res = CreateSalesOrderCommand { sales_order }.exec(&mut service)?;
    Ok(res)
}

pub fn void_sales_order(id: DbUuid, context: &AppState) -> FieldResult<SalesOrder> {
    let mut service = context.service.lock().unwrap();
    let res = VoidSalesOrderCommand { id }.exec(&mut service)?;
    Ok(res)
}
