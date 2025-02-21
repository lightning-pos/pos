use std::{
    iter::Sum,
    ops::{Add, Div, Mul, Sub},
};

use diesel::{
    deserialize::{self, FromSql},
    expression::AsExpression,
    serialize::{self, Output, ToSql},
    sql_types::Integer,
    sqlite::{Sqlite, SqliteValue},
    Queryable,
};
use juniper::{graphql_scalar, InputValue, ScalarValue, Value};

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, AsExpression)]
#[diesel(sql_type = Integer)]
#[graphql_scalar(parse_token(String))]
pub struct Percentage(i32);

impl Percentage {
    pub const BASIS_POINTS: i32 = 10000; // Using 10000 for 4 decimal precision

    /// Creates a new Percentage from a float value
    /// Example: 2.5 becomes 25000 basis points (2.5%)
    /// Rounds to nearest basis point
    pub fn from_float(value: f32) -> Self {
        Self((value * Self::BASIS_POINTS as f32).round() as i32)
    }

    /// Creates a new Percentage from a string representation
    /// Example: "2.5" becomes 25000 basis points (2.5%)
    /// Rounds to nearest basis point
    pub fn from_str(s: &str) -> Result<Self, String> {
        let value = s.parse::<f32>().map_err(|e| e.to_string())?;

        // Convert to basis points with rounding
        let basis_points = (value * Self::BASIS_POINTS as f32).round() as i32;
        Ok(Self(basis_points))
    }

    /// Returns the percentage as a string without % symbol
    /// Example: 25000 basis points becomes "2.5"
    pub fn to_string(&self) -> String {
        let whole = self.0 / Self::BASIS_POINTS;
        let frac = self.0 % Self::BASIS_POINTS;

        if frac == 0 {
            whole.to_string()
        } else {
            // Handle up to 4 decimal places
            let mut frac_str = format!("{:04}", frac);
            // Trim trailing zeros but keep necessary decimal places
            frac_str = frac_str.trim_end_matches('0').to_string();
            format!("{}.{}", whole, frac_str)
        }
    }

    /// Returns the raw basis points value
    pub fn basis_points(&self) -> i32 {
        self.0
    }

    pub fn to_output<S: ScalarValue>(&self) -> Value<S> {
        Value::scalar(self.to_string())
    }

    pub fn from_input<S: ScalarValue>(v: &InputValue<S>) -> Result<Self, String> {
        let s = v.as_string_value().ok_or("Expected a string")?;
        Self::from_str(s)
    }
}

impl FromSql<Integer, Sqlite> for Percentage {
    fn from_sql(bytes: SqliteValue<'_, '_, '_>) -> deserialize::Result<Self> {
        let basis_points = <i32 as FromSql<Integer, Sqlite>>::from_sql(bytes)?;
        Ok(Self(basis_points))
    }
}

impl ToSql<Integer, Sqlite> for Percentage {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        <i32 as ToSql<Integer, Sqlite>>::to_sql(&self.0, out)
    }
}

impl Queryable<Integer, Sqlite> for Percentage {
    type Row = i32;

    fn build(row: Self::Row) -> deserialize::Result<Self> {
        Ok(Percentage(row))
    }
}

impl From<String> for Percentage {
    fn from(s: String) -> Self {
        Self::from_str(&s).unwrap()
    }
}

impl Add for Percentage {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl Sum for Percentage {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::from("0".to_string()), |a, b| a + b)
    }
}

impl Sub for Percentage {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

impl Mul<i32> for Percentage {
    type Output = Self;

    fn mul(self, other: i32) -> Self::Output {
        Self(self.0 * other)
    }
}

impl Div<i32> for Percentage {
    type Output = Self;

