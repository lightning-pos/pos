use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;

use sea_query::{SelectStatement, InsertStatement, UpdateStatement, DeleteStatement, SqliteQueryBuilder};

use crate::{
    adapters::outgoing::database::{FromRow, DatabaseAdapter, DatabaseRow},
    error::{Error, Result},
};

// Implement DatabaseRow for libsql::Row
impl DatabaseRow for libsql::Row {}

pub trait FromLibsqlValue: Sized + Send {
    fn from_libsql_value(value: libsql::Value) -> Result<Option<Self>>;
}

/// LibSQLAdapter implements the DatabaseAdapter trait for LibSQL
pub struct LibSqlAdapter {
    db: libsql::Database,
    conn: Arc<Mutex<libsql::Connection>>,
}

impl LibSqlAdapter {
    /// Create a new LibSQLAdapter with the given connection
    pub fn new(db: libsql::Database, conn: libsql::Connection) -> Self {
        Self {
            db,
            conn: Arc::new(Mutex::new(conn))
        }
    }

    /// Extract parameters from a query for binding
    async fn extract_params(&self, _query: &str) -> Result<Vec<libsql::Value>> {
        // For now, we don't extract parameters from the query
        // In a real implementation, you would parse the query and extract parameters
        Ok(Vec::new())
    }
}

impl DatabaseAdapter for LibSqlAdapter {
    type Row = libsql::Row;
    async fn query_one<T>(&self, query: &SelectStatement) -> Result<T>
    where
        T: FromRow<Self::Row> + Send,
    {
        // Convert the SelectStatement to SQL string
        let sql = query.to_string(SqliteQueryBuilder);

        // Get a lock on the connection
        let conn = self.conn.lock().await;

        // Execute the query
        let mut stmt = conn.prepare(&sql).await
            .map_err(|e| Error::DatabaseError(format!("Failed to prepare statement: {}", e)))?;

        // Execute the statement
        let mut rows = stmt.query(Vec::<libsql::Value>::new()).await
            .map_err(|e| Error::DatabaseError(format!("Failed to execute query: {}", e)))?;

        // Get the first row
        let row = rows.next().await
            .map_err(|e| Error::DatabaseError(format!("Failed to get next row: {}", e)))?;

        // Check if we got a row
        match row {
            Some(row) => {
                // Convert the row to the model type using the FromRow trait
                T::from_row(&row)
            },
            None => Err(Error::NotFoundError),
        }
    }

    async fn query_optional<T>(&self, query: &SelectStatement) -> Result<Option<T>>
    where
        T: FromRow<Self::Row> + Send,
    {
        // Convert the SelectStatement to SQL string
        let sql = query.to_string(SqliteQueryBuilder);

        // Get a lock on the connection
        let conn = self.conn.lock().await;

        // Execute the query
        let mut stmt = conn.prepare(&sql).await
            .map_err(|e| Error::DatabaseError(format!("Failed to prepare statement: {}", e)))?;

        // Execute the statement
        let mut rows = stmt.query(Vec::<libsql::Value>::new()).await
            .map_err(|e| Error::DatabaseError(format!("Failed to execute query: {}", e)))?;

        // Get the first row
        let row = rows.next().await
            .map_err(|e| Error::DatabaseError(format!("Failed to get next row: {}", e)))?;

        // Check if we got a row
        match row {
            Some(row) => {
                // Convert the row to the model type using the FromRow trait
                let result = T::from_row(&row)?;
                Ok(Some(result))
            },
            None => Ok(None),
        }
    }

    async fn query_many<T>(&self, query: &SelectStatement) -> Result<Vec<T>>
    where
        T: FromRow<Self::Row> + Send,
    {
        // Convert the SelectStatement to SQL string
        let sql = query.to_string(SqliteQueryBuilder);

        // Get a lock on the connection
        let conn = self.conn.lock().await;

        // Execute the query
        let mut stmt = conn.prepare(&sql).await
            .map_err(|e| Error::DatabaseError(format!("Failed to prepare statement: {}", e)))?;

        // Execute the statement
        let mut rows = stmt.query(Vec::<libsql::Value>::new()).await
            .map_err(|e| Error::DatabaseError(format!("Failed to execute query: {}", e)))?;

        // Collect all rows
        let mut results = Vec::new();
        while let Some(row) = rows.next().await
            .map_err(|e| Error::DatabaseError(format!("Failed to get next row: {}", e)))? {

            // Convert the row to the model type using the FromRow trait
            let item = T::from_row(&row)?;
            results.push(item);
        }

        Ok(results)
    }

    async fn insert_one<T>(&self, query: &InsertStatement) -> Result<T>
    where
        T: FromRow<Self::Row> + Send,
    {
        // Convert the InsertStatement to SQL string
        let mut sql = query.to_string(SqliteQueryBuilder);

        // Modify the SQL to add a RETURNING clause
        // SQLite supports RETURNING since version 3.35.0
        // This ensures we get columns in a predictable order
        sql = format!("{} RETURNING *", sql);

        // Get a lock on the connection
        let conn = self.conn.lock().await;

        // Execute the query with RETURNING clause
        let mut stmt = conn.prepare(&sql).await
            .map_err(|e| Error::DatabaseError(format!("Failed to prepare statement: {}", e)))?;

        let mut rows = stmt.query(()).await
            .map_err(|e| Error::DatabaseError(format!("Failed to execute query: {}", e)))?;

        let sync_res = self.db.sync().await;
        if sync_res.is_err() {
            eprintln!("Failed to sync database: {}", sync_res.unwrap_err());
        }

        // Get the first row
        let row = rows.next().await
            .map_err(|e| Error::DatabaseError(format!("Failed to get next row: {}", e)))?;

        match row {
            Some(row) => {
                T::from_row(&row)
            },
            None => Err(Error::DatabaseError("Failed to retrieve inserted row".to_string())),
        }
    }

