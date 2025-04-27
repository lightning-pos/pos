//! Lightning Macros
//!
//! A collection of procedural macros for the Lightning POS system.

extern crate proc_macro;

mod macros;

use proc_macro::TokenStream;

/// Generates a SeaQuery Iden enum for a struct.
///
/// This macro will:
/// 1. Generate an enum with the pluralized form of the struct name
/// 2. Add variants for the table and all fields
/// 3. Implement `sea_query::Iden` for the enum
///
/// # Example
///
/// ```rust
/// #[derive(Debug, SeaQueryModel)]
/// pub struct User {
///     pub id: DbUuid,
///     pub username: String,
///     // ...other fields
/// }
/// ```
#[proc_macro_derive(SeaQueryModel)]
pub fn sea_query_model_derive(input: TokenStream) -> TokenStream {
    macros::sea_query_model::sea_query_model_derive(input)
}

/// Generates SeaQueryCrud implementation for a struct.
///
/// This macro will:
/// 1. Generate insert, update, and delete methods for the struct
/// 2. Use the primary key field(s) for WHERE clauses in update and delete
///
/// # Example
///
/// ```rust
/// #[derive(Debug, SeaQueryCrud)]
/// #[sea_query(table = "users")]
/// pub struct User {
///     #[sea_query(primary_key)]
///     pub id: DbUuid,
///     pub username: String,
///     // ...other fields
/// }
/// ```
#[proc_macro_derive(SeaQueryCrud, attributes(sea_query))]
pub fn sea_query_crud_derive(input: TokenStream) -> TokenStream {
    macros::sea_query_crud::sea_query_crud_derive(input)
}

/// Marks an enum for use with sea_query.
///
/// This macro is a marker that indicates the enum should be compatible with sea_query.
/// The actual implementations for From<Enum> and From<&Enum> for sea_query::Value should be
/// provided in the sea_query_value_impls.rs file to avoid conflicts.
///
/// # Example
///
/// ```rust
/// #[derive(Debug, Display, Clone, PartialEq, GraphQLEnum, SeaQueryEnum)]
/// pub enum UserState {
///     Active,
///     Inactive,
///     Locked,
/// }
/// ```
#[proc_macro_derive(SeaQueryEnum)]
pub fn sea_query_enum_derive(input: TokenStream) -> TokenStream {
    macros::sea_query_enum::sea_query_enum_derive(input)
}

/// Marks a newtype wrapper for use with sea_query.
///
/// This macro is a marker that indicates the newtype wrapper should be compatible with sea_query.
/// The actual implementations for From<Type> and From<&Type> for sea_query::Value should be
/// provided in the sea_query_value_impls.rs file to avoid conflicts.
///
/// # Example
///
/// ```rust
/// #[derive(Debug, Clone, SeaQueryType)]
/// pub struct DbUuid(uuid::Uuid);
/// ```
#[proc_macro_derive(SeaQueryType)]
pub fn sea_query_type_derive(input: TokenStream) -> TokenStream {
    macros::sea_query_type::sea_query_type_derive(input)
}

/// Automatically implements FromLibsqlValue for enums.
///
/// This macro will generate an implementation of FromLibsqlValue for an enum,
/// allowing it to be directly converted from libsql::Value.
///
/// # Example
///
/// ```rust
/// #[derive(Debug, Clone, Display, PartialEq, GraphQLEnum, LibsqlEnum)]
/// pub enum UserState {
///     Active,
///     Inactive,
///     Locked,
/// }
/// ```
#[proc_macro_derive(LibsqlEnum)]
pub fn libsql_enum_derive(input: TokenStream) -> TokenStream {
    macros::libsql_enum::libsql_enum_derive(input)
}

/// Automatically implements FromLibsqlValue for tuple structs.
///
/// This macro will generate an implementation of FromLibsqlValue for a tuple struct,
/// allowing it to be directly converted from libsql::Value. It works with types like
/// DbUuid, Money, etc. that wrap a single value.
///
/// # Example
///
/// ```rust
/// #[derive(Debug, Clone, Copy, LibsqlType)]
/// pub struct DbUuid(Uuid);
/// ```
#[proc_macro_derive(LibsqlType)]
pub fn libsql_type_derive(input: TokenStream) -> TokenStream {
    macros::libsql_type::libsql_type_derive(input)
}

/// Automatically implements FromRow<libsql::Row> for structs.
///
/// This macro will generate an implementation of FromRow<libsql::Row> for a struct,
/// allowing it to be directly converted from a database row. It works with structs
/// that have named fields and automatically handles Option types.
///
/// # Example
///
/// ```rust
/// #[derive(Debug, SeaQueryModel, SeaQueryCrud, LibsqlFromRow)]
/// pub struct User {
///     pub id: DbUuid,
///     pub username: String,
///     pub last_login_at: Option<NaiveDateTime>,
///     // ...other fields
/// }
/// ```
#[proc_macro_derive(LibsqlFromRow)]
pub fn libsql_from_row_derive(input: TokenStream) -> TokenStream {
    macros::libsql_from_row::libsql_from_row_derive(input)
}
