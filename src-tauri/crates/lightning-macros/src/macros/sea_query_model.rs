use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput};
use inflector::Inflector;

/// A derive macro that generates a SeaQuery Iden enum for a struct and optional input structs.
///
/// This macro will:
/// 1. Generate an enum with the pluralized form of the struct name
/// 2. Add variants for the table and all fields
/// 3. Implement `sea_query::Iden` for the enum
/// 4. Optionally generate NewInput and UpdateInput structs when specified
/// 5. Optionally generate a queries module with CRUD operations
///
/// # Configuration Options
///
/// - `new_input`: Generate the NewInput struct
/// - `update_input`: Generate the UpdateInput struct
/// - `queries`: Generate the queries module with CRUD operations
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
/// // Generate input structs and queries module
/// #[sea_query_model(new_input, update_input, queries)]
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
/// // Also generates these when specified:
/// pub struct UserNewInput { /* fields */ }
/// pub struct UserUpdateInput { /* fields */ }
/// pub mod queries { /* CRUD operations */ }
/// ```
pub fn sea_query_model_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident.clone();

    let (generate_new_input, generate_update_input, generate_queries) = parse_sea_query_model_attrs(&input.attrs);

    let enum_name = format_ident!("{}", struct_name.to_string().to_plural());

    let fields = extract_named_fields(&input.data);

    let (variants, arms, all_column_variants, new_input_fields, update_input_fields, field_idents) =
        process_fields(&fields, &struct_name);

    let new_input_name = format_ident!("{}{}", struct_name, "NewInput");
    let update_input_name = format_ident!("{}{}", struct_name, "UpdateInput");

    let mut expanded = generate_iden_enum_and_impl(
        &enum_name,
        &variants,
        &arms,
        &all_column_variants,
    );

    // Generate the queries module only if requested
    if generate_queries {
        let queries_module = generate_queries_module(
            &struct_name,
            &enum_name,
            &field_idents,
            generate_new_input,
            generate_update_input,
        );

        expanded = quote! {
            #expanded
            #queries_module
        };
    }

    if generate_new_input {
        expanded = quote! {
            #expanded
            #[derive(Debug, Clone, juniper::GraphQLInputObject)]
            pub struct #new_input_name {
                #(#new_input_fields),*
            }
        };
    }
    if generate_update_input {
        expanded = quote! {
            #expanded
            #[derive(Debug, Clone, juniper::GraphQLInputObject)]
            pub struct #update_input_name {
                #(#update_input_fields),*
            }
        };
    }
    TokenStream::from(expanded)
}

fn parse_sea_query_model_attrs(attrs: &[syn::Attribute]) -> (bool, bool, bool) {
    let mut generate_new_input = false;
    let mut generate_update_input = false;
    let mut generate_queries = false;
    for attr in attrs {
        let attr_str = format!("{:?}", attr);
        if attr_str.contains("sea_query_model") {
            if attr_str.contains("new_input") {
                generate_new_input = true;
            }
            if attr_str.contains("update_input") {
                generate_update_input = true;
            }
            if attr_str.contains("queries") {
                generate_queries = true;
            }
        }
    }
    (generate_new_input, generate_update_input, generate_queries)
}

fn extract_named_fields(data: &syn::Data) -> syn::punctuated::Punctuated<syn::Field, syn::token::Comma> {
    match data {
        syn::Data::Struct(ds) => {
            if let syn::Fields::Named(named) = &ds.fields {
                named.named.clone()
            } else {
                panic!("SeaQueryModel only supports named fields");
            }
        }
        _ => panic!("SeaQueryModel can only be used on structs"),
    }
}

