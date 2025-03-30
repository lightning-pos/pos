use chrono::NaiveDateTime;
use juniper::graphql_object;
use juniper::FieldResult;

use crate::{
    core::{
        models::{
            finance::cost_center_model::CostCenter,
            purchases::{expense_model::Expense, purchase_category_model::PurchaseCategory},
        },
        types::{db_uuid::DbUuid, money::Money},
    },
    schema::{cost_centers, purchase_categories},
    AppState,
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

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

    pub fn category_id(&self) -> DbUuid {
        self.category_id
    }

    pub fn cost_center_id(&self) -> DbUuid {
        self.cost_center_id
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

    pub fn category(&self, context: &AppState) -> FieldResult<PurchaseCategory> {
        let mut service = context.service.lock().unwrap();
        let category = purchase_categories::table
            .filter(purchase_categories::id.eq(&self.category_id))
            .first(&mut service.conn)?;

        Ok(category)
    }

    pub fn cost_center(&self, context: &AppState) -> FieldResult<CostCenter> {
        let mut service = context.service.lock().unwrap();
        let cost_center = cost_centers::table
            .filter(cost_centers::id.eq(&self.cost_center_id))
            .first(&mut service.conn)?;

        Ok(cost_center)
    }
}
