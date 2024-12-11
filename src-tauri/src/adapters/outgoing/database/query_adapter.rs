use crate::core::common::interface::sql::query::{
    join_entity::JoinEntities, query::QueryInterface,
};

/// Adapter for querying data from a database.
/// Usage:
/// ```
/// Initialize the query adapter
/// let adapter = QueryAdapter;

/// // Example 1: Query Items
/// let item_joins = JoinEntities::new(vec![ItemRelation::Category(ItemCategoryRelation::Items(vec![]))]);
/// let items: Result<Vec<Item>, _> = adapter.get_many(item_joins);
/// let single_item: Result<Item, _> = adapter.get_one_by_id(
///     "item_id",
///     JoinEntities::new(vec![ItemRelation::Category(ItemCategoryRelation::Items(vec![]))]),
/// );

/// // Example 2: Query ItemCategories
/// let category_joins = JoinEntities::new(vec![ItemCategoryRelation::Items(vec![])]);
/// let categories: Result<Vec<ItemCategory>, _> = adapter.get_many(category_joins);
/// let single_category: Result<ItemCategory, _> = adapter.get_one_by_id(
///     "category_id",
///     JoinEntities::new(vec![ItemCategoryRelation::Items(vec![])]),
/// );
pub struct QueryAdapter;

impl<T, R> QueryInterface<T, R> for QueryAdapter {
    fn get_many(&self, with: JoinEntities<R>) -> Result<Vec<T>, std::io::Error> {
        unimplemented!()
    }

    fn get_one_by_id(&self, id: &str, with: JoinEntities<R>) -> Result<T, std::io::Error> {
        unimplemented!()
    }
}
