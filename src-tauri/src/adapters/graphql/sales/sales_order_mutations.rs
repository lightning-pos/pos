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
use uuid::Uuid;

// Placeholder function to get user ID (replace with actual logic)
fn get_current_user_id(_context: &AppState) -> DbUuid {
    // TODO: Implement actual user ID retrieval from context/session
    Uuid::nil().into() // Using Nil UUID as a placeholder
}

pub fn create_sales_order(
    sales_order: SalesOrderNewInput,
    context: &AppState,
) -> FieldResult<SalesOrder> {
    let mut service = context.service.lock().unwrap();
    let current_user_id = get_current_user_id(context);
    let res = CreateSalesOrderCommand {
        sales_order,
        created_by_user_id: current_user_id,
    }
    .exec(&mut service)?;
    Ok(res)
}

pub fn void_sales_order(id: DbUuid, context: &AppState) -> FieldResult<SalesOrder> {
    let mut service = context.service.lock().unwrap();
    let current_user_id = get_current_user_id(context);
    let res = VoidSalesOrderCommand {
        id,
        updated_by_user_id: current_user_id,
    }
    .exec(&mut service)?;
    Ok(res)
}
