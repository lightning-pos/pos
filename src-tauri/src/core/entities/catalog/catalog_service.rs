use crate::core::entities::catalog::item::model::ItemRepository;
use crate::core::entities::catalog::item_category::model::ItemCategoryRepository;

pub struct CatalogService<'a> {
    pub item_category: &'a dyn ItemCategoryRepository,
    pub item: &'a dyn ItemRepository,
}
