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

    let (generate_new_input, generate_update_input) = parse_sea_query_model_attrs(&input.attrs);

    let enum_name = format_ident!("{}", struct_name.to_string().to_plural());

    let fields = extract_named_fields(&input.data);

    let (variants, arms, all_column_variants, new_input_fields, update_input_fields) =
        process_fields(&fields, &struct_name);

    let new_input_name = format_ident!("{}NewInput", struct_name);
    let update_input_name = format_ident!("{}UpdateInput", struct_name);

    let mut expanded = generate_iden_enum_and_impl(
        &enum_name,
        &variants,
        &arms,
        &all_column_variants,
    );

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

fn parse_sea_query_model_attrs(attrs: &[syn::Attribute]) -> (bool, bool) {
    let mut generate_new_input = false;
    let mut generate_update_input = false;
    for attr in attrs {
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
    (generate_new_input, generate_update_input)
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
) {
    let enum_name = format_ident!("{}", struct_name.to_string().to_plural());
    let table_name = struct_name.to_string().to_snake_case().to_plural();
    let mut variants = vec![quote! { Table }];
    let mut arms = vec![quote! { #enum_name::Table => #table_name }];
    let mut all_column_variants = Vec::new();
    let mut new_input_fields = Vec::new();
    let mut update_input_fields = Vec::new();
    for field in fields.iter() {
        let ident = field.ident.as_ref().unwrap();
        let field_name = ident.to_string();
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

