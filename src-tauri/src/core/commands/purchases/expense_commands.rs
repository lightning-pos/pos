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
                category_id: self.expense.category_id,
                cost_center_id: self.expense.cost_center_id,
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
                category_id: self.expense.category_id,
                cost_center_id: self.expense.cost_center_id,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        commands::{purchases::purchase_category_commands::CreatePurchaseCategoryCommand, Command},
        models::{
            finance::cost_center_model::CostCenter,
            purchases::purchase_category_model::{PurchaseCategory, PurchaseCategoryNew},
        },
    };
    use diesel::{ExpressionMethods, RunQueryDsl};

    fn create_test_category(service: &mut AppService) -> PurchaseCategory {
        let command = CreatePurchaseCategoryCommand {
            category: PurchaseCategoryNew {
                name: "Test Category".to_string(),
                description: None,
                state: None,
            },
        };
        command.exec(service).unwrap()
    }

    #[test]
    fn test_create_expense() {
        let mut service = AppService::new(":memory:");

        // Create a category first
        let category = create_test_category(&mut service);

        // Create a cost center
        let cost_center = create_test_cost_center(&mut service);

        // Create an expense
        let now = Utc::now().naive_utc();
        let command = CreateExpenseCommand {
            expense: ExpenseNewInput {
                title: "Test Expense".to_string(),
                amount: 1000.into(),
                expense_date: now,
                category_id: category.id,
                cost_center_id: cost_center.id,
                description: Some("Test Description".to_string()),
            },
        };

        let expense = command.exec(&mut service).unwrap();
        assert_eq!(expense.title, "Test Expense");
        assert_eq!(expense.amount, 1000.into());
        assert_eq!(expense.category_id, category.id);
        assert_eq!(expense.cost_center_id, cost_center.id);
        assert_eq!(expense.description, Some("Test Description".to_string()));
    }

    #[test]
    fn test_update_expense() {
        let mut service = AppService::new(":memory:");

        // Create a category first
        let category = create_test_category(&mut service);

        // Create a cost center
        let cost_center = create_test_cost_center(&mut service);

        // Create an expense
        let now = Utc::now().naive_utc();
        let create_command = CreateExpenseCommand {
            expense: ExpenseNewInput {
                title: "Test Expense".to_string(),
                amount: 1000.into(),
                expense_date: now,
                category_id: category.id,
                cost_center_id: cost_center.id,
                description: Some("Test Description".to_string()),
            },
        };

        let expense = create_command.exec(&mut service).unwrap();

        // Update the expense
        let update_command = UpdateExpenseCommand {
            expense: ExpenseUpdateInput {
                id: expense.id,
                title: Some("Updated Expense".to_string()),
                amount: Some(2000.into()),
                expense_date: None,
                category_id: None,
                cost_center_id: None,
                description: None,
            },
        };

        let updated_expense = update_command.exec(&mut service).unwrap();
        assert_eq!(updated_expense.title, "Updated Expense");
        assert_eq!(updated_expense.amount, 2000.into());
        assert_eq!(updated_expense.category_id, category.id);
        assert_eq!(updated_expense.cost_center_id, cost_center.id);
        assert_eq!(
            updated_expense.description,
            Some("Test Description".to_string())
        );
    }

    #[test]
    fn test_delete_expense() {
        let mut service = AppService::new(":memory:");

        // Create a category first
        let category = create_test_category(&mut service);

        // Create a cost center
        let cost_center = create_test_cost_center(&mut service);

        // Create an expense
        let now = Utc::now().naive_utc();
        let create_command = CreateExpenseCommand {
            expense: ExpenseNewInput {
                title: "Test Expense".to_string(),
                amount: 1000.into(),
                expense_date: now,
                category_id: category.id,
                cost_center_id: cost_center.id,
                description: Some("Test Description".to_string()),
            },
        };

        let expense = create_command.exec(&mut service).unwrap();

        // Delete the expense
        let delete_command = DeleteExpenseCommand { id: expense.id };
        let result = delete_command.exec(&mut service).unwrap();
        assert_eq!(result, 1);

        // Verify expense no longer exists
        let count: i64 = expenses::table
            .filter(expenses::id.eq(expense.id))
            .count()
            .get_result(&mut service.conn)
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_update_expense_cost_center() {
        let mut service = AppService::new(":memory:");

        // Create a category
        let category = create_test_category(&mut service);

        // Create two cost centers
        let cost_center1 = create_test_cost_center(&mut service);
        let cost_center2 =
            create_test_cost_center_with_name(&mut service, "Second Cost Center", "CC002");

        // Create an expense with the first cost center
        let now = Utc::now().naive_utc();
        let create_command = CreateExpenseCommand {
            expense: ExpenseNewInput {
                title: "Test Expense".to_string(),
                amount: 1000.into(),
                expense_date: now,
                category_id: category.id,
                cost_center_id: cost_center1.id,
                description: Some("Test Description".to_string()),
            },
        };

        let expense = create_command.exec(&mut service).unwrap();
        assert_eq!(expense.cost_center_id, cost_center1.id);

        // Update only the cost center
        let update_command = UpdateExpenseCommand {
            expense: ExpenseUpdateInput {
                id: expense.id,
                title: None,
                amount: None,
                expense_date: None,
                category_id: None,
                cost_center_id: Some(cost_center2.id),
                description: None,
            },
        };

        let updated_expense = update_command.exec(&mut service).unwrap();

        // Verify that only the cost center was updated
        assert_eq!(updated_expense.title, "Test Expense");
        assert_eq!(updated_expense.amount, 1000.into());
        assert_eq!(updated_expense.category_id, category.id);
        assert_eq!(updated_expense.cost_center_id, cost_center2.id);
        assert_eq!(
            updated_expense.description,
            Some("Test Description".to_string())
        );
    }

    fn create_test_cost_center(service: &mut AppService) -> CostCenter {
        use crate::core::{
            commands::{finance::cost_center_commands::CreateCostCenterCommand, Command},
            models::finance::cost_center_model::{CostCenterNewInput, CostCenterState},
        };

        let command = CreateCostCenterCommand {
            cost_center: CostCenterNewInput {
                name: "Test Cost Center".to_string(),
                code: "TCC001".to_string(),
                description: None,
                state: Some(CostCenterState::Active),
            },
        };
        command.exec(service).unwrap()
    }

    fn create_test_cost_center_with_name(
        service: &mut AppService,
        name: &str,
        code: &str,
    ) -> CostCenter {
        use crate::core::{
            commands::{finance::cost_center_commands::CreateCostCenterCommand, Command},
            models::finance::cost_center_model::{CostCenterNewInput, CostCenterState},
        };

        let command = CreateCostCenterCommand {
            cost_center: CostCenterNewInput {
                name: name.to_string(),
                code: code.to_string(),
                description: None,
                state: Some(CostCenterState::Active),
            },
        };
        command.exec(service).unwrap()
    }
}
