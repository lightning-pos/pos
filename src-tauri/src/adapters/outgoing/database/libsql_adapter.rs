use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::Mutex;

use sea_query::{SelectStatement, InsertStatement, UpdateStatement, DeleteStatement, SqliteQueryBuilder};

use crate::{
    adapters::outgoing::database::{FromRow, DatabaseAdapter, DatabaseRow, SqlParam},
    error::{Error, Result},
};

// Implement DatabaseRow for libsql::Row
impl DatabaseRow for libsql::Row {}

pub trait FromValue: Sized + Send {
    fn from_libsql_value(value: &libsql::Value) -> Result<Self>;
}

/// LibSQLAdapter implements the DatabaseAdapter trait for LibSQL
pub struct LibSqlAdapter {
    conn: Arc<Mutex<libsql::Connection>>,
}

impl LibSqlAdapter {
    /// Create a new LibSQLAdapter with the given connection
    pub fn new(conn: libsql::Connection) -> Self {
        Self {
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

/// Helper method to convert SqlParam to libsql::Value
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

/// Helper function to get a column value from a row with proper type conversion
fn get_column_value<T>(row: &libsql::Row, idx: usize) -> Result<T>
where
    T: From<libsql::Value> + Default
{
    match row.get_value(idx as i32) {
        Ok(value) => Ok(T::from(value)),
        Err(e) => Err(Error::DatabaseError(format!("Failed to get column value: {}", e))),
    }
}

/// Helper function to convert a row to a HashMap for easier access
fn row_to_map(row: &libsql::Row) -> Result<HashMap<String, libsql::Value>> {
    let mut map = HashMap::new();
    let column_count = row.column_count();

    for i in 0..column_count {
        let column_name = row.column_name(i as i32)
            .ok_or_else(|| Error::DatabaseError(format!("Failed to get column name for index {}", i)))?;

        let value = match row.get_value(i) {
            Ok(val) => val,
            Err(e) => return Err(Error::DatabaseError(format!("Failed to get value: {}", e))),
        };

        map.insert(column_name.to_string(), value);
    }

    Ok(map)
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
        let sql = query.to_string(SqliteQueryBuilder);

        // Execute the query
        self.execute(&sql).await?;

        // Get a lock on the connection
        let conn = self.conn.lock().await;

        // Get the table name from the query to construct a proper SELECT
        let table_name = extract_table_name_from_insert(&sql)?;

        // Construct a query to get the last inserted row
        let select_sql = format!("SELECT * FROM {} WHERE rowid = last_insert_rowid()", table_name);

        // Execute the query
        let mut stmt = conn.prepare(&select_sql).await
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
            None => Err(Error::DatabaseError("Failed to retrieve inserted row".to_string())),
        }
    }

    async fn insert_many(&self, query: &InsertStatement) -> Result<u64> {
        // Convert the InsertStatement to SQL string
        let sql = query.to_string(SqliteQueryBuilder);

        // Execute the query and return the number of affected rows
        self.execute(&sql).await
    }

    async fn update_one<T>(&self, query: &UpdateStatement) -> Result<T>
    where
        T: FromRow<Self::Row> + Send,
    {
        // Convert the UpdateStatement to SQL string
        let sql = query.to_string(SqliteQueryBuilder);

        // Execute the query
        let affected = self.execute(&sql).await?;

        if affected == 0 {
            return Err(Error::NotFoundError);
        }

        // Get a lock on the connection
        let conn = self.conn.lock().await;

        // Get the table name and where clause from the query
        let (table_name, where_clause) = extract_table_and_where_from_update(&sql)?;

        // Construct a query to get the updated row
        let select_sql = format!("SELECT * FROM {} WHERE {}", table_name, where_clause);

        // Execute the query
        let mut stmt = conn.prepare(&select_sql).await
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
            None => Err(Error::DatabaseError("Failed to retrieve updated row".to_string())),
        }
    }

    async fn update_many(&self, query: &UpdateStatement) -> Result<u64> {
        // Convert the UpdateStatement to SQL string
        let sql = query.to_string(SqliteQueryBuilder);

        // Execute the query and return the number of affected rows
        self.execute(&sql).await
    }

    async fn upsert<T>(&self, query: &InsertStatement) -> Result<T>
    where
        T: FromRow<Self::Row> + Send,
    {
        // Convert the InsertStatement to SQL string with ON CONFLICT clause
        let sql = query.to_string(SqliteQueryBuilder);

        // Execute the query
        self.execute(&sql).await?;

        // For SQLite, we need to fetch the upserted row
        // This is complex because we don't know if it was an insert or update
        // We'll use the same approach as insert_one and hope it works
        let conn = self.conn.lock().await;

        // Get the table name from the query
        let table_name = extract_table_name_from_insert(&sql)?;

        // Construct a query to get the last inserted/updated row
        // This is a best effort - it will work for inserts but may not for updates
        let select_sql = format!("SELECT * FROM {} WHERE rowid = last_insert_rowid()", table_name);

        // Execute the query
        let mut stmt = conn.prepare(&select_sql).await
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
            None => Err(Error::DatabaseError("Failed to retrieve upserted row".to_string())),
        }
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

// Helper function to extract table name from an INSERT statement
fn extract_table_name_from_insert(sql: &str) -> Result<String> {
    // Simple regex-free parsing for "INSERT INTO table_name"
    let parts: Vec<&str> = sql.split_whitespace().collect();

    // Find the "INTO" keyword and get the next token
    for (i, part) in parts.iter().enumerate() {
        if part.to_uppercase() == "INTO" && i + 1 < parts.len() {
            // Get the table name (might include schema name)
            let table_ref = parts[i + 1];

            // Remove any parentheses or quotes
            let table_name = table_ref
                .trim_start_matches('(')
                .trim_start_matches('`')
                .trim_start_matches('"')
                .trim_end_matches(')')
                .trim_end_matches('`')
                .trim_end_matches('"')
                .split('.')  // Handle schema.table format
                .last()
                .unwrap_or(table_ref);

            return Ok(table_name.to_string());
        }
    }

    Err(Error::DatabaseError("Failed to extract table name from INSERT statement".to_string()))
}

// Helper function to extract table name and WHERE clause from an UPDATE statement
fn extract_table_and_where_from_update(sql: &str) -> Result<(String, String)> {
    // Find the table name (after "UPDATE")
    let update_parts: Vec<&str> = sql.split_whitespace().collect();

    let mut table_name = String::new();
    // We don't need this variable
    // let mut where_start_idx = 0;

    // Find the table name (after "UPDATE")
    for (i, part) in update_parts.iter().enumerate() {
        if part.to_uppercase() == "UPDATE" && i + 1 < update_parts.len() {
            // Get the table name
            table_name = update_parts[i + 1]
                .trim_start_matches('`')
                .trim_start_matches('"')
                .trim_end_matches('`')
                .trim_end_matches('"')
                .to_string();
            break;
        }
    }

    if table_name.is_empty() {
        return Err(Error::DatabaseError("Failed to extract table name from UPDATE statement".to_string()));
    }

    // Find the WHERE clause
    let where_clause = if let Some(where_idx) = sql.to_uppercase().find("WHERE") {
        &sql[where_idx + 5..]
    } else {
        return Err(Error::DatabaseError("Failed to extract WHERE clause from UPDATE statement".to_string()));
    };

    Ok((table_name, where_clause.to_string()))
}
