use crate::core::entities::catalog::item::item_model::ItemRepository;
use crate::core::entities::catalog::item_category::item_category_model::ItemCategoryRepository;

pub struct CatalogService<'a> {
    pub item_category: &'a dyn ItemCategoryRepository,
    pub item: &'a dyn ItemRepository,
}
