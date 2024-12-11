use std::io::Error;

use crate::core::{
    common::interface::sql::query::{join_entity::JoinEntities, query::QueryInterface},
    entities::catalog::item::{
        interface::ItemInterface,
        model::{Item, ItemRelation},
    },
};

pub struct ItemAdapter;

impl QueryInterface<Item, ItemRelation> for ItemAdapter {
    fn get_many(&self, _with: JoinEntities<ItemRelation>) -> Result<Vec<Item>, Error> {
        unimplemented!()
    }

    fn get_one_by_id(&self, _id: &str, _with: JoinEntities<ItemRelation>) -> Result<Item, Error> {
        unimplemented!()
    }
}

impl ItemInterface for ItemAdapter {
    fn insert(&self, _item: &Item) -> Result<Item, Error> {
        unimplemented!()
    }

    fn update(&self, _item: &Item) -> Result<Item, Error> {
        unimplemented!()
    }

    fn delete(&self, _id: &str) -> Result<bool, Error> {
        unimplemented!()
    }
}
