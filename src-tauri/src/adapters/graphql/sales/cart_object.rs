use chrono::NaiveDateTime;
use sea_query::{Expr, Query};
use juniper::{graphql_object, FieldResult};

use crate::{
    adapters::outgoing::database::DatabaseAdapter,
    core::{
        models::sales::{
            cart_model::Cart,
            customer_model::{Customer, Customers},
        },
        types::db_uuid::DbUuid,
    },
    AppState,
};

#[graphql_object(context = AppState)]
impl Cart {
    pub fn id(&self) -> DbUuid {
        self.id
    }

    pub fn cart_data(&self) -> String {
        self.cart_data.clone()
    }

    pub fn customer_id(&self) -> Option<DbUuid> {
        self.customer_id
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }

    // Relationships
    pub async fn customer(&self, context: &AppState) -> FieldResult<Option<Customer>> {
        if let Some(customer_id) = self.customer_id {
            let service = context.service.lock().await;

            let mut query = Query::select();
            let query = query
                .from(Customers::Table)
                .columns([
                    Customers::Id,
                    Customers::FullName,
                    Customers::Email,
                    Customers::Phone,
                    Customers::Address,
                    Customers::CreatedAt,
                    Customers::UpdatedAt,
                ])
                .and_where(Expr::col(Customers::Id).eq(customer_id.to_string()));

            let customer = service.db_adapter.query_one::<Customer>(&query).await?;

            Ok(Some(customer))
        } else {
            Ok(None)
        }
    }
}
