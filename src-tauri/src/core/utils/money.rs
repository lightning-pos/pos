/// # Money
///
/// The Money struct represents a monetary amount in a specific currency in the smallest
/// unit of the currency.
///
/// Assertions are used to ensure that the currency code is a 3-letter ISO string, and that
/// the currency code is not empty.
///
/// ## Fields
///
/// * `base_units` - The amount in the smallest unit of the currency (e.g., cents for USD)
/// * `currency_code` - The ISO 4217 currency code (e.g., 'USD')
///
/// ## Usage
///
/// ```
/// let m = Money::new(10000, "USD");
/// ```
/// Add
/// ```
/// let m3 = &m1 + &m2;
/// ```
/// Subtract
/// ```
/// let m3 = &m1 - &m2;
/// ```
/// Multiply
/// ```
/// let m2 = &m1 * 2;
/// ```
/// Divide
/// ```
/// let m2 = &m1 / 2;
/// ```
use std::ops::{Add, Div, Mul, Sub};

struct Money {
    base_units: i64,
    currency_code: String,
}

impl Money {
    fn new(base_units: i64, currency_code: String) -> Self {
        debug_assert!(
            currency_code.len() == 3 && currency_code.chars().all(|c| c.is_ascii_alphabetic()),
            "Currency code must be a 3-letter ISO string"
        );
        Self {
            base_units,
            currency_code: currency_code.to_uppercase(),
        }
    }

    fn to_string(&self) -> String {
        format!("{} {}", self.base_units as f64 / 100.0, self.currency_code)
    }
}

impl Add for &Money {
    type Output = Money;

    fn add(self, other: &Money) -> Money {
        debug_assert_eq!(
            self.currency_code, other.currency_code,
            "Cannot add money with different currencies"
        );
        Money {
            base_units: self
                .base_units
                .checked_add(other.base_units)
                .expect("Integer overflow in addition"),
            currency_code: self.currency_code.clone(),
        }
    }
}

impl Sub for &Money {
    type Output = Money;

    fn sub(self, other: &Money) -> Money {
        debug_assert_eq!(
            self.currency_code, other.currency_code,
            "Cannot subtract money with different currencies"
        );
        Money {
            base_units: self
                .base_units
                .checked_sub(other.base_units)
                .expect("Integer underflow in subtraction"),
            currency_code: self.currency_code.clone(),
        }
    }
}

impl Mul<i64> for &Money {
    type Output = Money;

    fn mul(self, factor: i64) -> Money {
        Money {
            base_units: self
                .base_units
                .checked_mul(factor)
                .expect("Integer overflow in multiplication"),
            currency_code: self.currency_code.clone(),
        }
    }
}

impl Div<i64> for &Money {
    type Output = Money;

    fn div(self, divisor: i64) -> Money {
        debug_assert!(divisor != 0, "Cannot divide by zero");
        Money {
            base_units: self
                .base_units
                .checked_div(divisor)
                .expect("Integer overflow in division"),
            currency_code: self.currency_code.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money_creation() {
        let m = Money::new(10000, "USD".to_string());
        assert_eq!(m.base_units, 10000);
        assert_eq!(m.currency_code, "USD");
    }

    #[test]
    #[should_panic(expected = "Currency code must be a 3-letter ISO string")]
    fn test_invalid_currency() {
        Money::new(100, "INVALID".to_string());
    }

    #[test]
    fn test_addition() {
        let usd100 = Money::new(10000, "USD".to_string());
        let usd200 = Money::new(20000, "USD".to_string());
        let result = &usd100 + &usd200;
        assert_eq!(result.base_units, 30000);
        assert_eq!(result.currency_code, "USD");
    }

    #[test]
    #[should_panic(expected = "Cannot add money with different currencies")]
    fn test_add_different_currencies() {
        let usd100 = Money::new(10000, "USD".to_string());
        let eur100 = Money::new(10000, "EUR".to_string());
        let _result = &usd100 + &eur100;
    }

    #[test]
    fn test_subtraction() {
        let usd200 = Money::new(20000, "USD".to_string());
        let usd100 = Money::new(10000, "USD".to_string());
        let result = &usd200 - &usd100;
        assert_eq!(result.base_units, 10000);
        assert_eq!(result.currency_code, "USD");
    }

    #[test]
    #[should_panic(expected = "Cannot subtract money with different currencies")]
    fn test_subtract_different_currencies() {
        let usd100 = Money::new(10000, "USD".to_string());
        let eur100 = Money::new(10000, "EUR".to_string());
        let _result = &usd100 - &eur100;
    }

    #[test]
    fn test_multiplication() {
        let usd100 = Money::new(10000, "USD".to_string());
        let result = &usd100 * 2;
        assert_eq!(result.base_units, 20000);
        assert_eq!(result.currency_code, "USD");
    }

    #[test]
    fn test_division() {
        let usd100 = Money::new(10000, "USD".to_string());
        let result = &usd100 / 2;
        assert_eq!(result.base_units, 5000);
        assert_eq!(result.currency_code, "USD");
    }

    #[test]
    #[should_panic(expected = "Cannot divide by zero")]
    fn test_division_by_zero() {
        let usd100 = Money::new(10000, "USD".to_string());
        let _result = &usd100 / 0;
    }

    #[test]
    fn test_string_formatting() {
        let usd100 = Money::new(10001, "USD".to_string());
        assert_eq!(usd100.to_string(), "100.01 USD");
    }

    #[test]
    fn test_large_amounts() {
        let large_amount = i64::MAX;
        let m = Money::new(large_amount, "USD".to_string());
        assert_eq!(m.base_units, large_amount);
    }

    #[test]
    #[should_panic(expected = "Integer overflow in addition")]
    fn test_addition_overflow() {
        let max_money = Money::new(i64::MAX, "USD".to_string());
        let positive_money = Money::new(1, "USD".to_string());
        let _result = &max_money + &positive_money;
    }

    #[test]
    #[should_panic(expected = "Integer underflow in subtraction")]
    fn test_subtraction_underflow() {
        let min_money = Money::new(i64::MIN, "USD".to_string());
        let positive_money = Money::new(1, "USD".to_string());
        let _result = &min_money - &positive_money;
    }

    #[test]
    #[should_panic(expected = "Integer overflow in multiplication")]
    fn test_multiplication_overflow() {
        let large_money = Money::new(i64::MAX / 2 + 1, "USD".to_string());
        let _result = &large_money * 2;
    }

    #[test]
    #[should_panic(expected = "Integer overflow in division")]
    fn test_division_overflow() {
        let min_money = Money::new(i64::MIN, "USD".to_string());
        let _result = &min_money / -1; // This will overflow because MIN/-1 > MAX
    }
}
