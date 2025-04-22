use chrono::NaiveDateTime;
use juniper::GraphQLInputObject;
use sea_query::Iden;

use crate::core::types::{db_uuid::DbUuid, money::Money};

#[derive(Debug)]
pub struct Expense {
    pub id: DbUuid,
    pub title: String,
    pub amount: Money,
    pub expense_date: NaiveDateTime,
    pub category_id: DbUuid,
    pub cost_center_id: DbUuid,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ExpenseNewInput {
    pub title: String,
    pub amount: Money,
    pub expense_date: NaiveDateTime,
    pub category_id: DbUuid,
    pub cost_center_id: DbUuid,
    pub description: Option<String>,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ExpenseUpdateInput {
    pub id: DbUuid,
    pub title: Option<String>,
    pub amount: Option<Money>,
    pub expense_date: Option<NaiveDateTime>,
    pub category_id: Option<DbUuid>,
    pub cost_center_id: Option<DbUuid>,
    pub description: Option<Option<String>>,
}

// Define table and column identifiers for SeaQuery
#[derive(Iden)]
pub enum Expenses {
    Table,
    Id,
    Title,
    Amount,
    ExpenseDate,
    CategoryId,
    CostCenterId,
    Description,
    CreatedAt,
    UpdatedAt,
}