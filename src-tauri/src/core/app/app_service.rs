use crate::core::entities::catalog::{
    item::interface::ItemInterface, item_category::interface::ItemCategoryInterface,
};

pub struct AppService<'a> {
    pub item_category: &'a dyn ItemCategoryInterface,
    pub item: &'a dyn ItemInterface,
}
