use crate::error::Result;
use modql::{
    field::HasSeaFields,
    filter::{FilterGroups, ListOptions},
    SIden,
};
use sea_query::{IntoIden, TableRef};
use sqlx::{sqlite::SqliteRow, FromRow};

pub trait SQLEntity {
    const TABLE_NAME: &'static str;

    fn table_ref() -> TableRef {
        TableRef::Table(SIden(Self::TABLE_NAME).into_iden())
    }

    fn id(&self) -> String;
}

pub trait SQLInterface {
    async fn get_one<T: SQLEntity + HasSeaFields + for<'r> FromRow<'r, SqliteRow> + Send + Unpin>(
        &self,
        filters: Option<FilterGroups>,
        options: Option<ListOptions>,
    ) -> Result<Option<T>>;

    async fn get_many<T: SQLEntity + HasSeaFields + for<'r> FromRow<'r, SqliteRow> + Send + Unpin>(
        &self,
        filters: Option<FilterGroups>,
        options: Option<ListOptions>,
    ) -> Result<Vec<T>>;

    async fn save<T: SQLEntity + HasSeaFields + for<'r> FromRow<'r, SqliteRow> + Send + Unpin>(
        &self,
        entity: T,
    ) -> Result<T>;

    async fn delete<T: SQLEntity + HasSeaFields + for<'r> FromRow<'r, SqliteRow> + Send + Unpin>(
        &self,
        entity: T,
    ) -> Result<bool>;
}
