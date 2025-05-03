use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Type};

/// Derive macro to implement FromLibsqlValue for tuple structs.
/// This automatically generates the implementation for converting libsql::Value to the type.
pub fn libsql_type_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let type_name = input.ident;

    // Get the inner type of the tuple struct
    let inner_type = match input.data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Unnamed(ref fields) if fields.unnamed.len() == 1 => {
                    // Get the first and only field type
                    let field = fields.unnamed.first().unwrap();
                    &field.ty
                },
                _ => panic!("LibsqlType can only be applied to tuple structs with a single field"),
            }
        },
        _ => panic!("LibsqlType can only be applied to tuple structs"),
    };

    // Check if the inner type is a primitive that can be directly cast
    let is_primitive = is_primitive_type(inner_type);

    // Check if this is the Percentage type
    let is_percentage = type_name.to_string() == "Percentage";

    // Generate different implementations based on the inner type
    let gen = if is_percentage {
        // Special handling for Percentage type
        quote! {
            impl FromLibsqlValue for #type_name {
                fn from_libsql_value(value: libsql::Value) -> crate::error::Result<Option<Self>> {
                    match value {
                        libsql::Value::Integer(i) => {
                            // For integer values, use directly as basis points
                            Ok(Some(#type_name(i as #inner_type)))
                        },
                        libsql::Value::Real(f) => {
                            // For float values, convert to basis points
                            let basis_points = (f * Self::BASIS_POINTS as f64).round() as #inner_type;
                            Ok(Some(#type_name(basis_points)))
                        },
                        libsql::Value::Text(s) => {
                            // For text values, parse using from_str
                            Self::from_str(&s).map_err(|e| crate::error::Error::DatabaseError(e)).map(Some)
                        },
                        libsql::Value::Null => {
                            // Default to 0% for NULL values
                            Ok(None)
                        },
                        _ => Err(crate::error::Error::DatabaseError(
                            format!("Invalid {} value type in database", stringify!(#type_name))
                        )),
                    }
                }
            }
        }
    } else if is_primitive {
        // For primitive types, we can use direct conversion
        quote! {
            impl FromLibsqlValue for #type_name {
                fn from_libsql_value(value: libsql::Value) -> crate::error::Result<Option<Self>> {
                    match value {
                        libsql::Value::Text(s) => {
                            // For text values, try to parse as the inner type
                            match s.parse::<#inner_type>() {
                                Ok(val) => Ok(Some(#type_name(val))),
                                Err(_) => Err(crate::error::Error::DatabaseError(
                                    format!("Cannot parse '{}' as {}", s, stringify!(#inner_type))
                                )),
                            }
                        },
                        libsql::Value::Integer(i) => {
                            // For integer values, convert directly if possible
                            Ok(Some(#type_name(i as #inner_type)))
                        },
                        libsql::Value::Real(f) => {
                            // For float values, convert if possible
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
    } else if is_uuid_type(inner_type) {
        // Special handling for UUID types
        quote! {
            impl FromLibsqlValue for #type_name {
                fn from_libsql_value(value: libsql::Value) -> crate::error::Result<Option<Self>> {
                    match value {
                        libsql::Value::Text(s) => {
                            // For UUIDs, use the parse_str method
                            Self::parse_str(&s).map(Some)
                        },
                        libsql::Value::Null => Ok(None),
                        _ => Err(crate::error::Error::DatabaseError(
                            format!("Invalid {} value type in database", stringify!(#type_name))
                        )),
                    }
                }
            }
        }
    } else {
        // For other types, provide a more generic implementation
        quote! {
            impl FromLibsqlValue for #type_name {
                fn from_libsql_value(value: libsql::Value) -> crate::error::Result<Option<Self>> {
                    match value {
                        libsql::Value::Text(s) => {
                            // For text, use from_str if available
                            match Self::from_str(&s) {
                                Ok(val) => Ok(Some(val)),
                                Err(e) => Err(crate::error::Error::DatabaseError(
                                    format!("Error parsing {}: {}", stringify!(#type_name), e)
                                )),
                            }
                        },
                        libsql::Value::Integer(i) => {
                            // For integers, use From if available
                            Ok(Some(Self::from(i)))
                        },
                        libsql::Value::Null => Ok(None),
                        _ => Err(crate::error::Error::DatabaseError(
                            format!("Invalid {} value type in database", stringify!(#type_name))
                        )),
                    }
                }
            }
        }
    };

    gen.into()
}

// Helper function to check if a type is a primitive type that can be directly cast
fn is_primitive_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            let type_name = segment.ident.to_string();
            return matches!(
                type_name.as_str(),
                "i8" | "i16" | "i32" | "i64" | "i128" | "isize" |
                "u8" | "u16" | "u32" | "u64" | "u128" | "usize" |
                "f32" | "f64" | "bool" | "char"
            );
        }
    }
    false
}

// Helper function to check if a type is a UUID
fn is_uuid_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident.to_string() == "Uuid";
        }
    }
    false
}