    fn div(self, other: i32) -> Self::Output {
        Self(self.0 / other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percentage_creation() {
        // From integer basis points
        let p = Percentage(50000);
        assert_eq!(p.0, 50000);
        assert_eq!(p.to_string(), "5");

        let p = Percentage(25000);
        assert_eq!(p.0, 25000);
        assert_eq!(p.to_string(), "2.5");
    }

    #[test]
    fn test_percentage_from_float() {
        // Test zero
        let p = Percentage::from_float(0.0);
        assert_eq!(p.0, 0);
        assert_eq!(p.to_string(), "0");

        // Test whole numbers
        let p = Percentage::from_float(5.0);
        assert_eq!(p.0, 50000);
        assert_eq!(p.to_string(), "5");

        // Test simple decimals
        let p = Percentage::from_float(2.5);
        assert_eq!(p.0, 25000);
        assert_eq!(p.to_string(), "2.5");

        // Test precision with more decimal places
        let p = Percentage::from_float(2.375);
        assert_eq!(p.0, 23750);
        assert_eq!(p.to_string(), "2.375");

        let p = Percentage::from_float(0.3445);
        assert_eq!(p.0, 3445);
        assert_eq!(p.to_string(), "0.3445");

        // Test rounding behavior
        let p = Percentage::from_float(4.99991);
        assert_eq!(p.0, 49999);
        assert_eq!(p.to_string(), "4.9999");

        let p = Percentage::from_float(4.99996);
        assert_eq!(p.0, 50000);
        assert_eq!(p.to_string(), "5");

        // Test very small numbers
        let p = Percentage::from_float(0.0004);
        assert_eq!(p.0, 4);
        assert_eq!(p.to_string(), "0.0004");

        // Test larger numbers
        let p = Percentage::from_float(99.99);
        assert_eq!(p.0, 999900);
        assert_eq!(p.to_string(), "99.99");
    }

    #[test]
    fn test_percentage_from_str() {
        // Test zero
        let p = Percentage::from_str("0").unwrap();
        assert_eq!(p.0, 0);
        assert_eq!(p.to_string(), "0");

        // Test whole numbers
        let p = Percentage::from_str("5").unwrap();
        assert_eq!(p.0, 50000);
        assert_eq!(p.to_string(), "5");

        // Test simple decimals
        let p = Percentage::from_str("2.5").unwrap();
        assert_eq!(p.0, 25000);
        assert_eq!(p.to_string(), "2.5");

        // Test precision with more decimal places
        let p = Percentage::from_str("2.375").unwrap();
        assert_eq!(p.0, 23750);
        assert_eq!(p.to_string(), "2.375");

        // Test rounding behavior
        let p = Percentage::from_str("4.99991").unwrap();
        assert_eq!(p.0, 49999);
        assert_eq!(p.to_string(), "4.9999");

        let p = Percentage::from_str("4.99996").unwrap();
        assert_eq!(p.0, 50000);
        assert_eq!(p.to_string(), "5");

        // Test trailing zeros
        let p = Percentage::from_str("2.500").unwrap();
        assert_eq!(p.0, 25000);
        assert_eq!(p.to_string(), "2.5");

        let p = Percentage::from_str("2.0").unwrap();
        assert_eq!(p.0, 20000);
        assert_eq!(p.to_string(), "2");
    }

    #[test]
    fn test_percentage_arithmetic() {
        let p1 = Percentage::from_str("4.75").unwrap(); // 4.75%
        let p2 = Percentage::from_str("2.25").unwrap(); // 2.25%

        assert_eq!((p1 + p2).to_string(), "7"); // 7%
        assert_eq!((p1 - p2).to_string(), "2.5"); // 2.5%
        assert_eq!((p1 * 2).to_string(), "9.5"); // 9.5%
        assert_eq!((p1 / 2).to_string(), "2.375"); // 2.375%
    }

    #[test]
    fn test_percentage_sum() {
        let percentages = vec![
            Percentage::from_str("4.0").unwrap(),
            Percentage::from_str("3.0").unwrap(),
        ];
        let sum: Percentage = percentages.into_iter().sum();
        assert_eq!(sum.to_string(), "7");
    }

    #[test]
    fn test_percentage_decimal_places() {
        assert_eq!(Percentage(47500).to_string(), "4.75");
        assert_eq!(Percentage(47000).to_string(), "4.7");
        assert_eq!(Percentage(47070).to_string(), "4.707");
        assert_eq!(Percentage(40000).to_string(), "4");
        assert_eq!(Percentage(4000).to_string(), "0.4");
        assert_eq!(Percentage(400).to_string(), "0.04");
        assert_eq!(Percentage(40).to_string(), "0.004");
        assert_eq!(Percentage(4).to_string(), "0.0004");
    }
}
