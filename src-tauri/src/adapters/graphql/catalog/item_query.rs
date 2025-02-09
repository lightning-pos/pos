use chrono::NaiveDateTime;
use diesel::{
    query_dsl::methods::{FilterDsl, FindDsl, SelectDsl},
    ExpressionMethods, RunQueryDsl, SelectableHelper,
};
use juniper::{graphql_object, FieldResult};

use crate::{
    core::{
        models::{
            catalog::{
                item_group_model::ItemGroup,
                item_model::{Item, ItemNature, ItemState},
            },
            common::tax_model::Tax,
        },
        types::{db_uuid::DbUuid, money::Money},
    },
    schema::{item_categories, item_taxes, taxes},
    AppState,
};

#[graphql_object(context = AppState)]
impl Item {
    pub fn id(&self) -> DbUuid {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn nature(&self) -> ItemNature {
        self.nature
    }

    pub fn state(&self) -> ItemState {
        self.state
    }

    pub fn price(&self) -> Money {
        self.price
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }

    pub fn category(&self, context: &AppState) -> FieldResult<ItemGroup> {
        let mut service = context.service.lock().unwrap();
        let res = item_categories::table
            .find(&self.category_id)
            .select(ItemGroup::as_select())
            .get_result(&mut service.conn)?;
        Ok(res)
    }

    pub fn taxes(&self, context: &AppState) -> FieldResult<Vec<Tax>> {
        let mut service = context.service.lock().unwrap();

        let tax_ids = item_taxes::table
            .filter(item_taxes::item_id.eq(self.id))
            .select(item_taxes::tax_id)
            .load::<DbUuid>(&mut service.conn)?;

        let taxes = taxes::table
            .filter(taxes::id.eq_any(tax_ids))
            .load::<Tax>(&mut service.conn)?;

        Ok(taxes)
    }
}
