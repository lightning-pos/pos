use diesel::{
    deserialize,
    expression::AsExpression,
    serialize::{self, IsNull, Output, ToSql},
    sql_types::Text,
    sqlite::Sqlite,
    Queryable,
};
use juniper::graphql_scalar;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, AsExpression)]
#[diesel(sql_type = Text)]
#[graphql_scalar]
#[graphql(transparent)]
pub struct DbUuid(Uuid);

impl From<Uuid> for DbUuid {
    fn from(uuid: Uuid) -> Self {
        DbUuid(uuid)
    }
}

impl Queryable<Text, Sqlite> for DbUuid {
    type Row = String;

    fn build(row: String) -> deserialize::Result<Self> {
        Ok(DbUuid(
            Uuid::parse_str(&row).map_err(|e| format!("Error parsing UUID: {}", e))?,
        ))
    }
}

impl ToSql<Text, Sqlite> for DbUuid {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(self.0.to_string());
        Ok(IsNull::No)
    }
}
