use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, Data, Fields, Ident, Attribute, Lit};
use convert_case::{Case, Casing};
use inflector::Inflector;

/// A derive macro that generates SeaQueryCrud implementation for a struct.
///
/// This macro will:
/// 1. Generate insert, update, and delete methods for the struct
/// 2. Use the primary key field(s) for WHERE clauses in update and delete
///
/// # Example
///
/// ```rust
/// #[derive(Debug, SeaQueryCrud)]
/// #[sea_query(table = "users")]
/// pub struct User {
///     #[sea_query(primary_key)]
///     pub id: DbUuid,
///     pub username: String,
///     // ...other fields
/// }
/// ```
pub fn sea_query_crud_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident.clone();

    // Get the table name from attribute or infer from struct name
    let _table_name = get_table_name(&input.attrs, &struct_name);
    let table_enum = format_ident!("{}", struct_name.to_string().to_plural());

    // Extract fields and find primary key fields
    let fields = match input.data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => fields.named.iter().collect::<Vec<_>>(),
                _ => panic!("SeaQueryCrud only works with structs that have named fields"),
            }
        },
        _ => panic!("SeaQueryCrud only works with structs"),
    };

    // Collect fields with primary_key attribute
    let mut primary_key_fields: Vec<&syn::Field> = fields.iter()
        .filter(|field| has_primary_key_attr(&field.attrs))
        .cloned()
        .collect();

    if primary_key_fields.is_empty() {
        // If no primary key is explicitly marked, assume "id" is the primary key
        if let Some(id_field) = fields.iter()
            .find(|&field| field.ident.as_ref().map_or(false, |id| id == "id")) {
            primary_key_fields.push(id_field);
        } else {
            panic!("SeaQueryCrud requires at least one field marked as primary_key or a field named 'id'");
        }
    }

    // Generate the implementation
    let insert_impl = generate_insert_impl(&table_enum, &fields);
    let update_impl = generate_update_impl(&table_enum, &fields, &primary_key_fields);
    let delete_impl = generate_delete_impl(&table_enum, &primary_key_fields);

    // Generate the static helpers for primary key operations
    let static_helpers = generate_static_helpers(&table_enum, &primary_key_fields);

    let expanded = quote! {
        impl SeaQueryCrudTrait for #struct_name {
            #insert_impl

            #update_impl

            #delete_impl
        }

        impl #struct_name {
            #static_helpers
        }
    };

    TokenStream::from(expanded)
}

// Helper function to get the table name from attributes or infer from struct name
fn get_table_name(attrs: &[Attribute], struct_name: &Ident) -> String {
    for attr in attrs {
        if attr.path().is_ident("sea_query") {
            let mut table_name = None;
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("table") {
                    let value = meta.value()?;
                    let lit: Lit = value.parse()?;
                    if let Lit::Str(lit_str) = lit {
                        table_name = Some(lit_str.value());
                    }
                }
                Ok(())
            });
            if let Some(name) = table_name {
                return name;
            }
        }
    }

    // Default to pluralized snake_case of struct name
    struct_name.to_string().to_snake_case().to_plural()
}

// Helper function to check if a field has the primary_key attribute
fn has_primary_key_attr(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if attr.path().is_ident("sea_query") {
            let mut is_primary_key = false;
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("primary_key") {
                    is_primary_key = true;
                }
                Ok(())
            });
            if is_primary_key {
                return true;
            }
        }
    }
    false
}

// Generate the insert statement implementation
fn generate_insert_impl(table_enum: &Ident, fields: &[&syn::Field]) -> proc_macro2::TokenStream {
    // Generate column names
    let columns: Vec<proc_macro2::TokenStream> = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let column_name = format_ident!("{}", field_name.to_string().to_case(Case::Pascal));
        quote! { #table_enum::#column_name }
    }).collect();

    // Generate values
    let values: Vec<proc_macro2::TokenStream> = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        quote! {
            sea_query::SimpleExpr::from(
                sea_query::Value::from(self.#field_name.clone())
            )
        }
    }).collect();

    quote! {
        fn insert(&self) -> sea_query::InsertStatement {
            use sea_query::{Query, Table, Value};
            Query::insert()
                .into_table(#table_enum::Table)
                .columns([
                    #(#columns),*
                ])
                .values_panic([
                    #(#values),*
                ])
                .to_owned()
        }
    }
}

