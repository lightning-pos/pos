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
    fn query_one<T>(&self, query: &SelectStatement) -> impl Future<Output = Result<T>> + Send;

    /// Execute a query that returns an optional row (Some if found, None if not found)
    fn query_optional<T>(&self, query: &SelectStatement) -> impl Future<Output = Result<Option<T>>> + Send;

    /// Execute a query that returns all rows matching the parameters
    fn query_many<T>(&self, query: &SelectStatement) -> impl Future<Output = Result<Vec<T>>> + Send;

    /// Insert a single row and return the inserted entity
    fn insert_one<T>(&self, query: &InsertStatement) -> impl Future<Output = Result<T>> + Send;

    /// Insert multiple rows and return the number of rows affected
    fn insert_many(&self, query: &InsertStatement) -> impl Future<Output = Result<u64>> + Send;

    /// Update a single row and return the updated entity
    fn update_one<T>(&self, query: &UpdateStatement) -> impl Future<Output = Result<T>> + Send;

    /// Update multiple rows and return the number of rows affected
    fn update_many(&self, query: &UpdateStatement) -> impl Future<Output = Result<u64>> + Send;

    /// Upsert (insert or update) a row and return the resulting entity
    fn upsert<T>(&self, query: &InsertStatement) -> impl Future<Output = Result<T>> + Send;

    /// Delete rows matching the filter and return the number of rows affected
    fn delete(&self, query: &DeleteStatement) -> impl Future<Output = Result<u64>> + Send;

    /// Execute a transaction
    fn transaction<F, R>(&self, f: F) -> impl Future<Output = Result<R>> + Send
    where
        F: FnOnce(&Self) -> Pin<Box<dyn Future<Output = Result<R>> + Send>> + Send,
        R: Send;

    /// Execute a query that doesn't return rows but returns the number of affected rows
    /// This is a low-level method used by other methods
    fn execute(&self, query: &str) -> impl Future<Output = Result<u64>> + Send;
}


