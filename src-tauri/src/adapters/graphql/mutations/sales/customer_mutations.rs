use crate::{
    core::{
        commands::{
            sales::customer_commands::{CreateCustomerCommand, DeleteCustomerCommand, UpdateCustomerCommand},
            Command,
        },
        models::sales::customer_model::{Customer, CustomerNewInput, CustomerUpdateInput},
        types::db_uuid::DbUuid,
    },
    AppState,
};
use juniper::FieldResult;

pub fn create_customer(customer: CustomerNewInput, context: &AppState) -> FieldResult<Customer> {
    let mut service = context.service.lock().unwrap();
    let res = CreateCustomerCommand { customer }.exec(&mut service)?;
    Ok(res)
}

pub fn update_customer(customer: CustomerUpdateInput, context: &AppState) -> FieldResult<Customer> {
    let mut service = context.service.lock().unwrap();
    let res = UpdateCustomerCommand { customer }.exec(&mut service)?;
    Ok(res)
}

pub fn delete_customer(id: DbUuid, context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().unwrap();
    let res = DeleteCustomerCommand { id }.exec(&mut service)?;
    Ok(res)
}
