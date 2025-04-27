use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use std::str::FromStr;

use crate::{
    adapters::outgoing::database::FromRow,
    error::{Error, Result},
};

use super::FromLibsqlValue;

// Implementation of FromRow for BigDecimal
impl FromRow<libsql::Row> for BigDecimal {
    fn from_row(row: &libsql::Row) -> Result<Self> {
        // Assuming the BigDecimal value is in the first column as a string
        let value_str = row.get::<String>(0)
            .map_err(|e| Error::DatabaseError(format!("Failed to get BigDecimal value: {}", e)))?;

        BigDecimal::from_str(&value_str)
            .map_err(|e| Error::DatabaseError(format!("Failed to parse BigDecimal: {}", e)))
    }
}

// Implementation of FromRow for i64
impl FromRow<libsql::Row> for i64 {
    fn from_row(row: &libsql::Row) -> Result<Self> {
        // Try to get as integer first
        match row.get::<i64>(0) {
            Ok(value) => Ok(value),
            // If that fails, try to get as string and parse
            Err(_) => {
                let value_str = row.get::<String>(0)
                    .map_err(|e| Error::DatabaseError(format!("Failed to get i64 value: {}", e)))?;

                value_str.parse::<i64>()
                    .map_err(|e| Error::DatabaseError(format!("Failed to parse i64: {}", e)))
            }
        }
    }
}

impl FromLibsqlValue for String {
    fn from_libsql_value(value: libsql::Value) -> Result<Self> {
        match value {
            libsql::Value::Text(s) => Ok(s.clone()),
            _ => Err(Error::DatabaseError("Invalid string value type in database".to_string())),
        }
    }
}

impl FromLibsqlValue for NaiveDateTime {
    fn from_libsql_value(value: libsql::Value) -> Result<Self> {
        match value {
            libsql::Value::Text(s) => {
                println!("NaiveDateTime value: {}", s);
                NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S%.f")
                    .map_err(|e| Error::DatabaseError(format!("Failed to parse NaiveDateTime: {}", e)))
            },
            _ => Err(Error::DatabaseError("Invalid NaiveDateTime value type in database".to_string())),
        }
    }
}
