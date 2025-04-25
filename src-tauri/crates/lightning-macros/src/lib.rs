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
