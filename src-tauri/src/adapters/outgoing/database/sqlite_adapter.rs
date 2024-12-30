use std::io::Result;

use crate::core::common::interface::sql::{SQLEntity, SQLInterface};
use modql::filter::{FilterGroups, ListOptions};

pub struct SQLiteAdapter;

impl SQLInterface for SQLiteAdapter {
    fn get_one<T: SQLEntity + 'static>(
        &self,
        filters: Option<FilterGroups>,
        options: Option<ListOptions>,
    ) -> Option<T> {
        todo!()
    }

    fn get_many<T: SQLEntity + 'static>(
        &self,
        filters: Option<FilterGroups>,
        options: Option<ListOptions>,
    ) -> Vec<T> {
        todo!()
    }

    fn save<T: SQLEntity + 'static>(&self, entity: &T) -> Result<T> {
        todo!()
    }

    fn delete<T: SQLEntity + 'static>(&self, entity: &T) -> Result<bool> {
        todo!()
    }
}
