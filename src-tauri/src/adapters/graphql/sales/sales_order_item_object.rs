use chrono::NaiveDateTime;
use juniper::graphql_object;

use crate::{
    core::{
        models::sales::sales_order_item_model::SalesOrderItem,
        types::{db_uuid::DbUuid, money::Money},
    },
    AppState,
};

#[graphql_object(context = AppState)]
impl SalesOrderItem {
    pub fn id(&self) -> DbUuid {
        self.id
    }

    pub fn order_id(&self) -> DbUuid {
        self.order_id
    }

    pub fn item_id(&self) -> DbUuid {
        self.item_id
    }

    pub fn item_name(&self) -> String {
        self.item_name.clone()
    }

    pub fn quantity(&self) -> i32 {
        self.quantity
    }

    pub fn price_amount(&self) -> Money {
        self.price_amount
    }

    pub fn tax_amount(&self) -> Money {
        self.tax_amount
    }

    pub fn total_amount(&self) -> Money {
        self.total_amount
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }
}
