use diesel::{
    deserialize::FromSqlRow,
    expression::AsExpression,
    serialize::{self, IsNull, Output, ToSql},
    sql_types::BigInt,
    sqlite::Sqlite,
    Queryable,
};
use juniper::{
    graphql_scalar, InputValue, ParseScalarResult, ParseScalarValue, ScalarToken, ScalarValue,
    Value,
};

#[derive(Debug, Clone, Copy, AsExpression, FromSqlRow)]
#[diesel(sql_type = BigInt)]
#[graphql_scalar]
pub struct Money(i64);

impl Money {
    fn to_output<S: ScalarValue>(&self) -> Value<S> {
        Value::scalar(self.0.to_string())
    }

    fn from_input<S>(v: &InputValue<S>) -> Result<Self, String>
    where
        S: ScalarValue,
    {
        let s = v
            .as_string_value()
            .ok_or_else(|| "Expected a string value".to_string())?;
        let i = s
            .parse::<i64>()
            .map_err(|_| "Not a valid integer".to_string())?;
        Ok(Money(i))
    }

    fn parse_token<S>(value: ScalarToken<'_>) -> ParseScalarResult<S>
    where
        S: ScalarValue,
    {
        <String as ParseScalarValue<S>>::from_str(value)
            .or_else(|_| <i32 as ParseScalarValue<S>>::from_str(value))
    }
}

impl From<i64> for Money {
    fn from(value: i64) -> Self {
        Money(value)
    }
}

impl Queryable<BigInt, Sqlite> for Money {
    type Row = i64;

    fn build(row: i64) -> diesel::deserialize::Result<Self> {
        Ok(Money(row))
    }
}

impl ToSql<BigInt, Sqlite> for Money {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(self.0);
        Ok(IsNull::No)
    }
}
