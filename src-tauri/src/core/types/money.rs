use std::{
    iter::Sum,
    ops::{Add, Div, Mul, Sub},
};

use diesel::{
    deserialize,
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

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, AsExpression)]
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

impl Add<Money> for Money {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Money(self.0 + other.0)
    }
}

impl Sum<Money> for Money {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::from(0), |a, b| a + b)
    }
}

impl Sub<Money> for Money {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Money(self.0 - other.0)
    }
}

impl Mul<i32> for Money {
    type Output = Money;

    fn mul(self, other: i32) -> Self::Output {
        Money(self.0 * other as i64)
    }
}

impl Div<i32> for Money {
    type Output = Money;

    fn div(self, other: i32) -> Self::Output {
        Money(self.0 / other as i64)
    }
}

impl Queryable<BigInt, Sqlite> for Money {
    type Row = i64;

    fn build(row: i64) -> deserialize::Result<Self> {
        Ok(Money(row))
    }
}

impl ToSql<BigInt, Sqlite> for Money {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(self.0);
        Ok(IsNull::No)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_same_currency() {
        let m1 = Money(1000);
        let m2 = Money(500);
        assert_eq!(m1 + m2, Money(1500));
    }

    #[test]
    fn test_subtract() {
        let m1 = Money(1500);
        let m2 = Money(500);
        assert_eq!(m1 - m2, Money(1000));
    }

    #[test]
    fn test_multiply_by_scalar() {
        let m = Money(1000);
        assert_eq!(m * 2, Money(2000));
    }

    #[test]
    fn test_divide_by_scalar() {
        let m = Money(1000);
        assert_eq!(m / 2, Money(500));
    }
}
