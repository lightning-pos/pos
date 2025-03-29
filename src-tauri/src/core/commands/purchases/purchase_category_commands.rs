use chrono::Utc;
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{
    core::{
        commands::{app_service::AppService, Command},
        models::purchases::purchase_category_model::{
            PurchaseCategory, PurchaseCategoryNew, PurchaseCategoryState, PurchaseCategoryUpdate,
        },
        types::db_uuid::DbUuid,
    },
    error::{Error, Result},
    schema::purchase_categories::dsl::*,
};

// Commands
pub struct CreatePurchaseCategoryCommand {
    pub category: PurchaseCategoryNew,
}

pub struct UpdatePurchaseCategoryCommand {
    pub category: PurchaseCategoryUpdate,
}

pub struct DeletePurchaseCategoryCommand {
    pub id: DbUuid,
}

// Command Implementations
impl Command for CreatePurchaseCategoryCommand {
    type Output = PurchaseCategory;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let existing_cat = purchase_categories
                .filter(name.eq(&self.category.name))
                .select(PurchaseCategory::as_select())
                .get_result::<PurchaseCategory>(conn);

            if let Ok(_) = existing_cat {
                return Err(Error::UniqueConstraintError);
            }

            let now = Utc::now().naive_utc();
            let new_cat = PurchaseCategory {
                id: Uuid::now_v7().into(),
                name: self.category.name.clone(),
                description: self.category.description.clone(),
                state: PurchaseCategoryState::Inactive,
                created_at: now,
                updated_at: now,
            };

            let cat = diesel::insert_into(purchase_categories)
                .values(&new_cat)
                .returning(PurchaseCategory::as_returning())
                .get_result(conn)?;

            Ok(cat)
        })
    }
}

impl Command for UpdatePurchaseCategoryCommand {
    type Output = PurchaseCategory;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            purchase_categories
                .filter(id.eq(&self.category.id))
                .limit(1)
                .select(PurchaseCategory::as_select())
                .get_result::<PurchaseCategory>(conn)?;

            let now = Utc::now().naive_utc();

            let mut category = self.category.clone();
            category.updated_at = Some(now);

            let cat = diesel::update(purchase_categories.filter(id.eq(&self.category.id)))
                .set(&category)
                .returning(PurchaseCategory::as_returning())
                .get_result(conn)?;

            Ok(cat)
        })
    }
}

impl Command for DeletePurchaseCategoryCommand {
    type Output = i32;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Just check if the category exists
            purchase_categories
                .filter(id.eq(&self.id))
                .limit(1)
                .select(PurchaseCategory::as_select())
                .get_result::<PurchaseCategory>(conn)?;

            let res = diesel::delete(purchase_categories.filter(id.eq(&self.id))).execute(conn)?;

            Ok(res as i32)
        })
    }
}
