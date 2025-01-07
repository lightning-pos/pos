use std::str::FromStr;

use crate::core::common::interface::sql::{SQLEntity, SQLInterface};
use crate::error::Result;
use modql::field::{HasFields, HasSeaFields};
use modql::filter::{FilterGroups, ListOptions};
use modql::SIden;
use sea_query::{Condition, Expr, OnConflict, Query, SqliteQueryBuilder};
use sqlx::sqlite::{SqliteConnectOptions, SqliteRow};
use sqlx::{FromRow, SqlitePool};

pub struct SQLiteAdapter {
    pool: SqlitePool,
}

impl SQLiteAdapter {
    pub async fn new(db: &str) -> Result<Self> {
        let pool = SqlitePool::connect(db).await?;
        Self::initialize_schema(&pool).await?;
        Ok(Self { pool })
    }

    pub async fn initialize_schema(pool: &SqlitePool) -> Result<()> {
        // Create item_category table
        let item_category = String::from(
            "CREATE TABLE IF NOT EXISTS item_categories (
            id TEXT NOT NULL PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            state TEXT NOT NULL,
            description TEXT,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        )",
        );

        sqlx::query(&item_category).execute(pool).await?;

        // Create item table
        let item = String::from(
            "CREATE TABLE IF NOT EXISTS items (
            id TEXT NOT NULL PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            nature TEXT NOT NULL,
            category_id TEXT NOT NULL,
            state TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (category_id) REFERENCES item_categories (id)
        )",
        );

        sqlx::query(&item).execute(pool).await?;

        Ok(())
    }
}

impl SQLInterface for SQLiteAdapter {
    async fn get_one<
        T: SQLEntity + HasSeaFields + for<'r> FromRow<'r, SqliteRow> + Send + Unpin,
    >(
        &self,
        filters: Option<FilterGroups>,
        options: Option<ListOptions>,
    ) -> Result<Option<T>> {
        let mut query = Query::select();

        query.from(T::table_ref()).columns(T::sea_column_refs());

        if let Some(filters) = filters {
            let cond: Condition = filters.try_into()?;
            query.cond_where(cond);
        }

        if let Some(options) = options {
            options.apply_to_sea_query(&mut query);
        }

        let sql = query.to_string(SqliteQueryBuilder);

        let result = sqlx::query_as(&sql).fetch_optional(&self.pool).await?;
        Ok(result)
    }

    async fn get_many<
        T: SQLEntity + HasSeaFields + for<'r> FromRow<'r, SqliteRow> + Send + Unpin,
    >(
        &self,
        filters: Option<FilterGroups>,
        options: Option<ListOptions>,
    ) -> Result<Vec<T>> {
        let mut query = Query::select();

        query.from(T::table_ref()).columns(T::sea_column_refs());

        if let Some(filters) = filters {
            let cond: Condition = filters.try_into()?;
            query.cond_where(cond);
        }

        if let Some(options) = options {
            options.apply_to_sea_query(&mut query);
        }

        let sql = query.to_string(SqliteQueryBuilder);

        Ok(sqlx::query_as(&sql).fetch_all(&self.pool).await?)
    }

    async fn save<T: SQLEntity + HasSeaFields + for<'r> FromRow<'r, SqliteRow> + Send + Unpin>(
        &self,
        entity: T,
    ) -> Result<T> {
        let fields = entity.all_sea_fields();
        let (columns, values) = fields.for_sea_insert();
        let query = Query::insert()
            .into_table(T::table_ref())
            .columns(columns.clone())
            .values(values)?
            .on_conflict(
                OnConflict::column(SIden("id"))
                    .update_columns(columns)
                    .to_owned(),
            )
            .returning(Query::returning().all())
            .to_string(SqliteQueryBuilder);

        Ok(sqlx::query_as(&query).fetch_one(&self.pool).await?)
    }

    async fn delete<T: SQLEntity + HasSeaFields + for<'r> FromRow<'r, SqliteRow> + Send + Unpin>(
        &self,
        entity: T,
    ) -> Result<bool> {
        let sql = Query::delete()
            .from_table(T::table_ref())
            .and_where(Expr::col(SIden("id")).eq(entity.id()))
            .to_string(SqliteQueryBuilder);

        let count = sqlx::query(&sql).execute(&self.pool).await?.rows_affected();

        Ok(count > 0)
    }
}
