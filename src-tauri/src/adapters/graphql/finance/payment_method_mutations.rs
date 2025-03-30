use juniper::FieldResult;

use crate::{
    core::{
        commands::{
            finance::payment_method_commands::{
                CreatePaymentMethodCommand, DeletePaymentMethodCommand, UpdatePaymentMethodCommand,
            },
            Command,
        },
        models::finance::payment_method_model::{
            PaymentMethod, PaymentMethodNewInput, PaymentMethodState, PaymentMethodUpdateInput,
        },
        types::db_uuid::DbUuid,
    },
    AppState,
};

pub fn create_payment_method(
    name: String,
    code: String,
    description: Option<String>,
    state: Option<PaymentMethodState>,
    context: &AppState,
) -> FieldResult<PaymentMethod> {
    let mut service = context.service.lock().unwrap();
    let command = CreatePaymentMethodCommand {
        payment_method: PaymentMethodNewInput {
            name,
            code,
            description,
            state,
        },
    };
    let result = command.exec(&mut service)?;
    Ok(result)
}

pub fn update_payment_method(
    id: DbUuid,
    name: Option<String>,
    code: Option<String>,
    description: Option<Option<String>>,
    state: Option<PaymentMethodState>,
    context: &AppState,
) -> FieldResult<PaymentMethod> {
    let mut service = context.service.lock().unwrap();
    let command = UpdatePaymentMethodCommand {
        payment_method: PaymentMethodUpdateInput {
            id,
            name,
            code,
            description,
            state,
        },
    };
    let result = command.exec(&mut service)?;
    Ok(result)
}

pub fn delete_payment_method(id: DbUuid, context: &AppState) -> FieldResult<DbUuid> {
    let mut service = context.service.lock().unwrap();
    let command = DeletePaymentMethodCommand { id };
    let _ = command.exec(&mut service)?;
    // Return the id of the deleted payment method
    Ok(id)
}
