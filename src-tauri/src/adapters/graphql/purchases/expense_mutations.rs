use crate::{
    core::{
        commands::{
            purchases::expense_commands::{
                CreateExpenseCommand, DeleteExpenseCommand, UpdateExpenseCommand,
            },
            Command,
        },
        models::purchases::expense_model::{Expense, ExpenseNewInput, ExpenseUpdateInput},
        types::db_uuid::DbUuid,
    },
    AppState,
};
use juniper::FieldResult;

pub fn create_expense(expense: ExpenseNewInput, context: &AppState) -> FieldResult<Expense> {
    let mut service = context.service.lock().unwrap();
    let res = CreateExpenseCommand { expense }.exec(&mut service)?;
    Ok(res)
}

pub fn update_expense(expense: ExpenseUpdateInput, context: &AppState) -> FieldResult<Expense> {
    let mut service = context.service.lock().unwrap();
    let res = UpdateExpenseCommand { expense }.exec(&mut service)?;
    Ok(res)
}

pub fn delete_expense(id: DbUuid, context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().unwrap();
    let res = DeleteExpenseCommand { id }.exec(&mut service)?;
    Ok(res)
}
