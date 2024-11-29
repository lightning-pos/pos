use super::{item::interface::ItemInterface, item_category::interface::ItemCategoryInterface};

pub struct CatalogService<'a> {
    pub item_category: &'a dyn ItemCategoryInterface,
    pub item: &'a dyn ItemInterface,
}
