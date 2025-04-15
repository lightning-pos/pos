use chrono::NaiveDateTime;
use sea_query::{Expr, Query, SqliteQueryBuilder};
use juniper::{graphql_object, FieldResult};

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::catalog::{
            variant_type_model::{VariantType, VariantTypes},
            variant_value_model::VariantValue,
        },
        types::db_uuid::DbUuid,
    },
    AppState,
};

#[graphql_object(context = AppState)]
impl VariantValue {
    pub fn id(&self) -> DbUuid {
        self.id
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }

    pub fn display_order(&self) -> i32 {
        self.display_order
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }

    pub fn variant_type(&self, context: &AppState) -> FieldResult<VariantType> {
        let service = context.service.lock().unwrap();
        
        let query = Query::select()
            .from(VariantTypes::Table)
            .columns([
                VariantTypes::Id,
                VariantTypes::Name,
                VariantTypes::Description,
                VariantTypes::CreatedAt,
                VariantTypes::UpdatedAt,
            ])
            .and_where(Expr::col(VariantTypes::Id).eq(self.variant_type_id.to_string()))
            .to_string(SqliteQueryBuilder);
            
        let variant_type = service.db_adapter.query_one::<VariantType>(&query, vec![])?;
        
        Ok(variant_type)
    }
}