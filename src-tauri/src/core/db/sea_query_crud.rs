use sea_query::{InsertStatement, UpdateStatement, DeleteStatement};

/// Trait for generating SeaQuery statements from model structs
///
/// This trait provides methods to generate insert, update, and delete statements
/// for a model struct. It is typically implemented via the `#[derive(SeaQueryCrud)]` macro.
pub trait SeaQueryCrudTrait {
    /// Generates an insert statement for the model
    fn insert(&self) -> InsertStatement;

    /// Generates an update statement for the model with primary key WHERE clause
    fn update(&self) -> UpdateStatement;

    /// Generates a delete statement for the model with primary key WHERE clause
    fn delete(&self) -> DeleteStatement;
}