fn process_fields(
    fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
    struct_name: &syn::Ident,
) -> (
    Vec<proc_macro2::TokenStream>,
    Vec<proc_macro2::TokenStream>,
    Vec<proc_macro2::TokenStream>,
    Vec<proc_macro2::TokenStream>,
    Vec<proc_macro2::TokenStream>,
    Vec<proc_macro2::TokenStream>,
) {
    let enum_name = format_ident!("{}", struct_name.to_string().to_plural());
    let table_name = struct_name.to_string().to_snake_case().to_plural();
    let mut variants = vec![quote! { Table }];
    let mut arms = vec![quote! { #enum_name::Table => #table_name }];
    let mut all_column_variants = Vec::new();
    let mut new_input_fields = Vec::new();
    let mut update_input_fields = Vec::new();
    let mut field_idents = Vec::new();
    for field in fields.iter() {
        let ident = field.ident.as_ref().unwrap();
        let field_name = ident.to_string();
        field_idents.push(quote! { #ident });
        let pascal = field_name.to_pascal_case();
        let var_ident = format_ident!("{}", pascal);
        let field_type = &field.ty;
        variants.push(quote! { #var_ident });
        arms.push(quote! { #enum_name::#var_ident => #field_name });
        all_column_variants.push(quote! { #enum_name::#var_ident });
        if field_name != "id" && field_name != "created_at" && field_name != "updated_at" {
            new_input_fields.push(quote! {
                pub #ident: #field_type
            });
        }
        if field_name == "id" {
            update_input_fields.push(quote! {
                pub #ident: #field_type
            });
        } else if field_name != "created_at" && field_name != "updated_at" {
            update_input_fields.push(quote! {
                pub #ident: Option<#field_type>
            });
        }
    }
    (
        variants,
        arms,
        all_column_variants,
        new_input_fields,
        update_input_fields,
        field_idents,
    )
}

fn generate_iden_enum_and_impl(
    enum_name: &proc_macro2::Ident,
    variants: &[proc_macro2::TokenStream],
    arms: &[proc_macro2::TokenStream],
    all_column_variants: &[proc_macro2::TokenStream],
) -> proc_macro2::TokenStream {
    quote! {
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
    }
}

fn generate_queries_module(
    struct_name: &proc_macro2::Ident,
    enum_name: &proc_macro2::Ident,
    field_idents: &[proc_macro2::TokenStream],
    generate_new_input: bool,
    generate_update_input: bool,
) -> proc_macro2::TokenStream {
    // Check if this model has an id field
    let has_id_field = field_idents.iter().any(|field_ident| {
        let field_str = format!("{}", quote! { #field_ident });
        field_str.contains("id")
    });
    let new_input_name = format_ident!("{}{}", struct_name, "NewInput");
    let update_input_name = format_ident!("{}{}", struct_name, "UpdateInput");

    // Generate find_by_id function only if the model has an id field
    let find_by_id = if has_id_field {
        quote! {
            pub fn find_by_id(id: &DbUuid) -> SelectStatement {
                let columns = #enum_name::all_columns();
                SelectStatement::new()
                    .from(#enum_name::Table)
                    .columns(columns)
                    .and_where(Expr::col(#enum_name::Id).eq(id)).to_owned()
            }
        }
    } else {
        quote! {}
    };

    // Check if this is a User model (only User model should have find_by_username)
    let is_user_model = struct_name.to_string() == "User";

    // Generate find_by_username function only for User model
    let find_by_username = if is_user_model {
        quote! {
            pub fn find_by_username(username: &str) -> SelectStatement {
                let columns = #enum_name::all_columns();
                SelectStatement::new()
                    .from(#enum_name::Table)
                    .columns(columns)
                    .and_where(Expr::col(#enum_name::Username).eq(username)).to_owned()
            }
        }
    } else {
        quote! {}
    };

    // Generate insert function if new_input is enabled
    let insert = if generate_new_input {
        // Create field variant identifiers for each field
        let mut field_variants = Vec::new();
        let mut field_accessors = Vec::new();

        for field_ident in field_idents {
            let field_str = format!("{}", quote! { #field_ident });
            let field_name = field_str.trim_matches(|c| c == ' ' || c == '"');

            // Skip id, created_at, and updated_at as they are handled separately
            if field_name != "id" && field_name != "created_at" && field_name != "updated_at" {
                let field_pascal = field_name.to_pascal_case();
                let field_variant = format_ident!("{}", field_pascal);
                field_variants.push(quote! { #enum_name::#field_variant });

                let field_ident_parsed = format_ident!("{}", field_name);
                field_accessors.push(quote! { input.#field_ident_parsed.clone().into() });
            }
        }

        quote! {
            pub fn insert(input: &#new_input_name) -> InsertStatement {
                let id: DbUuid = Uuid::now_v7().into();
                let now = Utc::now().naive_utc();

                let mut stmt = InsertStatement::new();
                stmt.into_table(#enum_name::Table);

                // Add all columns in the correct order
                stmt.columns([
                    #enum_name::Id,
                    #(#field_variants),*,
                    #enum_name::CreatedAt,
                    #enum_name::UpdatedAt
                ]);

                // Add all values in the same order
                stmt.values_panic([
                    id.into(),
                    #(#field_accessors),*,
                    now.into(),
                    now.into()
                ]);

                stmt.to_owned()
            }
        }
    } else {
        quote! {}
    };

    // Generate update function if update_input is enabled
    let update = if generate_update_input {
        // Process each field for conditional updates
        let mut field_updates = Vec::new();

        for field_ident in field_idents {
            let field_str = format!("{}", quote! { #field_ident });
            let field_name = field_str.trim_matches(|c| c == ' ' || c == '"');

            // Skip id, created_at, and updated_at as they are handled separately
            if field_name != "id" && field_name != "created_at" && field_name != "updated_at" {
                let field_pascal = field_name.to_pascal_case();
                let field_variant = format_ident!("{}", field_pascal);
                let field_ident_parsed = format_ident!("{}", field_name);

                // Handle Option<Option<T>> fields specially (like last_login_at)
                if field_name.contains("_at") || field_name.starts_with("opt_") {
                    field_updates.push(quote! {
                        if let Some(value) = &input.#field_ident_parsed {
                            match value {
                                Some(v) => stmt.value(#enum_name::#field_variant, v.clone()),
                                None => stmt.value(#enum_name::#field_variant, Expr::value(Value::String(None))),
                            };
                        }
                    });
                } else {
                    field_updates.push(quote! {
                        if let Some(value) = &input.#field_ident_parsed {
                            stmt.value(#enum_name::#field_variant, value.clone());
                        }
                    });
                }
            }
        }

        quote! {
            pub fn update(input: &#update_input_name) -> UpdateStatement {
                let now = Utc::now().naive_utc();

                let mut stmt = UpdateStatement::new();
                stmt.table(#enum_name::Table);

                // Apply conditional updates for each field
                #(#field_updates)*

                // Always update the updated_at field
                stmt.value(#enum_name::UpdatedAt, now);

                // Add the WHERE condition for the ID
                stmt.and_where(Expr::col(#enum_name::Id).eq(input.id));

                stmt.to_owned()
            }
        }
    } else {
        quote! {}
    };

    // Generate delete_by_id function only if the model has an id field
    let delete_by_id = if has_id_field {
        quote! {
            pub fn delete_by_id(id: &DbUuid) -> DeleteStatement {
                DeleteStatement::new()
                    .from_table(#enum_name::Table)
                    .and_where(Expr::col(#enum_name::Id).eq(id))
                    .to_owned()
            }
        }
    } else {
        quote! {}
    };

    // Combine all functions into the queries module with a unique name
    quote! {
        pub mod queries {
            use chrono::{NaiveDateTime, Utc};
            use sea_query::{DeleteStatement, Expr, InsertStatement, SelectStatement, SimpleExpr, UpdateStatement, Value};
            use uuid::Uuid;
            use crate::core::types::db_uuid::DbUuid;

            use super::*;

            #find_by_id

            #find_by_username

            #insert

            #update

            #delete_by_id
        }
    }
}
