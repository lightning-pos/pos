mod sqlx_adapter;

pub use sqlx_adapter::SqlxAdapter;

use sea_query::{SelectStatement, InsertStatement, UpdateStatement, DeleteStatement};

use std::future::Future;
use std::pin::Pin;

use crate::{
    core::types::db_uuid::DbUuid,
    error::Result,
};

/// A type for SQL parameters that can be used with different database backends
#[derive(Clone)]
pub enum SqlParam {
    Null,
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Uuid(DbUuid),
}

/// A trait for database operations that can be implemented for different database backends.
/// This allows commands to construct queries without directly executing them.
pub trait DatabaseAdapter: Send + Sync {
    /// Execute a query that returns a single row or fails if no row is found
    async fn query_one<T>(&self, query: &SelectStatement) -> Result<T>;

    /// Execute a query that returns an optional row (Some if found, None if not found)
    async fn query_optional<T>(&self, query: &SelectStatement) -> Result<Option<T>>;

    /// Execute a query that returns all rows matching the parameters
    async fn query_many<T>(&self, query: &SelectStatement) -> Result<Vec<T>>;

    /// Insert a single row and return the inserted entity
    async fn insert_one<T>(&self, query: &InsertStatement) -> Result<T>;

    /// Insert multiple rows and return the number of rows affected
    async fn insert_many(&self, query: &InsertStatement) -> Result<u64>;

    /// Update a single row and return the updated entity
    async fn update_one<T>(&self, query: &UpdateStatement) -> Result<T>;

    /// Update multiple rows and return the number of rows affected
    async fn update_many(&self, query: &UpdateStatement) -> Result<u64>;

    /// Upsert (insert or update) a row and return the resulting entity
    async fn upsert<T>(&self, query: &InsertStatement) -> Result<T>;

    /// Delete rows matching the filter and return the number of rows affected
    async fn delete(&self, query: &DeleteStatement) -> Result<u64>;

    /// Execute a transaction
    async fn transaction<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&Self) -> Pin<Box<dyn Future<Output = Result<R>> + Send>> + Send,
        R: Send;

    /// Execute a query that doesn't return rows but returns the number of affected rows
    /// This is a low-level method used by other methods
    async fn execute(&self, query: &str) -> Result<u64>;
}


