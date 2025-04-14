mod sqlx_adapter;

pub use sqlx_adapter::SqlxAdapter;

use crate::{
    core::types::db_uuid::DbUuid,
    error::Result,
};

/// A type for SQL parameters that can be used with different database backends
#[derive(Clone)]
pub enum SqlParam {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
    Uuid(DbUuid),
}

/// A trait for database operations that can be implemented for different database backends.
/// This allows commands to construct queries without directly executing them.
pub trait DatabaseAdapter: Send + Sync {
    /// Execute a query that returns a single row
    fn query_one<T>(&self, query: &str, params: Vec<SqlParam>) -> Result<T>
    where
        T: Send + Sync;

    /// Execute a query that returns an optional row
    fn query_optional<T>(&self, query: &str, params: Vec<SqlParam>) -> Result<Option<T>>
    where
        T: Send + Sync;

    /// Execute a query that returns multiple rows
    fn query_many<T>(&self, query: &str, params: Vec<SqlParam>) -> Result<Vec<T>>
    where
        T: Send + Sync;

    /// Execute a query that doesn't return rows but returns the number of affected rows
    fn execute(&self, query: &str, params: Vec<SqlParam>) -> Result<u64>;

    /// Execute a transaction
    fn transaction<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&Self) -> Result<R> + Send + Sync,
        R: Send + Sync;
}
