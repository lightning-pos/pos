use std::ops::{Add, Div, Mul, Sub};

/// Represents a monetary amount in a specific currency.
///
/// # Fields
/// - `base_units`: Amount in the smallest unit of the currency (e.g., cents for USD, paise for INR)
/// - `currency_code`: ISO 4217 currency code (e.g., 'USD', 'INR')
///
/// # Examples
/// ```
/// let m1 = Money::new(10000, "USD");
/// let m2 = Money::new(5000, "USD");
///
/// let sum = m1 + m2;
/// let diff = m1 - m2;
/// let doubled = m1 * 2;
/// let halved = m1 / 2;
/// ```
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Money {
    base_units: i64,
    currency_code: CurrencyCode,
}

impl Money {
    fn new(base_units: i64, currency_code: &str) -> Self {
        let code = CurrencyCode::from_str(currency_code).expect("Invalid currency code.");
        Self {
            base_units,
            currency_code: code,
        }
    }

    fn to_string(&self) -> String {
        format!(
            "{} {}",
            self.base_units as f64 / 100.0,
            self.currency_code.as_str()
        )
    }
}

/// Enum representing different currencies.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CurrencyCode {
    INR,
    USD,
    EUR,
    GBP,
    JPY,
    CNY,
    // Add other currencies as needed
}

impl CurrencyCode {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "INR" => Some(Self::INR),
            "USD" => Some(Self::USD),
            "EUR" => Some(Self::EUR),
            "GBP" => Some(Self::GBP),
            "JPY" => Some(Self::JPY),
            "CNY" => Some(Self::CNY),
            _ => None,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            Self::INR => "INR",
            Self::USD => "USD",
            Self::EUR => "EUR",
            Self::GBP => "GBP",
            Self::JPY => "JPY",
            Self::CNY => "CNY",
        }
    }
}

impl Add for Money {
    type Output = Money;

    fn add(self, other: Money) -> Money {
        assert_eq!(
            self.currency_code, other.currency_code,
            "Cannot add money with different currencies"
        );
        Money {
            base_units: self
                .base_units
                .checked_add(other.base_units)
                .expect("Integer overflow in addition"),
            currency_code: self.currency_code,
        }
    }
}

impl Sub for Money {
    type Output = Money;

    fn sub(self, other: Money) -> Money {
        assert_eq!(
            self.currency_code, other.currency_code,
            "Cannot subtract money with different currencies"
        );
        Money {
            base_units: self
                .base_units
                .checked_sub(other.base_units)
                .expect("Integer underflow in subtraction"),
            currency_code: self.currency_code,
        }
    }
}

impl Mul<i64> for Money {
    type Output = Money;

    fn mul(self, factor: i64) -> Money {
        Money {
            base_units: self
                .base_units
                .checked_mul(factor)
                .expect("Integer overflow in multiplication"),
            currency_code: self.currency_code,
        }
    }
}

impl Div<i64> for Money {
    type Output = Money;

    fn div(self, divisor: i64) -> Money {
        assert_ne!(divisor, 0, "Cannot divide by zero");
        Money {
            base_units: self
                .base_units
                .checked_div(divisor)
                .expect("Integer overflow in division"),
            currency_code: self.currency_code,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money_creation() {
        let m = Money::new(10000, "USD");
        assert_eq!(m.base_units, 10000);
        assert_eq!(m.currency_code.as_str(), "USD");
    }

    #[test]
    #[should_panic(expected = "Invalid currency code.")]
    fn test_invalid_currency() {
        Money::new(100, "INVALID");
    }

    #[test]
    fn test_addition() {
        let usd100 = Money::new(10000, "USD");
        let usd200 = Money::new(20000, "USD");
        let result = usd100 + usd200;
        assert_eq!(result.base_units, 30000);
        assert_eq!(result.currency_code.as_str(), "USD");
    }

    #[test]
    #[should_panic(expected = "Cannot add money with different currencies")]
    fn test_add_different_currencies() {
        let usd100 = Money::new(10000, "USD");
        let eur100 = Money::new(10000, "EUR");
        let _result = usd100 + eur100;
    }

    #[test]
    fn test_subtraction() {
        let usd200 = Money::new(20000, "USD");
        let usd100 = Money::new(10000, "USD");
        let result = usd200 - usd100;
        assert_eq!(result.base_units, 10000);
        assert_eq!(result.currency_code.as_str(), "USD");
    }

    #[test]
    #[should_panic(expected = "Cannot subtract money with different currencies")]
    fn test_subtract_different_currencies() {
        let usd100 = Money::new(10000, "USD");
        let eur100 = Money::new(10000, "EUR");
        let _result = usd100 - eur100;
    }

    #[test]
    fn test_multiplication() {
        let usd100 = Money::new(10000, "USD");
        let result = usd100 * 2;
        assert_eq!(result.base_units, 20000);
        assert_eq!(result.currency_code.as_str(), "USD");
    }

    #[test]
    fn test_division() {
        let usd100 = Money::new(10000, "USD");
        let result = usd100 / 2;
        assert_eq!(result.base_units, 5000);
        assert_eq!(result.currency_code.as_str(), "USD");
    }

    #[test]
    #[should_panic(expected = "Cannot divide by zero")]
    fn test_division_by_zero() {
        let usd100 = Money::new(10000, "USD");
        let _result = usd100 / 0;
    }

    #[test]
    fn test_string_formatting() {
        let usd100 = Money::new(10001, "USD");
        assert_eq!(usd100.to_string(), "100.01 USD");
    }

    #[test]
    fn test_large_amounts() {
        let large_amount = i64::MAX;
        let m = Money::new(large_amount, "USD");
        assert_eq!(m.base_units, large_amount);
    }

    #[test]
    #[should_panic(expected = "Integer overflow in addition")]
    fn test_addition_overflow() {
        let max_money = Money::new(i64::MAX, "USD");
        let positive_money = Money::new(1, "USD");
        let _result = max_money + positive_money;
    }

    #[test]
    #[should_panic(expected = "Integer underflow in subtraction")]
    fn test_subtraction_underflow() {
        let min_money = Money::new(i64::MIN, "USD");
        let positive_money = Money::new(1, "USD");
        let _result = min_money - positive_money;
    }

    #[test]
    #[should_panic(expected = "Integer overflow in multiplication")]
    fn test_multiplication_overflow() {
        let large_money = Money::new(i64::MAX / 2 + 1, "USD");
        let _result = large_money * 2;
    }

    #[test]
    #[should_panic(expected = "Integer overflow in division")]
    fn test_division_overflow() {
        let min_money = Money::new(i64::MIN, "USD");
        let _result = min_money / -1; // This will overflow because MIN/-1 > MAX
    }
}
