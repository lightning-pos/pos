use std::io::Error;

use crate::core::{
    common::interface::sql::query::QueryInterface,
    entities::catalog::item::model::{Item, ItemRelation},
};

pub trait ItemInterface: QueryInterface<Item, ItemRelation> {
    // Write methods
    fn insert(&self, item: &Item) -> Result<Item, Error>;
    fn update(&self, item: &Item) -> Result<Item, Error>;
    fn delete(&self, id: &str) -> Result<bool, Error>;
}
