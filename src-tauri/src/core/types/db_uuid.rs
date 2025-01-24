use diesel::{
    deserialize::FromSqlRow,
    expression::AsExpression,
    serialize::{IsNull, Output, ToSql},
    sql_types::Text,
    sqlite::Sqlite,
    Queryable,
};
use juniper::graphql_scalar;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, AsExpression, FromSqlRow)]
#[diesel(sql_type = diesel::sql_types::Text)]
#[graphql_scalar]
#[graphql(transparent)]
pub struct DbUuid(Uuid);

impl From<Uuid> for DbUuid {
    fn from(uuid: Uuid) -> Self {
        DbUuid(uuid)
    }
}

impl Into<Uuid> for DbUuid {
    fn into(self) -> Uuid {
        self.0
    }
}

impl Queryable<diesel::sql_types::Text, Sqlite> for DbUuid {
    type Row = String;

    fn build(row: String) -> diesel::deserialize::Result<Self> {
        Ok(DbUuid(
            Uuid::parse_str(&row).map_err(|e| format!("Error parsing UUID: {}", e))?,
        ))
    }
}

impl ToSql<Text, Sqlite> for DbUuid {
    fn to_sql<'b>(
        &'b self,
        out: &mut Output<'b, '_, diesel::sqlite::Sqlite>,
    ) -> diesel::serialize::Result {
        out.set_value(self.0.to_string());
        Ok(IsNull::No)
    }
}
