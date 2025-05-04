use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

/// Derive macro to enable SeaQuery Value conversions for newtype wrappers.
///
/// This macro implements From<Type> and From<&Type> for sea_query::Value,
/// as well as From<Type> and From<&Type> for sea_query::SimpleExpr.
///
/// It's designed for newtype wrappers like DbUuid and Money that need
/// to be converted to sea_query::Value for use in database operations.
pub fn sea_query_type_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let type_name = input.ident;

    // Ensure this is a struct with a single field (newtype pattern)
    match input.data {
        Data::Struct(data_struct) => {
            match data_struct.fields {
                Fields::Unnamed(ref fields) if fields.unnamed.len() == 1 => {
                    // Valid newtype struct, continue with code generation
                },
                _ => panic!("SeaQueryType can only be applied to newtype structs (structs with a single unnamed field)"),
            }
        },
        _ => panic!("SeaQueryType can only be applied to structs"),
    };

    // Generate implementations for From<Type> and From<&Type> for sea_query::Value
    let gen = quote! {
        // Implement From<Type> for sea_query::Value
        impl From<#type_name> for sea_query::Value {
            fn from(value: #type_name) -> Self {
                sea_query::Value::String(Some(Box::new(value.to_sql())))
            }
        }

        // Implement From<&Type> for sea_query::Value
        impl From<&#type_name> for sea_query::Value {
            fn from(value: &#type_name) -> Self {
                sea_query::Value::String(Some(Box::new(value.to_sql())))
            }
        }
    };

    gen.into()
}
