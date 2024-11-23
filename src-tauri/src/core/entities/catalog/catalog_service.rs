use super::{item::repository::ItemRepository, item_category::repository::ItemCategoryRepository};

pub struct CatalogService<'a> {
    pub item_category: &'a dyn ItemCategoryRepository,
    pub item: &'a dyn ItemRepository,
}
