use std::{
    iter::Sum,
    ops::{Add, Div, Mul, Sub},
};

use bigdecimal::{BigDecimal, ToPrimitive};
use juniper::{graphql_scalar, InputValue, ScalarValue, Value};
use lightning_macros::{LibsqlType, SeaQueryType};

use crate::adapters::outgoing::database::FromLibsqlValue;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, SeaQueryType, LibsqlType)]
#[graphql_scalar(parse_token(String))]
pub struct Money(i64);

impl Money {
    pub const BASE_UNIT: i64 = 100; // For 2 decimal places (cents)

    /// Creates a new Money from a float value
    /// Example: 10.99 becomes 1099 cents ($10.99)
    /// Rounds to nearest cent
    pub fn from_float(value: f64) -> Self {
        Self((value * Self::BASE_UNIT as f64).round() as i64)
    }

    /// Creates a new Money from a string representation
    /// Example: "10.99" becomes 1099 cents ($10.99)
    /// Rounds to nearest cent
    pub fn from_str(s: &str) -> Result<Self, String> {
        let value = s.parse::<f64>().map_err(|e| e.to_string())?;

        // Convert to cents with rounding
        let cents = (value * Self::BASE_UNIT as f64).round() as i64;
        Ok(Self(cents))
    }

    /// Returns the money value as a string without currency symbol
    /// Example: 1099 cents becomes "10.99"
    pub fn to_string(&self) -> String {
        let dollars = self.0 / Self::BASE_UNIT;
        let cents = self.0 % Self::BASE_UNIT;

        if cents == 0 {
            dollars.to_string()
        } else {
            // Format with 2 decimal places
            format!("{}.{:02}", dollars, cents.abs())
        }
    }

    /// Returns the raw cents value
    pub fn cents(&self) -> i64 {
        self.0
    }

    fn to_output<S: ScalarValue>(&self) -> Value<S> {
        Value::scalar(self.to_string())
    }

    fn from_input<S>(v: &InputValue<S>) -> Result<Self, String>
    where
        S: ScalarValue,
    {
        let s = v
            .as_string_value()
            .ok_or_else(|| "Expected a string value".to_string())?;
        Self::from_str(s)
    }
}

impl From<i64> for Money {
    fn from(value: i64) -> Self {
        Money(value)
    }
}

// For aggregate result from database
impl From<BigDecimal> for Money {
    fn from(value: BigDecimal) -> Self {
        Money(value.to_i64().unwrap())
    }
}

impl From<String> for Money {
    fn from(s: String) -> Self {
        Self::from_str(&s).unwrap()
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

    #[test]
    fn test_sum() {
        let v = vec![
            Money::from_str("10").unwrap(),
            Money::from_str("5").unwrap(),
        ];
        assert_eq!(v.into_iter().sum::<Money>(), Money(1500));
    }

    #[test]
    fn test_money_from_float() {
        // Test zero
        let m = Money::from_float(0.0);
        assert_eq!(m.0, 0);
        assert_eq!(m.to_string(), "0");

        // Test whole numbers
        let m = Money::from_float(10.0);
        assert_eq!(m.0, 1000);
        assert_eq!(m.to_string(), "10");

        // Test with cents
        let m = Money::from_float(10.99);
        assert_eq!(m.0, 1099);
        assert_eq!(m.to_string(), "10.99");

        // Test rounding behavior
        let m = Money::from_float(10.994);
        assert_eq!(m.0, 1099);
        assert_eq!(m.to_string(), "10.99");

        let m = Money::from_float(10.995);
        assert_eq!(m.0, 1100);
        assert_eq!(m.to_string(), "11");

        // Test negative values
        let m = Money::from_float(-5.75);
        assert_eq!(m.0, -575);
        assert_eq!(m.to_string(), "-5.75");
    }

    #[test]
    fn test_money_from_str() {
        // Test parsing valid strings
        let m = Money::from_str("0").unwrap();
        assert_eq!(m.0, 0);
        assert_eq!(m.to_string(), "0");

        let m = Money::from_str("10").unwrap();
        assert_eq!(m.0, 1000);
        assert_eq!(m.to_string(), "10");

        let m = Money::from_str("10.99").unwrap();
        assert_eq!(m.0, 1099);
        assert_eq!(m.to_string(), "10.99");

        let m = Money::from_str("-5.75").unwrap();
        assert_eq!(m.0, -575);
        assert_eq!(m.to_string(), "-5.75");

        // Test parsing invalid strings
        assert!(Money::from_str("abc").is_err());
        assert!(Money::from_str("10.abc").is_err());
    }

    #[test]
    fn test_money_to_string() {
        // Test zero
        let m = Money(0);
        assert_eq!(m.to_string(), "0");

        // Test whole numbers
        let m = Money(1000);
        assert_eq!(m.to_string(), "10");

        // Test with cents
        let m = Money(1099);
        assert_eq!(m.to_string(), "10.99");

        let m = Money(1001);
        assert_eq!(m.to_string(), "10.01");

        // Test negative values
        let m = Money(-575);
        assert_eq!(m.to_string(), "-5.75");
    }
}
