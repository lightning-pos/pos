use juniper::GraphQLInputObject;
use lightning_macros::{LibsqlFromRow, SeaQueryCrud, SeaQueryModel};

use crate::{adapters::outgoing::database::{FromLibsqlValue, FromRow}, core::{db::SeaQueryCrudTrait, types::db_uuid::DbUuid}};

#[derive(Debug, SeaQueryModel, SeaQueryCrud, LibsqlFromRow)]
pub struct ItemDiscount {
    #[sea_query(primary_key)]
    pub item_id: DbUuid,
    #[sea_query(primary_key)]
    pub discount_id: DbUuid,
}

#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ItemDiscountNewInput {
    pub item_id: DbUuid,
    pub discount_id: DbUuid,
}
