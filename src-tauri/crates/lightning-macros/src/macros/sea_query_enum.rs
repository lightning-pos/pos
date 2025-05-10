use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data};

/// Derive macro to enable SeaQuery Value conversions for enums.
/// Implements From<Enum> and From<&Enum> for sea_query::Value using the enum's Display implementation.
pub fn sea_query_enum_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = input.ident;
    let data_enum = match input.data {
        Data::Enum(data_enum) => data_enum,
        _ => panic!("SeaQueryEnum can only be applied to enums"),
    };
    // Ensure variants are unit variants
    for variant in data_enum.variants.iter() {
        if !variant.fields.is_empty() {
            panic!("SeaQueryEnum only supports fieldless enum variants");
        }
    }

    // Generate implementations for From<Enum> and From<&Enum> for sea_query::Value
    let gen = quote! {
        // Implement From<Enum> for sea_query::Value
        impl From<#enum_name> for sea_query::Value {
            fn from(value: #enum_name) -> Self {
                sea_query::Value::String(Some(Box::new(value.to_string())))
            }
        }

        // Implement From<&Enum> for sea_query::Value
        impl From<&#enum_name> for sea_query::Value {
            fn from(value: &#enum_name) -> Self {
                sea_query::Value::String(Some(Box::new(value.to_string())))
            }
        }
    };
    gen.into()
}
