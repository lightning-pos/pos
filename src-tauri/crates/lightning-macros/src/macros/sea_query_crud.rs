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

    let primary_key_fields: Vec<&syn::Field> = fields.iter()
        .filter(|field| has_primary_key_attr(&field.attrs))
        .cloned()
        .collect();

    if primary_key_fields.is_empty() {
        // If no primary key is explicitly marked, assume "id" is the primary key
        let id_field = fields.iter()
            .find(|&field| field.ident.as_ref().map_or(false, |id| id == "id"));

        if id_field.is_none() {
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
    let field_columns = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let column_name = format_ident!("{}", field_name.to_string().to_case(Case::Pascal));
        quote! { #table_enum::#column_name }
    });

    // Generate field values for insert
    let field_values = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        quote! {
            sea_query::SimpleExpr::from(sea_query::Value::from(self.#field_name.clone()))
        }
    });

    quote! {
        fn insert(&self) -> sea_query::InsertStatement {
            use sea_query::{Query, Table};

            Query::insert()
                .into_table(#table_enum::Table)
                .columns([
                    #(#field_columns),*
                ])
                .values_panic([
                    #(#field_values),*
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
    let pk_fields: Vec<&syn::Field> = if primary_key_fields.is_empty() {
        // If no primary key is explicitly marked, assume "id" is the primary key
        fields.iter()
            .filter(|field| field.ident.as_ref().map_or(false, |id| id == "id"))
            .cloned()
            .collect()
    } else {
        primary_key_fields.to_vec()
    };

    // Generate set expressions for non-primary key fields
    let set_exprs = fields.iter()
        .filter(|field| !pk_fields.contains(field))
        .map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let column_name = format_ident!("{}", field_name.to_string().to_case(Case::Pascal));
            quote! {
                stmt.value(#table_enum::#column_name, sea_query::Value::from(self.#field_name.clone()));
            }
        });

    // Generate where conditions for primary key fields
    let where_conditions = pk_fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let column_name = format_ident!("{}", field_name.to_string().to_case(Case::Pascal));
        quote! {
            stmt.and_where(sea_query::Expr::col(#table_enum::#column_name).eq(sea_query::Value::from(self.#field_name.clone())));
        }
    });

    quote! {
        fn update(&self) -> sea_query::UpdateStatement {
            use sea_query::{Query, Table, Expr};

            let mut stmt = Query::update();
            stmt.table(#table_enum::Table);

            // Set all non-primary key fields
            #(#set_exprs)*

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
    let pk_fields: Vec<&syn::Field> = if primary_key_fields.is_empty() {
        // If no primary key is explicitly marked, assume "id" is the primary key
        vec![]  // This will be handled in the generated code
    } else {
        primary_key_fields.to_vec()
    };

    // Generate where conditions for primary key fields
    let where_conditions = pk_fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let column_name = format_ident!("{}", field_name.to_string().to_case(Case::Pascal));
        quote! {
            stmt.and_where(sea_query::Expr::col(#table_enum::#column_name).eq::<#field.ty>(sea_query::Value::from(self.#field_name.clone())));
        }
    });

    // If no primary keys were explicitly marked, assume "id"
    let default_id_condition = if pk_fields.is_empty() {
        quote! {
            stmt.and_where(sea_query::Expr::col(#table_enum::Id).eq::<crate::core::types::db_uuid::DbUuid>(self.id.clone().into()));
        }
    } else {
        quote! {}
    };

    quote! {
        fn delete(&self) -> sea_query::DeleteStatement {
            use sea_query::{Query, Table, Expr};

            let mut stmt = Query::delete();
            stmt.from_table(#table_enum::Table);

            // Add where conditions for primary key fields
            #(#where_conditions)*
            #default_id_condition

            stmt.to_owned()
        }
    }
}

// Generate static helper methods for the struct
fn generate_static_helpers(
    table_enum: &Ident,
    primary_key_fields: &[&syn::Field]
) -> proc_macro2::TokenStream {
    let pk_fields: Vec<&syn::Field> = if primary_key_fields.is_empty() {
        // If no primary key is explicitly marked, assume "id" is the primary key
        vec![]  // This will be handled in the generated code
    } else {
        primary_key_fields.to_vec()
    };

    if pk_fields.is_empty() {
        // Generate a helper for id-based operations
        let id_field_type = quote! { crate::core::types::db_uuid::DbUuid };

        quote! {
            /// Delete a record by its ID
            pub fn delete_by_id(id: #id_field_type) -> sea_query::DeleteStatement {
                use sea_query::{Query, Table, Expr};

                Query::delete()
                    .from_table(#table_enum::Table)
                    .and_where(Expr::col(#table_enum::Id).eq::<#id_field_type>(id.into()))
                    .to_owned()
            }

            /// Find a record by its ID
            pub fn find_by_id(id: #id_field_type) -> sea_query::SelectStatement {
                use sea_query::{Query, Table, Expr};

                Query::select()
                    .from(#table_enum::Table)
                    .columns([
                        #table_enum::Table
                    ])
                    .and_where(Expr::col(#table_enum::Id).eq::<#id_field_type>(id.into()))
                    .to_owned()
            }
        }
    } else if pk_fields.len() == 1 {
        // Single primary key field
        let pk_field = pk_fields[0];
        let pk_field_name = pk_field.ident.as_ref().unwrap();
        let pk_field_type = &pk_field.ty;
        let pk_column_name = format_ident!("{}", pk_field_name.to_string().to_case(Case::Pascal));
        let fn_name_delete = format_ident!("delete_by_{}", pk_field_name);
        let fn_name_find = format_ident!("find_by_{}", pk_field_name);

        quote! {
            /// Delete a record by its primary key
            pub fn #fn_name_delete(#pk_field_name: #pk_field_type) -> sea_query::DeleteStatement {
                use sea_query::{Query, Table, Expr};

                Query::delete()
                    .from_table(#table_enum::Table)
                    .and_where(Expr::col(#table_enum::#pk_column_name).eq::<#pk_field.ty>(#pk_field_name.into()))
                    .to_owned()
            }

            /// Find a record by its primary key
            pub fn #fn_name_find(#pk_field_name: #pk_field_type) -> sea_query::SelectStatement {
                use sea_query::{Query, Table, Expr};

                Query::select()
                    .from(#table_enum::Table)
                    .columns([
                        #table_enum::Table
                    ])
                    .and_where(Expr::col(#table_enum::#pk_column_name).eq::<#pk_field.ty>(#pk_field_name.into()))
                    .to_owned()
            }
        }
    } else {
        // Multiple primary keys - more complex case
        quote! {
            // For composite primary keys, we don't generate static helpers
            // as they would require multiple parameters and be less convenient
        }
    }
}