    async fn insert_many(&self, query: &InsertStatement) -> Result<u64> {
        // Convert the InsertStatement to SQL string
        let sql = query.to_string(SqliteQueryBuilder);

        // Execute the query and return the number of affected rows
        let result = self.execute(&sql).await;

        let sync_res = self.db.sync().await;
        if sync_res.is_err() {
            eprintln!("Failed to sync database: {}", sync_res.unwrap_err());
        }

        result
    }

    async fn update_one<T>(&self, query: &UpdateStatement) -> Result<T>
    where
        T: FromRow<Self::Row> + Send,
    {
        // Convert the UpdateStatement to SQL string
        let mut sql = query.to_string(SqliteQueryBuilder);

        // Modify the SQL to add a RETURNING clause
        // SQLite supports RETURNING since version 3.35.0
        // This ensures we get columns in a predictable order
        sql = format!("{} RETURNING *", sql);

        // Get a lock on the connection
        let conn = self.conn.lock().await;

        // Execute the query with RETURNING clause
        let mut stmt = conn.prepare(&sql).await
            .map_err(|e| Error::DatabaseError(format!("Failed to prepare statement: {}", e)))?;

        let mut rows = stmt.query(()).await
            .map_err(|e| Error::DatabaseError(format!("Failed to execute query: {}", e)))?;

        // Get the first row
        let row = rows.next().await
            .map_err(|e| Error::DatabaseError(format!("Failed to get next row: {}", e)))?;

        // Check if we got a row
        match row {
            Some(row) => {
                // Convert the row to the model type using the FromRow trait
                T::from_row(&row)
            },
            None => Err(Error::NotFoundError),
        }
    }

    async fn update_many(&self, query: &UpdateStatement) -> Result<u64> {
        // Convert the UpdateStatement to SQL string
        let sql = query.to_string(SqliteQueryBuilder);

        // Execute the query and return the number of affected rows
        let result = self.execute(&sql).await;

        let sync_res = self.db.sync().await;
        if sync_res.is_err() {
            eprintln!("Failed to sync database: {}", sync_res.unwrap_err());
        }

        result
    }

    async fn upsert<T>(&self, query: &InsertStatement) -> Result<T>
    where
        T: FromRow<Self::Row> + Send,
    {
        // Convert the InsertStatement to SQL string with ON CONFLICT clause
        let mut sql = query.to_string(SqliteQueryBuilder);

        // Modify the SQL to add a RETURNING clause
        // SQLite supports RETURNING since version 3.35.0
        // Since we're mapping columns by name in LibsqlFromRow, we can use RETURNING *
        sql = format!("{} RETURNING *", sql);

        // Get a lock on the connection
        let conn = self.conn.lock().await;

        // Execute the query with RETURNING clause
        let mut stmt = conn.prepare(&sql).await
            .map_err(|e| Error::DatabaseError(format!("Failed to prepare statement: {}", e)))?;

        let mut rows = stmt.query(()).await
            .map_err(|e| Error::DatabaseError(format!("Failed to execute query: {}", e)))?;

        let sync_res = self.db.sync().await;
        if sync_res.is_err() {
            eprintln!("Failed to sync database: {}", sync_res.unwrap_err());
        }

        // Get the first row
        let row = rows.next().await
            .map_err(|e| Error::DatabaseError(format!("Failed to get next row: {}", e)))?;

        // Check if we got a row
        match row {
            Some(row) => {
                T::from_row(&row)
            },
            None => Err(Error::DatabaseError("Failed to retrieve upserted row".to_string())),
        }
    }

    async fn delete(&self, query: &DeleteStatement) -> Result<u64> {
        // Convert the DeleteStatement to SQL string
        let sql = query.to_string(SqliteQueryBuilder);

        // Execute the query and return the number of affected rows
        let result = self.execute(&sql).await;

        let sync_res = self.db.sync().await;
        if sync_res.is_err() {
            eprintln!("Failed to sync database: {}", sync_res.unwrap_err());
        }

        result
    }

    async fn transaction<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&Self) -> Pin<Box<dyn Future<Output = Result<R>> + Send>> + Send,
        R: Send,
    {
        // Get a lock on the connection
        let conn = self.conn.lock().await;

        // Begin transaction
        conn.execute("BEGIN TRANSACTION", Vec::<libsql::Value>::new()).await
            .map_err(|e| Error::DatabaseError(format!("Failed to begin transaction: {}", e)))?;

        // Execute the function
        let future = f(self);
        let result = future.await;

        // Commit or rollback transaction based on the result
        match &result {
            Ok(_) => {
                conn.execute("COMMIT", Vec::<libsql::Value>::new()).await
                    .map_err(|e| Error::DatabaseError(format!("Failed to commit transaction: {}", e)))?;
            },
            Err(_) => {
                conn.execute("ROLLBACK", Vec::<libsql::Value>::new()).await
                    .map_err(|e| Error::DatabaseError(format!("Failed to rollback transaction: {}", e)))?;
            },
        }

        result
    }

    async fn execute(&self, query: &str) -> Result<u64> {
        // Get a lock on the connection
        let conn = self.conn.lock().await;

        // Extract parameters from the query
        let params = self.extract_params(query).await?;

        // Execute the query
        let result = conn.execute(query, params).await
            .map_err(|e| Error::DatabaseError(format!("Failed to execute statement: {}", e)))?;

        // Return the number of affected rows
        Ok(result)
    }
}
