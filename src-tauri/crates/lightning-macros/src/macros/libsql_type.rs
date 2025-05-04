use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Type};

/// Derive macro to implement FromLibsqlValue for tuple structs.
/// This automatically generates the implementation for converting libsql::Value to the type.
pub fn libsql_type_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let type_name = &input.ident;
    let inner_type = get_inner_type(&input);
    let is_simple = is_simple_type(inner_type);

    let gen = match is_simple {
        true => gen_simple_type_impl(type_name, inner_type),
        false => gen_custom_type_impl(type_name),
    };

    gen.into()
}

fn get_inner_type(input: &DeriveInput) -> &Type {
    match input.data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Unnamed(ref fields) if fields.unnamed.len() == 1 => {
                    let field = fields.unnamed.first().unwrap();
                    &field.ty
                },
                _ => panic!("LibsqlType can only be applied to tuple structs with a single field"),
            }
        },
        _ => panic!("LibsqlType can only be applied to tuple structs"),
    }
}

fn gen_simple_type_impl(type_name: &syn::Ident, inner_type: &Type) -> proc_macro2::TokenStream {
    quote! {
        impl FromLibsqlValue for #type_name {
            fn from_libsql_value(value: libsql::Value) -> crate::error::Result<Option<Self>> {
                match value {
                    libsql::Value::Text(s) => {
                        match s.parse::<#inner_type>() {
                            Ok(val) => Ok(Some(#type_name(val))),
                            Err(_) => Err(crate::error::Error::DatabaseError(
                                format!("Cannot parse '{}' as {}", s, stringify!(#inner_type))
                            )),
                        }
                    },
                    libsql::Value::Integer(i) => {
                        Ok(Some(#type_name(i as #inner_type)))
                    },
                    libsql::Value::Real(f) => {
                        Ok(Some(#type_name(f as #inner_type)))
                    },
                    libsql::Value::Null => Ok(None),
                    _ => Err(crate::error::Error::DatabaseError(
                        format!("Invalid {} value type in database", stringify!(#type_name))
                    )),
                }
            }
        }
    }
}

fn gen_custom_type_impl(type_name: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        impl FromLibsqlValue for #type_name {
            fn from_libsql_value(value: libsql::Value) -> crate::error::Result<Option<Self>> {
                match value {
                    libsql::Value::Text(s) => {
                        match Self::from_str(&s) {
                            Ok(val) => Ok(Some(val)),
                            Err(e) => Err(crate::error::Error::DatabaseError(
                                format!("Error parsing {}: {}", stringify!(#type_name), e)
                            )),
                        }
                    },
                    libsql::Value::Null => Ok(None),
                    _ => Err(crate::error::Error::DatabaseError(
                        format!("Invalid {} value type in database (expected string)", stringify!(#type_name))
                    )),
                }
            }
        }
    }
}


// Helper function to check if a type is a simple type (primitive or String)
fn is_simple_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            let type_name = segment.ident.to_string();
            return matches!(
                type_name.as_str(),
                "i8" | "i16" | "i32" | "i64" | "i128" | "isize" |
                "u8" | "u16" | "u32" | "u64" | "u128" | "usize" |
                "f32" | "f64" | "bool" | "char" | "String"
            );
        }
    }
    false
}
