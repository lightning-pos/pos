use chrono::NaiveDateTime;
use juniper::graphql_object;

use crate::{
    core::{
        models::purchases::expense_model::Expense,
        types::{db_uuid::DbUuid, money::Money},
    },
    AppState,
};

#[graphql_object(context = AppState)]
impl Expense {
    pub fn id(&self) -> DbUuid {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn amount(&self) -> Money {
        self.amount
    }

    pub fn expense_date(&self) -> NaiveDateTime {
        self.expense_date
    }

    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }
}
