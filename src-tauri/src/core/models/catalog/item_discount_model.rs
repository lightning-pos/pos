use juniper::GraphQLInputObject;
use sea_query::Iden;

use crate::core::types::db_uuid::DbUuid;

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
