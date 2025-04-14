use std::sync::{Arc, Mutex};

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
fn convert_params(params: Vec<SqlParam>) -> Vec<libsql::Value> {
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
    fn query_one<T>(&self, query: &str, params: Vec<SqlParam>) -> Result<T>
    where
        T: Send + Sync,
    {
        // Create a runtime to execute async code in a sync function
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| Error::DatabaseError(format!("Failed to create runtime: {}", e)))?;

        // Convert SqlParam to libsql::Value
        let libsql_params = convert_params(params);

        // Get a lock on the connection
        let conn = self.conn.lock()
            .map_err(|e| Error::DatabaseError(format!("Failed to lock connection: {}", e)))?;

        // Execute the query
        let mut stmt = rt.block_on(async {
            conn.prepare(query).await
                .map_err(|e| Error::DatabaseError(format!("Failed to prepare statement: {}", e)))
        })?;

        // Execute the statement
        let mut rows = rt.block_on(async {
            stmt.query(libsql_params).await
                .map_err(|e| Error::DatabaseError(format!("Failed to execute query: {}", e)))
        })?;

        // Get the first row
        let row = rt.block_on(async {
            rows.next().await
                .map_err(|e| Error::DatabaseError(format!("Failed to get next row: {}", e)))
        })?;

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

    fn query_optional<T>(&self, query: &str, params: Vec<SqlParam>) -> Result<Option<T>>
    where
        T: Send + Sync,
    {
        // Create a runtime to execute async code in a sync function
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| Error::DatabaseError(format!("Failed to create runtime: {}", e)))?;

        // Convert SqlParam to libsql::Value
        let libsql_params = convert_params(params);

        // Get a lock on the connection
        let conn = self.conn.lock()
            .map_err(|e| Error::DatabaseError(format!("Failed to lock connection: {}", e)))?;

        // Execute the query
        let mut stmt = rt.block_on(async {
            conn.prepare(query).await
                .map_err(|e| Error::DatabaseError(format!("Failed to prepare statement: {}", e)))
        })?;

        // Execute the statement
        let mut rows = rt.block_on(async {
            stmt.query(libsql_params).await
                .map_err(|e| Error::DatabaseError(format!("Failed to execute query: {}", e)))
        })?;

        // Get the first row
        let row = rt.block_on(async {
            rows.next().await
                .map_err(|e| Error::DatabaseError(format!("Failed to get next row: {}", e)))
        })?;

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

    fn query_many<T>(&self, query: &str, params: Vec<SqlParam>) -> Result<Vec<T>>
    where
        T: Send + Sync,
    {
        // Create a runtime to execute async code in a sync function
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| Error::DatabaseError(format!("Failed to create runtime: {}", e)))?;

        // Convert SqlParam to libsql::Value
        let libsql_params = convert_params(params);

        // Get a lock on the connection
        let conn = self.conn.lock()
            .map_err(|e| Error::DatabaseError(format!("Failed to lock connection: {}", e)))?;

        // Execute the query
        let mut stmt = rt.block_on(async {
            conn.prepare(query).await
                .map_err(|e| Error::DatabaseError(format!("Failed to prepare statement: {}", e)))
        })?;

        // Execute the statement
        let _rows = rt.block_on(async {
            stmt.query(libsql_params).await
                .map_err(|e| Error::DatabaseError(format!("Failed to execute query: {}", e)))
        })?;

        // For now, we'll return a placeholder error
        // In a real implementation, we would collect all rows and deserialize them into Vec<T>
        Err(Error::DatabaseError("Deserialization not implemented yet".to_string()))
    }

    fn execute(&self, query: &str, params: Vec<SqlParam>) -> Result<u64> {
        // Create a runtime to execute async code in a sync function
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| Error::DatabaseError(format!("Failed to create runtime: {}", e)))?;

        // Convert SqlParam to libsql::Value
        let libsql_params = convert_params(params);

        // Get a lock on the connection
        let conn = self.conn.lock()
            .map_err(|e| Error::DatabaseError(format!("Failed to lock connection: {}", e)))?;

        // Execute the query
        let mut stmt = rt.block_on(async {
            conn.prepare(query).await
                .map_err(|e| Error::DatabaseError(format!("Failed to prepare statement: {}", e)))
        })?;

        // Execute the statement
        let result = rt.block_on(async {
            stmt.execute(libsql_params).await
                .map_err(|e| Error::DatabaseError(format!("Failed to execute statement: {}", e)))
        })?;

        // Return the number of affected rows
        Ok(result as u64)
    }

    fn transaction<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&Self) -> Result<R> + Send + Sync,
        R: Send + Sync,
    {
        // Create a runtime to execute async code in a sync function
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| Error::DatabaseError(format!("Failed to create runtime: {}", e)))?;

        // Get a lock on the connection
        let conn = self.conn.lock()
            .map_err(|e| Error::DatabaseError(format!("Failed to lock connection: {}", e)))?;

        // Begin transaction
        rt.block_on(async {
            conn.execute("BEGIN TRANSACTION", Vec::<libsql::Value>::new()).await
                .map_err(|e| Error::DatabaseError(format!("Failed to begin transaction: {}", e)))
        })?;

        // Execute the function
        let result = f(self);

        // Commit or rollback transaction based on the result
        match &result {
            Ok(_) => {
                rt.block_on(async {
                    conn.execute("COMMIT", Vec::<libsql::Value>::new()).await
                        .map_err(|e| Error::DatabaseError(format!("Failed to commit transaction: {}", e)))
                })?;
            },
            Err(_) => {
                rt.block_on(async {
                    conn.execute("ROLLBACK", Vec::<libsql::Value>::new()).await
                        .map_err(|e| Error::DatabaseError(format!("Failed to rollback transaction: {}", e)))
                })?;
            },
        }

        result
    }
}
