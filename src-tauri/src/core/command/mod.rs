pub mod app_service;
pub mod catalog;

use crate::{
    core::entities::catalog::item_category::ItemCategory,
    error::{Error, Result},
    schema::item_categories::dsl::*,
};
use app_service::AppService;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

pub trait Command {
    fn exec(&self, service: &mut AppService) -> Result<()>;
}

pub struct AddItemCatCommand {
    pub cat: ItemCategory,
}

impl Command for AddItemCatCommand {
    fn exec(&self, service: &mut AppService) -> Result<()> {
        let existing_cat = item_categories
            .filter(name.eq(&self.cat.name))
            .select(ItemCategory::as_select())
            .get_result::<ItemCategory>(&mut service.conn);

        if let Ok(_) = existing_cat {
            return Err(Error::UniqueConstraintError);
        }

        diesel::insert_into(item_categories)
            .values(&self.cat)
            .execute(&mut service.conn)?;

        Ok(())
    }
}
