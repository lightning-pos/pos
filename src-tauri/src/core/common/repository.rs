use std::{collections::HashSet, hash::Hash};

/// Manages entity relations for dynamic join operations in repositories.
///
/// `JoinEntities` specifies which related entities to include when fetching data,
/// using a `HashSet` to store unique relation types.
///
/// # Type Parameters
///
/// * `R`: The relation enum type (implements `Sized + Clone + Eq + Hash`).
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
/// joins.with.insert(ItemRelation::Category(ItemCategoryRelation::Items(vec![])));
/// repository.get_many(joins);
///
/// // Nested join
/// let mut item_joins = JoinEntities::default();
/// item_joins.with.insert(ItemRelation::Category(
///     ItemCategoryRelation::Items(vec![ItemRelation::Category(ItemCategoryRelation::Items(vec![]))])
/// ));
/// ```
#[derive(Debug, Clone)]
pub struct JoinEntities<R: Sized + Clone + Eq + Hash> {
    pub with: HashSet<R>,
}

impl<R: Sized + Clone + Eq + Hash> Default for JoinEntities<R> {
    fn default() -> Self {
        Self {
            with: HashSet::new(),
        }
    }
}
