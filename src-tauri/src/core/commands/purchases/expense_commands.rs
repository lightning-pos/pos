use chrono::Utc;
use sea_query::{Expr, Query, SqliteQueryBuilder};
use uuid::Uuid;

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        commands::{app_service::AppService, Command},
        models::purchases::expense_model::{
            Expense, ExpenseNewInput, ExpenseUpdateInput, Expenses,
        },
        types::db_uuid::DbUuid,
    },
    error::Result,
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

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let now = Utc::now().naive_utc();
        let new_id = Uuid::now_v7();

        let new_expense = Expense {
            id: new_id.into(),
            title: self.expense.title.clone(),
            amount: self.expense.amount,
            expense_date: self.expense.expense_date,
            category_id: self.expense.category_id,
            cost_center_id: self.expense.cost_center_id,
            description: self.expense.description.clone(),
            created_at: now,
            updated_at: now,
        };

        // Build the insert query with SeaQuery
        let mut query = Query::insert();
        let stmt = query
            .into_table(Expenses::Table)
            .columns([
                Expenses::Id,
                Expenses::Title,
                Expenses::Amount,
                Expenses::ExpenseDate,
                Expenses::CategoryId,
                Expenses::CostCenterId,
                Expenses::Description,
                Expenses::CreatedAt,
                Expenses::UpdatedAt,
            ])
            .values_panic([
                new_id.to_string().into(),
                self.expense.title.clone().into(),
                self.expense.amount.to_base_unit().into(),
                self.expense.expense_date.to_string().into(),
                self.expense.category_id.to_string().into(),
                self.expense.cost_center_id.to_string().into(),
                self.expense.description.clone().into(),
                now.to_string().into(),
                now.to_string().into(),
            ]);

        println!("Inserting expense: {}", stmt.to_string(SqliteQueryBuilder));

        // Execute the query
        service.db_adapter.insert_many(&stmt).await?;

        // Return the newly created expense
        Ok(new_expense)
    }
}

impl Command for UpdateExpenseCommand {
    type Output = Expense;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let now = Utc::now().naive_utc();
        let expense_id = self.expense.id;

        // First, get the current expense
        let mut get_query = Query::select();
        let get_stmt = get_query
            .from(Expenses::Table)
            .columns([
                Expenses::Id,
                Expenses::Title,
                Expenses::Amount,
                Expenses::ExpenseDate,
                Expenses::CategoryId,
                Expenses::CostCenterId,
                Expenses::Description,
                Expenses::CreatedAt,
                Expenses::UpdatedAt,
            ])
            .and_where(Expr::col(Expenses::Id).eq(expense_id.to_string()));

        let mut update_query = Query::update();
        let mut update_stmt = update_query.table(Expenses::Table);

        // Only set fields that are provided in the update input
        if let Some(title) = &self.expense.title {
            update_stmt = update_stmt.value(Expenses::Title, title.clone());
        }

        if let Some(amount) = &self.expense.amount {
            update_stmt = update_stmt.value(Expenses::Amount, amount.to_base_unit());
        }

        if let Some(expense_date) = &self.expense.expense_date {
            update_stmt = update_stmt.value(Expenses::ExpenseDate, expense_date.to_string());
        }

        if let Some(category_id) = &self.expense.category_id {
            update_stmt = update_stmt.value(Expenses::CategoryId, category_id.to_string());
        }

        if let Some(cost_center_id) = &self.expense.cost_center_id {
            update_stmt = update_stmt.value(Expenses::CostCenterId, cost_center_id.to_string());
        }

        if let Some(description) = &self.expense.description {
            match description {
                Some(desc) => update_stmt = update_stmt.value(Expenses::Description, desc.clone()),
                None => update_stmt = update_stmt.value(Expenses::Description, sea_query::Value::String(None)),
            };
        }

        // Always update the updated_at timestamp
        update_stmt = update_stmt.value(Expenses::UpdatedAt, now.to_string());

        // Add the WHERE clause
        update_stmt = update_stmt.and_where(Expr::col(Expenses::Id).eq(expense_id.to_string()));

        // Execute the query
        service.db_adapter.update_many(&update_stmt).await?;