// Generate the update statement implementation
fn generate_update_impl(
    table_enum: &Ident,
    fields: &[&syn::Field],
    primary_key_fields: &[&syn::Field]
) -> proc_macro2::TokenStream {
    // Generate value assignments for each field
    let value_assignments: Vec<proc_macro2::TokenStream> = fields.iter()
        .filter(|field| {
            // Skip primary key fields in the update statement
            !primary_key_fields.contains(field)
        })
        .map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let column_name = format_ident!("{}", field_name.to_string().to_case(Case::Pascal));
            quote! {
                stmt.value(#table_enum::#column_name, sea_query::Value::from(self.#field_name.clone()));
            }
        })
        .collect();

    // Generate where conditions for primary key fields
    let where_conditions: Vec<proc_macro2::TokenStream> = primary_key_fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let column_name = format_ident!("{}", field_name.to_string().to_case(Case::Pascal));
        quote! {
            stmt.and_where(sea_query::Expr::col(#table_enum::#column_name).eq(sea_query::Value::from(self.#field_name.clone())));
        }
    }).collect();

    quote! {
        fn update(&self) -> sea_query::UpdateStatement {
            use sea_query::{Query, Table, Expr, Value};

            let mut stmt = Query::update();
            stmt.table(#table_enum::Table);

            // Add value assignments for each field
            #(#value_assignments)*

            // Add where conditions for primary key fields
            #(#where_conditions)*

            stmt.to_owned()
        }
    }
}

// Generate the delete statement implementation
fn generate_delete_impl(
    table_enum: &Ident,
    primary_key_fields: &[&syn::Field]
) -> proc_macro2::TokenStream {
    // Generate where conditions for primary key fields
    let where_conditions: Vec<proc_macro2::TokenStream> = primary_key_fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let column_name = format_ident!("{}", field_name.to_string().to_case(Case::Pascal));

        quote! {
            stmt.and_where(sea_query::Expr::col(#table_enum::#column_name).eq(sea_query::Value::from(self.#field_name.clone())));
        }
    }).collect();

    quote! {
        fn delete(&self) -> sea_query::DeleteStatement {
            use sea_query::{Query, Table, Expr, Value};

            let mut stmt = Query::delete();
            stmt.from_table(#table_enum::Table);

            // Add where conditions for primary key fields
            #(#where_conditions)*

            stmt.to_owned()
        }
    }
}

// Generate static helper methods for the struct
fn generate_static_helpers(
    table_enum: &Ident,
    primary_key_fields: &[&syn::Field]
) -> proc_macro2::TokenStream {
    if primary_key_fields.len() == 1 {
        // Single primary key field
        let pk_field = primary_key_fields[0];
        let pk_field_name = pk_field.ident.as_ref().unwrap();
        let pk_field_type = &pk_field.ty;
        let pk_column_name = format_ident!("{}", pk_field_name.to_string().to_case(Case::Pascal));
        let fn_name_delete = format_ident!("delete_by_{}", pk_field_name);
        let fn_name_find = format_ident!("find_by_{}", pk_field_name);

        quote! {
            /// Delete a record by its primary key
            pub fn #fn_name_delete(#pk_field_name: #pk_field_type) -> sea_query::DeleteStatement {
                use sea_query::{Query, Table, Expr, Value};

                Query::delete()
                    .from_table(#table_enum::Table)
                    .and_where(Expr::col(#table_enum::#pk_column_name).eq(Value::from(#pk_field_name)))
                    .to_owned()
            }

            /// Find a record by its primary key
            pub fn #fn_name_find(#pk_field_name: #pk_field_type) -> sea_query::SelectStatement {
                use sea_query::{Query, Table, Expr, Value};

                Query::select()
                    .from(#table_enum::Table)
                    .columns([
                        #table_enum::Table
                    ])
                    .and_where(Expr::col(#table_enum::#pk_column_name).eq(Value::from(#pk_field_name)))
                    .to_owned()
            }
        }
    } else if primary_key_fields.len() > 1 {
        // Multiple primary keys - generate a composite finder
        // Collect parameters for the function signature
        let pk_params: Vec<proc_macro2::TokenStream> = primary_key_fields.iter().map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let field_type = &field.ty;
            quote! { #field_name: #field_type }
        }).collect();

        // Collect where conditions
        let where_conditions: Vec<proc_macro2::TokenStream> = primary_key_fields.iter().map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let column_name = format_ident!("{}", field_name.to_string().to_case(Case::Pascal));
            quote! {
                stmt.and_where(Expr::col(#table_enum::#column_name).eq(Value::from(#field_name)));
            }
        }).collect();

        quote! {
            /// Find a record by its composite primary key
            pub fn find_by_primary_key(#(#pk_params),*) -> sea_query::SelectStatement {
                use sea_query::{Query, Table, Expr, Value};

                let mut stmt = Query::select();
                stmt.from(#table_enum::Table)
                   .columns([#table_enum::Table]);

                // Add where conditions for all primary key fields
                #(#where_conditions)*

                stmt.to_owned()
            }

            /// Delete a record by its composite primary key
            pub fn delete_by_primary_key(#(#pk_params),*) -> sea_query::DeleteStatement {
                use sea_query::{Query, Table, Expr, Value};

                let mut stmt = Query::delete();
                stmt.from_table(#table_enum::Table);

                // Add where conditions for all primary key fields
                #(#where_conditions)*

                stmt.to_owned()
            }
        }
    } else {
        // This should never happen due to our earlier checks
        quote! {}
    }
}
