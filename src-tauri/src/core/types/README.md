# Core Types

Core types for type-safe domain modeling with database and GraphQL integration.

## Implementation Requirements

### 1. Type Safety and Traits
Core type definition with required traits for compile-time safety and database integration. Use newtype pattern to wrap primitive types.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, AsExpression)]
#[diesel(sql_type = SqlType)]
pub struct NewType(InnerType);
```

### 2. Database Integration
Required traits for database operations. Queryable for high-level queries, FromSql for raw SQL and aggregates, ToSql for writing to database.

```rust
// For high-level query interface (select, first)
impl Queryable<SqlType, Sqlite> for NewType {
    type Row = InnerType;
    fn build(row: Self::Row) -> deserialize::Result<Self> {
        Ok(Self(row))
    }
}

// For low-level value conversion (raw SQL, aggregates)
impl FromSql<SqlType, Sqlite> for NewType {
    fn from_sql(value: SqliteValue) -> deserialize::Result<Self> {
        let raw_value = <InnerType as FromSql<SqlType, Sqlite>>::from_sql(value)?;
        Ok(Self(raw_value))
    }
}

// For writing to database
impl ToSql<SqlType, Sqlite> for NewType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        self.0.to_sql(out)
    }
}
```

### 3. GraphQL Integration
Three approaches for GraphQL integration: transparent for simple wrappers, macro for basic customization, manual for complex parsing.

Choose one approach:

1. Transparent (for wrapper types):
```rust
#[graphql_scalar]
#[graphql(transparent)]
pub struct NewType(InnerType);
```

2. Custom with macro:
```rust
#[graphql_scalar(
    name = "TypeName",
    description = "Description",
    parse_token(String)
)]
pub struct NewType(InnerType);
```

3. Manual implementation:
```rust
#[graphql_scalar]
impl NewType {
    fn to_output<S: ScalarValue>(&self) -> Value<S> {
        Value::scalar(self.0.to_string())
    }

    fn from_input<S>(v: &InputValue<S>) -> Result<Self, String>
    where S: ScalarValue {
        v.as_string_value()
            .ok_or_else(|| "Expected string".to_string())
            .and_then(|s| s.parse().map_err(|e| e.to_string()))
            .map(Self)
    }
}
```

### 4. Type Conversions
Implement From for infallible conversions and TryFrom for fallible conversions. Essential for type safety and ergonomic API.

```rust
// For infallible conversions
impl From<OtherType> for NewType {
    fn from(value: OtherType) -> Self {
        Self(value.into())
    }
}

// For fallible conversions
impl TryFrom<OtherType> for NewType {
    type Error = String;
    fn try_from(value: OtherType) -> Result<Self, Self::Error> {
        // Conversion logic
    }
}
```

### 5. Testing Requirements
Test all aspects of the type including core functionality, database operations, GraphQL integration, and edge cases. Place tests in cfg(test) module.

Place in `#[cfg(test)]` module:
- Core functionality
- Database roundtrip
- GraphQL serialization
- Edge cases
- Type conversions
- String parsing