        // Get the updated expense
        let updated_expense = service.db_adapter.query_one::<Expense>(&get_stmt).await?;

        Ok(updated_expense)
    }
}

impl Command for DeleteExpenseCommand {
    type Output = i32;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Build the delete query with SeaQuery
        let mut query = Query::delete();
        let stmt = query
            .from_table(Expenses::Table)
            .and_where(Expr::col(Expenses::Id).eq(self.id.to_string()));

        // Execute the query
        let affected_rows = service.db_adapter.delete(&stmt).await?;

        Ok(affected_rows as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        commands::{purchases::purchase_category_commands::CreatePurchaseCategoryCommand, tests::setup_service, Command},
        models::{
            finance::cost_center_model::CostCenter,
            purchases::purchase_category_model::{PurchaseCategory, PurchaseCategoryNew},
        },
    };
    use sea_query::{Alias, Expr, Query};
    use tokio;

    async fn create_test_category(service: &mut AppService) -> PurchaseCategory {
        let command = CreatePurchaseCategoryCommand {
            category: PurchaseCategoryNew {
                name: "Test Category".to_string(),
                description: None,
                state: None,
            },
        };
        command.exec(service).await.unwrap()
    }

    #[tokio::test]
    async fn test_create_expense() {
        let mut service = setup_service().await;

        // Create a category first
        let category = create_test_category(&mut service).await;

        // Create a cost center
        let cost_center = create_test_cost_center(&mut service).await;

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

        let expense = command.exec(&mut service).await.unwrap();
        assert_eq!(expense.title, "Test Expense");
        assert_eq!(expense.amount, 1000.into());
        assert_eq!(expense.category_id, category.id);
        assert_eq!(expense.cost_center_id, cost_center.id);
        assert_eq!(expense.description, Some("Test Description".to_string()));
    }

    #[tokio::test]
    async fn test_update_expense() {
        let mut service = setup_service().await;

        // Create a category first
        let category = create_test_category(&mut service).await;

        // Create a cost center
        let cost_center = create_test_cost_center(&mut service).await;

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

        let expense = create_command.exec(&mut service).await.unwrap();

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

        let updated_expense = update_command.exec(&mut service).await.unwrap();
        assert_eq!(updated_expense.title, "Updated Expense");
        assert_eq!(updated_expense.amount, 2000.into());
        assert_eq!(updated_expense.category_id, category.id);
        assert_eq!(updated_expense.cost_center_id, cost_center.id);
        assert_eq!(
            updated_expense.description,
            Some("Test Description".to_string())
        );
    }

    #[tokio::test]
    async fn test_delete_expense() {
        let mut service = setup_service().await;

        // Create a category first
        let category = create_test_category(&mut service).await;

        // Create a cost center
        let cost_center = create_test_cost_center(&mut service).await;

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

        let expense = create_command.exec(&mut service).await.unwrap();

        // Delete the expense
        let delete_command = DeleteExpenseCommand { id: expense.id };
        let result = delete_command.exec(&mut service).await.unwrap();
        assert_eq!(result, 1);

        // Verify expense no longer exists
        let mut count_query = Query::select();
        let count_stmt = count_query
            .from(Expenses::Table)
            .expr_as(Expr::col(Expenses::Id).count(), Alias::new("count"))
            .and_where(Expr::col(Expenses::Id).eq(expense.id.to_string()));

        let count = service.db_adapter.query_one::<i64>(&count_stmt).await.unwrap();
        assert_eq!(count, 0);
    }

    #[tokio::test]
    async fn test_update_expense_cost_center() {
        let mut service = setup_service().await;

        // Create a category
        let category = create_test_category(&mut service).await;

        // Create two cost centers
        let cost_center1 = create_test_cost_center(&mut service).await;
        let cost_center2 =
            create_test_cost_center_with_name(&mut service, "Second Cost Center", "CC002").await;

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

        let expense = create_command.exec(&mut service).await.unwrap();
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

        let updated_expense = update_command.exec(&mut service).await.unwrap();

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

    async fn create_test_cost_center(service: &mut AppService) -> CostCenter {
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
        command.exec(service).await.unwrap()
    }

    async fn create_test_cost_center_with_name(
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
        command.exec(service).await.unwrap()
    }
}
