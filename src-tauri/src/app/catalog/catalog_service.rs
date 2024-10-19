use crate::app::catalog::item::item_model::ItemRepository;
use crate::app::catalog::item_category::item_category_model::ItemCategoryRepository;

pub struct CatalogService<A, B>
where
    A: ItemCategoryRepository,
    B: ItemRepository,
{
    pub item_category: A,
    pub item: B,
}
