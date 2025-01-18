use crate::core::command::catalog::item_category::*;
use crate::core::command::Command;
use crate::core::entities::catalog::item_category::ItemCategory;
use crate::error::Result;
use crate::schema::item_categories;
use crate::AppState;
use diesel::prelude::*;
use juniper::{
    graphql_object, Context, DefaultScalarValue, EmptyMutation, EmptySubscription, ExecutionError,
    FieldResult, RootNode, Variables,
};
use tauri::State;

impl Context for AppState {}

#[derive(Default)]
pub struct Query;

#[graphql_object(context = AppState)]
impl Query {
    fn api_version() -> &'static str {
        "1.0.0"
    }

    fn item_categories(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        context: &AppState,
    ) -> FieldResult<Vec<ItemCategory>> {
        let mut service = context.service.lock().unwrap();

        let mut query = item_categories::table.into_boxed();

        // Apply pagination if parameters are provided
        if let Some(limit) = first {
            query = query.limit(limit as i64);
        }
        if let Some(off) = offset {
            query = query.offset(off as i64);
        }

        let result = query
            .select(ItemCategory::as_select())
            .load::<ItemCategory>(&mut service.conn)?;

        Ok(result)
    }
}

type Schema = RootNode<'static, Query, EmptyMutation<AppState>, EmptySubscription<AppState>>;

#[tauri::command]
pub fn graphql(
    query: String,
    state: State<'_, AppState>,
) -> Result<(juniper::Value, Vec<ExecutionError<DefaultScalarValue>>)> {
    println!("Query: {}", query);
    juniper::execute_sync(
        &query,
        None,
        &Schema::new(Query, EmptyMutation::new(), EmptySubscription::new()),
        &Variables::new(),
        &state,
    )
    .map_err(|err| err.into())
}

#[tauri::command]
pub fn create_item_category(
    name: String,
    description: Option<String>,
    state: State<'_, AppState>,
) -> Result<ItemCategory> {
    let mut service = state.service.lock().unwrap();
    CreateItemCategoryCommand { name, description }.exec(&mut service)
}

#[tauri::command]
pub fn update_item_category(
    category: ItemCategory,
    state: State<'_, AppState>,
) -> Result<ItemCategory> {
    let mut service = state.service.lock().unwrap();
    UpdateItemCategoryCommand { category }.exec(&mut service)
}

#[tauri::command]
pub fn delete_item_category(category: ItemCategory, state: State<'_, AppState>) -> Result<()> {
    let mut service = state.service.lock().unwrap();
    DeleteItemCategoryCommand { category }.exec(&mut service)
}
