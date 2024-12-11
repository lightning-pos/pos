/// Manages entity relations for dynamic join operations in repositories.
///
/// `JoinEntities` specifies which related entities to include when fetching data,
/// using a `Vec` to store relation types.
///
/// # Type Parameters
///
/// * `R`: The relation enum type to be joined.
///
/// # Examples
///
/// ```rust
/// enum ItemRelation {
///     Category(ItemCategoryRelation),
/// }
///
/// enum ItemCategoryRelation {
///     Items(Vec<ItemRelation>),
/// }
///
/// // Simple join
/// let mut joins = JoinEntities::default();
/// joins.with.push(ItemRelation::Category(ItemCategoryRelation::Items(vec![])));
/// repository.get_many(joins);
///
/// // Nested join
/// let mut item_joins = JoinEntities::default();
/// item_joins.with.push(ItemRelation::Category(
///     ItemCategoryRelation::Items(vec![ItemRelation::Category(ItemCategoryRelation::Items(vec![]))])
/// ));
/// ```
#[derive(Debug, Clone)]
pub struct JoinEntities<R> {
    pub with: Vec<R>,
}

impl<R> JoinEntities<R> {
    pub fn new(with: Vec<R>) -> Self {
        Self { with }
    }
}
