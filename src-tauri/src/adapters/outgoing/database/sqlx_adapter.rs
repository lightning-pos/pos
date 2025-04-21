use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;

use sea_query::{SelectStatement, InsertStatement, UpdateStatement, DeleteStatement, SqliteQueryBuilder};

use crate::{
    adapters::outgoing::database::{DatabaseAdapter, SqlParam},
    error::{Error, Result},
};

pub struct SqlxAdapter {
    conn: Arc<Mutex<libsql::Connection>>,
}

impl SqlxAdapter {
    pub fn new(conn: libsql::Connection) -> Self {
        Self {
            conn: Arc::new(Mutex::new(conn))
        }
    }
}

// Helper method to convert SqlParam to libsql::Value
fn _convert_params(params: Vec<SqlParam>) -> Vec<libsql::Value> {
    params.into_iter().map(|param| {
        match param {
            SqlParam::String(s) => libsql::Value::Text(s),
            SqlParam::Integer(i) => libsql::Value::Integer(i),
            SqlParam::Float(f) => libsql::Value::Real(f),
            SqlParam::Boolean(b) => libsql::Value::Integer(if b { 1 } else { 0 }),
            SqlParam::Null => libsql::Value::Null,
            SqlParam::Uuid(uuid) => libsql::Value::Text(uuid.to_string()),
        }
    }).collect()
}

impl DatabaseAdapter for SqlxAdapter {
    async fn query_one<T>(&self, query: &SelectStatement) -> Result<T> {
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
            Some(_row) => {
                // For now, we'll return a placeholder error
                // In a real implementation, we would deserialize the row into T
                Err(Error::DatabaseError("Deserialization not implemented yet".to_string()))
            },
            None => Err(Error::NotFoundError),
        }
    }

    async fn query_optional<T>(&self, query: &SelectStatement) -> Result<Option<T>> {
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
            Some(_) => {
                // For now, we'll return a placeholder error
                // In a real implementation, we would deserialize the row into T
                Err(Error::DatabaseError("Deserialization not implemented yet".to_string()))
            },
            None => Ok(None),
        }
    }

    async fn query_many<T>(&self, query: &SelectStatement) -> Result<Vec<T>> {
        // Convert the SelectStatement to SQL string
        let sql = query.to_string(SqliteQueryBuilder);

        // Get a lock on the connection
        let conn = self.conn.lock().await;

        // Execute the query
        let mut stmt = conn.prepare(&sql).await
            .map_err(|e| Error::DatabaseError(format!("Failed to prepare statement: {}", e)))?;

        // Execute the statement
        let _rows = stmt.query(Vec::<libsql::Value>::new()).await
            .map_err(|e| Error::DatabaseError(format!("Failed to execute query: {}", e)))?;

        // For now, we'll return a placeholder error
        // In a real implementation, we would collect all rows and deserialize them into Vec<T>
        Err(Error::DatabaseError("Deserialization not implemented yet".to_string()))
    }

    async fn execute(&self, query: &str) -> Result<u64> {
        // Get a lock on the connection
        let conn = self.conn.lock().await;

        // Execute the query
        let mut stmt = conn.prepare(query).await
            .map_err(|e| Error::DatabaseError(format!("Failed to prepare statement: {}", e)))?;

        // Execute the statement
        let result = stmt.execute(Vec::<libsql::Value>::new()).await
            .map_err(|e| Error::DatabaseError(format!("Failed to execute statement: {}", e)))?;

        // Return the number of affected rows
        Ok(result as u64)
    }

    async fn insert_one<T>(&self, query: &InsertStatement) -> Result<T> {
        // Convert the InsertStatement to SQL string
        let sql = query.to_string(SqliteQueryBuilder);

        // Execute the query
        self.execute(&sql).await?;

        // For now, we'll return a placeholder error
        // In a real implementation, we would fetch the inserted row and deserialize it into T
        Err(Error::DatabaseError("Deserialization not implemented yet".to_string()))
    }

    async fn insert_many(&self, query: &InsertStatement) -> Result<u64> {
        // Convert the InsertStatement to SQL string
        let sql = query.to_string(SqliteQueryBuilder);

        // Execute the query and return the number of affected rows
        self.execute(&sql).await
    }

    async fn update_one<T>(&self, query: &UpdateStatement) -> Result<T> {
        // Convert the UpdateStatement to SQL string
        let sql = query.to_string(SqliteQueryBuilder);

        // Execute the query
        let affected = self.execute(&sql).await?;

        if affected == 0 {
            return Err(Error::NotFoundError);
        }

        // For now, we'll return a placeholder error
        // In a real implementation, we would fetch the updated row and deserialize it into T
        Err(Error::DatabaseError("Deserialization not implemented yet".to_string()))
    }

    async fn update_many(&self, query: &UpdateStatement) -> Result<u64> {
        // Convert the UpdateStatement to SQL string
        let sql = query.to_string(SqliteQueryBuilder);

        // Execute the query and return the number of affected rows
        self.execute(&sql).await
    }

    async fn upsert<T>(&self, query: &InsertStatement) -> Result<T> {
        // Convert the InsertStatement to SQL string with ON CONFLICT clause
        let sql = query.to_string(SqliteQueryBuilder);

        // Execute the query
        self.execute(&sql).await?;

        // For now, we'll return a placeholder error
        // In a real implementation, we would fetch the upserted row and deserialize it into T
        Err(Error::DatabaseError("Deserialization not implemented yet".to_string()))
    }

    async fn delete(&self, query: &DeleteStatement) -> Result<u64> {
        // Convert the DeleteStatement to SQL string
        let sql = query.to_string(SqliteQueryBuilder);

        // Execute the query and return the number of affected rows
        self.execute(&sql).await
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
}
