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

pub async fn create_expense(expense: ExpenseNewInput, context: &AppState) -> FieldResult<Expense> {
    let mut service = context.service.lock().await;
    let res = CreateExpenseCommand { expense }.exec(&mut service).await?;
    Ok(res)
}

pub async fn update_expense(expense: ExpenseUpdateInput, context: &AppState) -> FieldResult<Expense> {
    let mut service = context.service.lock().await;
    let res = UpdateExpenseCommand { expense }.exec(&mut service).await?;
    Ok(res)
}

pub async fn delete_expense(id: DbUuid, context: &AppState) -> FieldResult<i32> {
    let mut service = context.service.lock().await;
    let res = DeleteExpenseCommand { id }.exec(&mut service).await?;
    Ok(res)
}
