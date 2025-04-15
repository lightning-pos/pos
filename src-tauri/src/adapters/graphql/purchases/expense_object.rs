use chrono::NaiveDateTime;
use sea_query::{Expr, Query, SqliteQueryBuilder};
use juniper::{graphql_object, FieldResult};

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::{
            finance::cost_center_model::{CostCenter, CostCenters},
            purchases::{
                expense_model::Expense,
                purchase_category_model::{PurchaseCategory, PurchaseCategories},
            },
        },
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
        let service = context.service.lock().unwrap();
        
        let query = Query::select()
            .from(PurchaseCategories::Table)
            .columns([
                PurchaseCategories::Id,
                PurchaseCategories::Name,
                PurchaseCategories::Description,
                PurchaseCategories::State,
                PurchaseCategories::CreatedAt,
                PurchaseCategories::UpdatedAt,
            ])
            .and_where(Expr::col(PurchaseCategories::Id).eq(self.category_id.to_string()))
            .to_string(SqliteQueryBuilder);
            
        let category = service.db_adapter.query_one::<PurchaseCategory>(&query, vec![])?;

        Ok(category)
    }

    pub fn cost_center(&self, context: &AppState) -> FieldResult<CostCenter> {
        let service = context.service.lock().unwrap();
        
        let query = Query::select()
            .from(CostCenters::Table)
            .columns([
                CostCenters::Id,
                CostCenters::Name,
                CostCenters::Code,
                CostCenters::Description,
                CostCenters::State,
                CostCenters::CreatedAt,
                CostCenters::UpdatedAt,
            ])
            .and_where(Expr::col(CostCenters::Id).eq(self.cost_center_id.to_string()))
            .to_string(SqliteQueryBuilder);
            
        let cost_center = service.db_adapter.query_one::<CostCenter>(&query, vec![])?;

        Ok(cost_center)
    }
}