use std::{collections::HashSet, hash::Hash};

/// A generic struct that manages entity relations for dynamic join operations in repositories.
///
/// `JoinEntities` provides a type-safe way to specify which related entities should be included
/// when fetching data from a repository. It uses a `HashSet` to store unique relation types,
/// ensuring that each relation is only joined once.
///
/// # Type Parameters
///
/// * `R` - The relation enum type that implements `Sized + Clone + Eq + Hash`. This is typically
///         an enum that defines all possible relations for a specific entity.
///
/// # Examples
///
/// ```rust
/// // Define relations for an Item entity
/// enum ItemRelation {
///     Category(ItemCategoryRelation),  // Nested relation
/// }
///
/// // Define relations for an ItemCategory entity
/// enum ItemCategoryRelation {
///     Items(Vec<ItemRelation>),  // Can specify nested relations
/// }
///
/// # Simple Join Example
/// // Create a new JoinEntities instance
/// let mut joins = JoinEntities::default();
///
/// // Add relations to include in the query
/// joins.with.insert(ItemRelation::Category(
///     ItemCategoryRelation::Items(vec![])
/// ));
///
/// // Use in a repository method
/// repository.get_many(joins);
/// ```
///
/// # Multiple Join Example
///
/// Here's how to perform nested joins to fetch items with their categories and the category's other items:
///
/// ```rust
/// // 1. Create JoinEntities for fetching an Item with its Category
/// let mut item_joins = JoinEntities::default();
///
/// // 2. Request Category relation and specify that we want the Category's Items too
/// item_joins.with.insert(ItemRelation::Category(
///     ItemCategoryRelation::Items(vec![
///         // 3. For each Item in the Category, we can specify what to include
///         ItemRelation::Category(ItemCategoryRelation::Items(vec![])) // Further nesting if needed
///     ])
/// ));
///
/// // This will generate SQL similar to:
/// // SELECT * FROM items i
/// // LEFT JOIN item_categories ic ON i.category_id = ic.id
/// // LEFT JOIN items category_items ON ic.id = category_items.category_id
///
/// // Use in repository implementation:
/// impl ItemRepository for ItemAdapter {
///     fn get_many(&self, with: JoinEntities<ItemRelation>) -> Result<Vec<Item>, Error> {
///         let mut query = "SELECT * FROM items";
///
///         // Check for Category relation
///         if let Some(ItemRelation::Category(cat_relation)) = with.with.iter().next() {
///             query += " LEFT JOIN item_categories ON items.category_id = item_categories.id";
///
///             // Check if we need to join Category's Items
///             if matches!(cat_relation, ItemCategoryRelation::Items(_)) {
///                 query += " LEFT JOIN items category_items ON item_categories.id = category_items.category_id";
///             }
///         }
///         // ... rest of implementation
///     }
/// }
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
