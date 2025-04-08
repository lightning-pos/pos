use diesel::{prelude::Insertable, Associations, Queryable};
use juniper::GraphQLInputObject;

use crate::core::models::catalog::{
    discount_model::Discount,
    item_model::Item,
};
use crate::core::types::db_uuid::DbUuid;
use crate::schema::item_discounts;

#[derive(Debug, Clone, Queryable, Insertable, Associations)]
#[diesel(belongs_to(Item, foreign_key = item_id))]
#[diesel(belongs_to(Discount, foreign_key = discount_id))]
#[diesel(table_name = item_discounts)]
pub struct ItemDiscount {
    pub item_id: DbUuid,
    pub discount_id: DbUuid,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ItemDiscountNewInput {
    pub item_id: DbUuid,
    pub discount_id: DbUuid,
}
