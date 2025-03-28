use chrono::Utc;
use diesel::{Connection, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{
    core::{
        commands::{app_service::AppService, Command},
        models::purchases::expense_model::{
            Expense, ExpenseNewInput, ExpenseUpdateChangeset, ExpenseUpdateInput,
        },
        types::db_uuid::DbUuid,
    },
    error::Result,
    schema::expenses,
};

// Commands
pub struct CreateExpenseCommand {
    pub expense: ExpenseNewInput,
}

pub struct UpdateExpenseCommand {
    pub expense: ExpenseUpdateInput,
}

pub struct DeleteExpenseCommand {
    pub id: DbUuid,
}

// Command Implementations
impl Command for CreateExpenseCommand {
    type Output = Expense;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let now = Utc::now().naive_utc();
            let new_expense = Expense {
                id: Uuid::now_v7().into(),
                title: self.expense.title.clone(),
                amount: self.expense.amount,
                expense_date: self.expense.expense_date,
                category: self.expense.category.clone(),
                description: self.expense.description.clone(),
                created_at: now,
                updated_at: now,
            };

            let res = diesel::insert_into(expenses::table)
                .values(&new_expense)
                .returning(Expense::as_returning())
                .get_result(conn)?;

            Ok(res)
        })
    }
}

impl Command for UpdateExpenseCommand {
    type Output = Expense;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let now = Utc::now().naive_utc();
            let expense_id = self.expense.id;

            let changeset = ExpenseUpdateChangeset {
                id: expense_id,
                title: self.expense.title.clone(),
                amount: self.expense.amount,
                expense_date: self.expense.expense_date,
                category: self.expense.category.clone(),
                description: self.expense.description.clone(),
                updated_at: now,
            };

            let res = diesel::update(expenses::table.find(expense_id))
                .set(changeset)
                .returning(Expense::as_returning())
                .get_result(conn)?;

            Ok(res)
        })
    }
}

impl Command for DeleteExpenseCommand {
    type Output = i32;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let res = diesel::delete(expenses::table.find(self.id)).execute(conn)?;
            Ok(res as i32)
        })
    }
}
