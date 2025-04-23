use juniper::GraphQLInputObject;
use sea_query::Iden;

use crate::{adapters::outgoing::database::FromRow, core::types::db_uuid::DbUuid, error::Result};

#[derive(Debug)]
pub struct ItemDiscount {
    pub item_id: DbUuid,
    pub discount_id: DbUuid,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ItemDiscountNewInput {
    pub item_id: DbUuid,
    pub discount_id: DbUuid,
}

#[derive(Iden)]
pub enum ItemDiscounts {
    Table,
    ItemId,
    DiscountId,
}

impl FromRow<libsql::Row> for ItemDiscount {
    fn from_row(row: &libsql::Row) -> Result<Self> {
        todo!()
    }
}

