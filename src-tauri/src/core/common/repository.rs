use std::{collections::HashSet, hash::Hash};

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
