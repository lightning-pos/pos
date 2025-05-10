use crate::adapters::graphql::{Mutation, Query, Schema};
use crate::error::Result;
use crate::AppState;
use juniper::{DefaultScalarValue, EmptySubscription, ExecutionError, Variables};
use tauri::State;

#[tauri::command]
pub async fn graphql(
    query: String,
    vars: Option<Variables<DefaultScalarValue>>,
    state: State<'_, AppState>,
) -> Result<(juniper::Value, Vec<ExecutionError<DefaultScalarValue>>)> {
    println!("yoyo query: {:?}", query);
    juniper::execute(
        &query,
        None,
        &Schema::new(Query, Mutation, EmptySubscription::new()),
        vars.as_ref().unwrap_or(&Variables::new()),
        &state,
    )
    .await
    .map_err(|err| err.into())
}

#[tauri::command]
pub fn schema() -> String {
    Schema::new(Query, Mutation, EmptySubscription::new()).as_sdl()
}
