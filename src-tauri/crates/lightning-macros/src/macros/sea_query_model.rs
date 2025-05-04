use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, Data, Fields};
use inflector::Inflector;

/// A derive macro that generates a SeaQuery Iden enum for a struct.
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
///
/// This will generate:
///
/// ```rust
/// pub enum Users {
///     Table,
///     Id,
///     Username,
///     // ...other variants
/// }
///
/// impl sea_query::Iden for Users {
///     fn unquoted(&self, s: &mut dyn sea_query::Write) {
///         // ...implementation
///     }
/// }
/// ```
pub fn sea_query_model_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident.clone();

    // Default to plural form of struct name (e.g., User -> Users)
    let enum_name = format_ident!("{}", struct_name.to_string().to_plural());

    // Default table: snake_case(struct_name) + pluralized
    let table_name = struct_name.to_string().to_snake_case().to_plural();

    // Extract named fields
    let fields = match input.data {
        Data::Struct(ds) => {
            if let Fields::Named(named) = ds.fields {
                named.named
            } else {
                panic!("SeaQueryModel only supports named fields");
            }
        }
        _ => panic!("SeaQueryModel can only be used on structs"),
    };

    let mut variants = vec![quote! { Table }];
    let mut arms = vec![quote! { #enum_name::Table => #table_name }];
    let mut all_column_variants = Vec::new();

    for field in fields.iter() {
        let ident = field.ident.as_ref().unwrap();
        let field_name = ident.to_string();
        let pascal = field_name.to_pascal_case();
        let var_ident = format_ident!("{}", pascal);
        variants.push(quote! { #var_ident });
        arms.push(quote! { #enum_name::#var_ident => #field_name });
        all_column_variants.push(quote! { #enum_name::#var_ident });
    }

    let expanded = quote! {
        pub enum #enum_name {
            #(#variants),*
        }

        impl #enum_name {
            pub fn all_columns() -> Vec<Self> {
                vec![#(#all_column_variants),*]
            }
        }

        impl sea_query::Iden for #enum_name {
            fn unquoted(&self, s: &mut dyn sea_query::Write) {
                let _ = s.write_str(
                    match self {
                        #(#arms),*
                    }
                );
            }
        }
    };


    TokenStream::from(expanded)
}
