use crate::adapters::graphql::{Mutation, Query, Schema};
use crate::error::Result;
use crate::AppState;
use juniper::{DefaultScalarValue, EmptySubscription, ExecutionError, Variables};
use tauri::State;

#[tauri::command]
pub fn graphql(
    query: String,
    vars: Variables<DefaultScalarValue>,
    state: State<'_, AppState>,
) -> Result<(juniper::Value, Vec<ExecutionError<DefaultScalarValue>>)> {
    juniper::execute_sync(
        &query,
        None,
        &Schema::new(Query, Mutation, EmptySubscription::new()),
        &vars,
        &state,
    )
    .map_err(|err| err.into())
}

#[tauri::command]
pub fn schema() -> String {
    Schema::new(Query, Mutation, EmptySubscription::new()).as_sdl()
}
