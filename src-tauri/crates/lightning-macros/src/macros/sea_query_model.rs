use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, Data, Fields, Type};
use inflector::Inflector;

/// A derive macro that generates a SeaQuery Iden enum for a struct and optional input structs.
///
/// This macro will:
/// 1. Generate an enum with the pluralized form of the struct name
/// 2. Add variants for the table and all fields
/// 3. Implement `sea_query::Iden` for the enum
/// 4. Optionally generate NewInput and UpdateInput structs when specified
///
/// # Configuration Options
///
/// - `new_input`: Generate the NewInput struct
/// - `update_input`: Generate the UpdateInput struct
///
/// # Example
///
/// ```rust
/// // Basic usage (only generates Iden enum)
/// #[derive(Debug, SeaQueryModel)]
/// pub struct User {
///     pub id: DbUuid,
///     pub username: String,
///     // ...other fields
/// }
///
/// // Generate input structs
/// #[sea_query_model(new_input, update_input)]
/// #[derive(Debug, SeaQueryModel)]
/// pub struct Item {
///     pub id: DbUuid,
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
///
/// // Also generates these unless skipped:
/// pub struct UserNewInput { /* fields */ }
/// pub struct UserUpdateInput { /* fields */ }
/// ```
pub fn sea_query_model_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident.clone();

    // Parse configuration options from attributes
    let mut generate_new_input = false;
    let mut generate_update_input = false;

    // Simple string-based parsing for attributes
    for attr in &input.attrs {
        // Convert the entire attribute to a string for simple parsing
        let attr_str = format!("{:?}", attr);
        if attr_str.contains("sea_query_model") {
            if attr_str.contains("new_input") {
                generate_new_input = true;
            }
            if attr_str.contains("update_input") {
                generate_update_input = true;
            }
        }
    }

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

    // Collect fields for NewInput and UpdateInput structs
    let mut new_input_fields = Vec::new();
    let mut update_input_fields = Vec::new();
    let new_input_name = format_ident!("{}{}", struct_name, "NewInput");
    let update_input_name = format_ident!("{}{}", struct_name, "UpdateInput");

    for field in fields.iter() {
        let ident = field.ident.as_ref().unwrap();
        let field_name = ident.to_string();
        let pascal = field_name.to_pascal_case();
        let var_ident = format_ident!("{}", pascal);
        let field_type = &field.ty;

        // Add to Iden enum
        variants.push(quote! { #var_ident });
        arms.push(quote! { #enum_name::#var_ident => #field_name });
        all_column_variants.push(quote! { #enum_name::#var_ident });

        // Skip id, created_at, updated_at for NewInput
        if field_name != "id" && field_name != "created_at" && field_name != "updated_at" {
            // For NewInput, use the original type
            new_input_fields.push(quote! {
                pub #ident: #field_type
            });
        }

        // For UpdateInput
        if field_name == "id" {
            // id is required in UpdateInput
            update_input_fields.push(quote! {
                pub #ident: #field_type
            });
        } else if field_name != "created_at" && field_name != "updated_at" {
            // Make all other fields optional in UpdateInput
            // Check if the type is already an Option<T>
            let is_option = if let Type::Path(type_path) = field_type {
                if let Some(segment) = type_path.path.segments.first() {
                    segment.ident == "Option"
                } else {
                    false
                }
            } else {
                false
            };

            if is_option {
                // If already Option<T>, wrap in another Option
                update_input_fields.push(quote! {
                    pub #ident: Option<#field_type>
                });
            } else {
                // If not Option<T>, make it Option<T>
                update_input_fields.push(quote! {
                    pub #ident: Option<#field_type>
                });
            }
        }
    }

    // Build the base implementation for the Iden enum
    let mut expanded = quote! {
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

    // Add NewInput struct if specified
    if generate_new_input {
        expanded = quote! {
            #expanded

            #[derive(Debug, Clone, juniper::GraphQLInputObject)]
            pub struct #new_input_name {
                #(#new_input_fields),*
            }
        };
    }

    // Add UpdateInput struct if specified
    if generate_update_input {
        expanded = quote! {
            #expanded

            #[derive(Debug, Clone, juniper::GraphQLInputObject)]
            pub struct #update_input_name {
                #(#update_input_fields),*
            }
        };
    };

    TokenStream::from(expanded)
}
