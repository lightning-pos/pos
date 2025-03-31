use juniper::graphql_object;

use crate::core::{
    models::finance::sales_order_payment_model::{SalesOrderPayment, SalesOrderPaymentState},
    types::{db_uuid::DbUuid, money::Money},
};

#[graphql_object(description = "Sales Order Payment")]
impl SalesOrderPayment {
    fn id(&self) -> DbUuid {
        self.id
    }

    fn order_id(&self) -> DbUuid {
        self.order_id
    }

    fn payment_method_id(&self) -> DbUuid {
        self.payment_method_id
    }

    fn payment_date(&self) -> chrono::NaiveDateTime {
        self.payment_date
    }

    fn amount(&self) -> Money {
        self.amount
    }

    fn reference_number(&self) -> Option<&str> {
        self.reference_number.as_deref()
    }

    fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }

    fn state(&self) -> SalesOrderPaymentState {
        self.state
    }
}
