use modql::filter::{FilterGroups, ListOptions};
use std::io::Result;

pub trait SQLEntity {
    const TABLE_NAME: &'static str;
}

#[mockall::automock]
pub trait SQLInterface {
    fn get_one<T: SQLEntity + 'static>(
        &self,
        filters: Option<FilterGroups>,
        options: Option<ListOptions>,
    ) -> Option<T>;

    fn get_many<T: SQLEntity + 'static>(
        &self,
        filters: Option<FilterGroups>,
        options: Option<ListOptions>,
    ) -> Vec<T>;

    fn save<T: SQLEntity + 'static>(&self, entity: &T) -> Result<T>;

    fn delete<T: SQLEntity + 'static>(&self, entity: &T) -> Result<bool>;
}
