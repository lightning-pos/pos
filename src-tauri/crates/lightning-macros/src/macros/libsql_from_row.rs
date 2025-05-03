use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Type, PathArguments, GenericArgument};

/// Derive macro to implement FromRow<libsql::Row> for structs.
/// This automatically generates the implementation for converting a database row to a struct.
pub fn libsql_from_row_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    // Get the fields of the struct
    let fields = match input.data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => &fields.named,
                _ => panic!("LibsqlFromRow can only be applied to structs with named fields"),
            }
        },
        _ => panic!("LibsqlFromRow can only be applied to structs"),
    };

    // Generate field conversions
    let mut field_conversions = Vec::new();
    let mut field_assignments = Vec::new();

    for (i, field) in fields.iter().enumerate() {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        // Check if the field type is an Option
        if let Some(inner_type) = extract_option_inner_type(field_type) {
            // For Option types, we need special handling
            field_conversions.push(quote! {
                let #field_name = match row.get_value(#i as i32) {
                    Ok(libsql::Value::Null) => None,
                    Ok(value) => {
                        let value_result = <#inner_type as FromLibsqlValue>::from_libsql_value(value)?;
                        match value_result {
                            Some(val) => Some(val),
                            None => None,
                        }
                    },
                    Err(_) => None,
                };
            });
        } else {
            // For non-Option types, we can use from_libsql_value directly
            field_conversions.push(quote! {
                let value_result = <#field_type as FromLibsqlValue>::from_libsql_value(row.get_value(#i as i32)?)?;
                let #field_name = value_result.ok_or_else(|| crate::error::Error::DatabaseError(
                    format!("Column at index {} cannot be null for non-optional field", #i)
                ))?;
            });
        }

        field_assignments.push(quote! {
            #field_name,
        });
    }

    // Generate the implementation
    let gen = quote! {
        impl FromRow<libsql::Row> for #struct_name {
            fn from_row(row: &libsql::Row) -> crate::error::Result<Self> {
                #(#field_conversions)*

                Ok(#struct_name {
                    #(#field_assignments)*
                })
            }
        }
    };

    gen.into()
}

// Helper function to extract the inner type of Option<T>
fn extract_option_inner_type(ty: &Type) -> Option<&Type> {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(GenericArgument::Type(inner_type)) = args.args.first() {
                        return Some(inner_type);
                    }
                }
            }
        }
    }
    None
}
