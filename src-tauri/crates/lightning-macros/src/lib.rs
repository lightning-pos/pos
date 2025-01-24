use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data};
use diesel::sql_types::Text;
use diesel::serialize::{ToSql, IsNull, Output};
use diesel::deserialize::{FromSql, Result as DeserializeResult};
use diesel::backend::RawValue;
use diesel::Queryable;

#[proc_macro_derive(ToSqlEnum)]
pub fn derive_to_sql(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    let name = input.ident;
    
    let variants = match input.data {
        Data::Enum(data) => data.variants,
        _ => panic!("ToSqlEnum can only be derived for enums"),
    };
    
    let match_arms = variants.into_iter().map(|v| {
        let variant = v.ident;
        let value = variant.to_string().to_lowercase();
        quote! {
            #name::#variant => #value
        }
    });
    
    let expanded = quote! {
        impl diesel::serialize::ToSql<diesel::sql_types::Text, diesel::sqlite::Sqlite> for #name {
            fn to_sql<'b>(
                &'b self,
                out: &mut diesel::serialize::Output<'b, '_, diesel::sqlite::Sqlite>,
            ) -> diesel::serialize::Result {
                let s = match self {
                    #(#match_arms),*
                };
                out.set_value(s);
                Ok(diesel::serialize::IsNull::No)
            }
        }
    };
    
    TokenStream::from(expanded)
}

#[proc_macro_derive(QueryableEnum)]
pub fn derive_queryable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    
    let expanded = quote! {
        impl diesel::deserialize::FromSql<diesel::sql_types::Text, diesel::sqlite::Sqlite> for #name {
            fn from_sql(bytes: diesel::backend::RawValue<'_, diesel::sqlite::Sqlite>) -> diesel::deserialize::Result<Self> {
                let string = <String as diesel::deserialize::FromSql<diesel::sql_types::Text, diesel::sqlite::Sqlite>>::from_sql(bytes)?;
                Ok(string.parse()?)
            }
        }

        impl diesel::Queryable<diesel::sql_types::Text, diesel::sqlite::Sqlite> for #name {
            type Row = String;
            
            fn build(row: Self::Row) -> diesel::deserialize::Result<Self> {
                Ok(Self::from_str(&row)?)
            }
        }
    };
    
    TokenStream::from(expanded)
}
