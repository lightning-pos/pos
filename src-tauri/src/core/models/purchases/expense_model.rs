use chrono::NaiveDateTime;
use diesel::{
    prelude::{AsChangeset, Insertable, Queryable},
    Selectable,
};
use juniper::GraphQLInputObject;

use crate::core::types::{db_uuid::DbUuid, money::Money};
use crate::schema::expenses;

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = expenses)]
pub struct Expense {
    pub id: DbUuid,
    pub title: String,
    pub amount: Money,
    pub expense_date: NaiveDateTime,
    pub category: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ExpenseNewInput {
    pub title: String,
    pub amount: Money,
    pub expense_date: NaiveDateTime,
    pub category: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ExpenseUpdateInput {
    pub id: DbUuid,
    pub title: Option<String>,
    pub amount: Option<Money>,
    pub expense_date: Option<NaiveDateTime>,
    pub category: Option<String>,
    pub description: Option<Option<String>>,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = expenses)]
pub struct ExpenseUpdateChangeset {
    pub id: DbUuid,
    pub title: Option<String>,
    pub amount: Option<Money>,
    pub expense_date: Option<NaiveDateTime>,
    pub category: Option<String>,
    pub description: Option<Option<String>>,
    pub updated_at: NaiveDateTime,
}
