use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Ident};

/// Derive macro to implement FromLibsqlValue for enums.
/// This automatically generates the implementation for converting libsql::Value to the enum.
pub fn libsql_enum_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = input.ident;
    
    let data_enum = match input.data {
        Data::Enum(data_enum) => data_enum,
        _ => panic!("LibsqlEnum can only be applied to enums"),
    };
    
    // Ensure variants are unit variants
    for variant in data_enum.variants.iter() {
        if !variant.fields.is_empty() {
            panic!("LibsqlEnum only supports fieldless enum variants");
        }
    }
    
    // Extract variant names for match arms
    let variant_names: Vec<&Ident> = data_enum.variants.iter()
        .map(|v| &v.ident)
        .collect();
    
    // Generate match arms for each variant
    let match_arms = variant_names.iter().map(|name| {
        let name_str = name.to_string();
        quote! {
            #name_str => Ok(#enum_name::#name),
        }
    });
    
    // Generate the implementation
    let gen = quote! {
        impl crate::adapters::outgoing::database::FromLibsqlValue for #enum_name {
            fn from_libsql_value(value: libsql::Value) -> crate::error::Result<Self> {
                match value {
                    libsql::Value::Text(s) => match s.as_str() {
                        #(#match_arms)*
                        _ => Err(crate::error::Error::DatabaseError(format!("Invalid {} value in database: {}", stringify!(#enum_name), s))),
                    },
                    _ => Err(crate::error::Error::DatabaseError(format!("Invalid {} value type in database", stringify!(#enum_name)))),
                }
            }
        }
    };
    
    gen.into()
}
