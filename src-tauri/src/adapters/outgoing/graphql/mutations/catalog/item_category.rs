use crate::{
    adapters::outgoing::graphql::Mutation,
    core::{
        command::{catalog::item_category::CreateItemCategoryCommand, Command},
        entities::catalog::item_category::ItemCategory,
    },
    AppState,
};
use juniper::{graphql_object, FieldResult};

#[graphql_object(context = AppState)]
impl Mutation {
    fn create_item_category(
        name: String,
        description: Option<String>,
        context: &AppState,
    ) -> FieldResult<ItemCategory> {
        let mut service = context.service.lock().unwrap();
        let res = CreateItemCategoryCommand { name, description }.exec(&mut service)?;
        Ok(res)
    }
}
