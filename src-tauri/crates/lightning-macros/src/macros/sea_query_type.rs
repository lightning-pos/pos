use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};
use proc_macro2::TokenStream as TokenStream2;

/// Derive macro to enable SeaQuery Value conversions for newtype wrappers.
///
/// This macro implements:
/// - From<Type> and From<&Type> for sea_query::Value
/// - sea_query::Nullable for Type (to support Option<Type> in SeaQuery)
/// - to_sql() method for converting the type to a SQL string representation
///
/// Note: SimpleExpr conversion is automatically provided by sea_query's blanket implementation
/// for any type that implements Into<sea_query::Value>.
///
/// It's designed for newtype wrappers like DbUuid and Money that need
/// to be converted to sea_query::Value for use in database operations.
pub fn sea_query_type_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let type_name = input.ident;

    // Ensure this is a struct with a single field (newtype pattern)
    match &input.data {
        Data::Struct(ref data_struct) => {
            match &data_struct.fields {
                Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                    // Valid newtype struct, continue with code generation
                },
                _ => panic!("SeaQueryType can only be applied to newtype structs (structs with a single unnamed field)"),
            }
        },
        _ => panic!("SeaQueryType can only be applied to structs"),
    };

    // Extract the inner field type to determine the appropriate value type
    let inner_type = match &input.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                    // Get the first and only field's type
                    let first_field = fields.unnamed.first().unwrap();
                    &first_field.ty
                },
                _ => panic!("SeaQueryType can only be applied to newtype structs (structs with a single unnamed field)"),
            }
        },
        _ => panic!("SeaQueryType can only be applied to structs"),
    };

    // Determine the appropriate value and null implementations based on the inner type
    let (value_impl, null_impl) = generate_value_impls(inner_type);

    // Generate implementations for From<Type> and From<&Type> for sea_query::Value
    // and to_sql method for the type
    let gen = quote! {
        // ---
        // Implements conversion from your newtype to sea_query::Value.
        // This enables you to use your type directly in SeaQuery expressions.
        //
        // Example:
        //   let uuid = DbUuid::from(...);
        //   let value: sea_query::Value = uuid.into();
        //
        // This will convert your newtype to the appropriate SQL value (e.g., String, Int).
        impl From<#type_name> for sea_query::Value {
            fn from(value: #type_name) -> Self {
                #value_impl
            }
        }

        // ---
        // Implements conversion from a reference to your newtype to sea_query::Value.
        // This allows you to pass references to your type in SeaQuery APIs.
        //
        // Example:
        //   let uuid = DbUuid::from(...);
        //   let value: sea_query::Value = (&uuid).into();
        //
        // This will dereference and convert as above.
        impl From<&#type_name> for sea_query::Value {
            fn from(value: &#type_name) -> Self {
                sea_query::Value::from(*value)
            }
        }

        // ---
        // Implements the Nullable trait for your type.
        // This enables SeaQuery to generate NULL values for Option<T> fields in your models.
        //
        // Example:
        //   let null_val = <DbUuid as sea_query::Nullable>::null();
        //
        // This will produce e.g., sea_query::Value::String(None) for DbUuid.
        impl sea_query::Nullable for #type_name {
            fn null() -> sea_query::Value {
                #null_impl
            }
        }

        // ---
        // Adds a to_sql method to your type for convenience.
        // This method returns a String suitable for storing in the database.
        //
        // Example:
        //   let money = Money(1099);
        //   let sql_str = money.to_sql(); // "1099"
        //
        // This is used by the macro's Value conversion as well.
        impl #type_name {
            #[inline]
            pub fn to_sql(&self) -> String {
                self.0.to_string()
            }
        }
    };

    gen.into()
}

// Helper function to generate the appropriate value and null implementations based on the inner type
fn generate_value_impls(ty: &syn::Type) -> (TokenStream2, TokenStream2) {
    // Convert the type to a string for pattern matching
    let type_str = quote!(#ty).to_string();

    // Determine the appropriate value and null implementations based on the inner type
    if type_str.contains("i64") || type_str.contains("i32") ||
       type_str.contains("u64") || type_str.contains("u32") ||
       type_str.contains("isize") || type_str.contains("usize") {
        // For integer types
        let value_impl = quote!(sea_query::Value::Int(Some(value.to_sql().parse().unwrap())));
        let null_impl = quote!(sea_query::Value::Int(None));
        (value_impl, null_impl)
    } else if type_str.contains("f64") || type_str.contains("f32") {
        // For floating point types
        let value_impl = quote!(sea_query::Value::Float(Some(value.to_sql().parse().unwrap())));
        let null_impl = quote!(sea_query::Value::Float(None));
        (value_impl, null_impl)
    } else if type_str.contains("bool") {
        // For boolean types
        let value_impl = quote!(sea_query::Value::Bool(Some(value.to_sql().parse().unwrap())));
        let null_impl = quote!(sea_query::Value::Bool(None));
        (value_impl, null_impl)
    } else {
        // Default to String for other types
        let value_impl = quote!(sea_query::Value::String(Some(Box::new(value.to_sql()))));
        let null_impl = quote!(sea_query::Value::String(None));
        (value_impl, null_impl)
    }
}
