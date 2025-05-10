# Migration Guide: Diesel to SeaQuery

This document outlines the process for migrating command files from using Diesel ORM directly to using SeaQuery for query building with the database adapter pattern.

## Background

The project is transitioning from:
- Direct Diesel ORM usage with `service.conn` for database operations
- Manual SQL query construction in some places

To:
- SeaQuery for type-safe query building
- Database adapter pattern with `service.db_adapter` for database operations
- Better abstraction to support multiple database backends (e.g., SQLite, libsql)

## Migration Steps

### 1. Add SeaQuery Identifier Enum to Model Files

For each model, add a SeaQuery `Iden` enum in the corresponding model file:

```rust
use sea_query::Iden;

#[derive(Iden)]
pub enum TableName {
    Table,
    Id,
    // Add all column names as enum variants
    Column1,
    Column2,
    // ...
}
```

Make sure to add this enum for all related tables that might be used in queries, including join tables.

### 2. Add Display Trait to State Enums

For any state enums that need to be converted to strings in queries:

```rust
use derive_more::Display;

#[derive(Debug, Clone, Copy, DbEnum, GraphQLEnum, PartialEq, Eq, Display)]
#[DbValueStyle = "snake_case"]
pub enum SomeState {
    Active,
    Inactive,
    // ...
}
```

The `Display` trait is essential for converting enum values to strings in SeaQuery expressions.

### 3. Update Command Imports

Replace Diesel imports with SeaQuery imports:

```rust
// Remove
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use crate::schema::table_name::dsl::*;

// Add
use sea_query::{Expr, Query, SqliteQueryBuilder};
use crate::adapters::outgoing::database::DatabaseAdapter;
use crate::core::models::some_model::TableName; // The Iden enum
```

Make sure to import all the necessary SeaQuery types and the Iden enums for all tables used in the queries.

### 4. Refactor Command Implementations

#### 4.1 Create/Insert Operations

```rust
// Old Diesel approach
let created_item = diesel::insert_into(table)
    .values(&new_item)
    .returning(Item::as_returning())
    .get_result(conn)?;

// New SeaQuery approach
let insert_query = Query::insert()
    .into_table(TableName::Table)
    .columns([
        TableName::Id,
        TableName::Name,
        // ...other columns
    ])
    .values_panic([
        item_id.to_string().into(),
        self.item.name.clone().into(),
        // ...other values
    ])
    .to_string(SqliteQueryBuilder);

service.db_adapter.execute(&insert_query, vec![])?;

// Return the created item
let new_item = Item {
    id: item_id,
    name: self.item.name.clone(),
    // ...other fields
};
```

For insert operations, always use the `values_panic` method rather than `values`. The `values_panic` method will panic if the number of values doesn't match the number of columns, which helps catch errors at runtime:

```rust
// Correct approach - use values_panic
let insert_query = Query::insert()
    .into_table(TableName::Table)
    .columns([
        TableName::Id,
        TableName::Name,
        // ...other columns
    ])
    .values_panic([  // This will panic if the number of values doesn't match columns
        item_id.to_string().into(),
        self.item.name.clone().into(),
        // ...other values
    ])
    .to_string(SqliteQueryBuilder);
```

Note that SeaQuery doesn't have a direct equivalent to Diesel's `returning` clause, so you'll need to manually construct the returned object or perform a separate select query.

#### 4.2 Read/Select Operations

```rust
// Old Diesel approach
let item = table
    .filter(id.eq(&self.id))
    .select(Item::as_select())
    .first::<Item>(conn)?;

// New SeaQuery approach
let select_query = Query::select()
    .from(TableName::Table)
    .columns([
        TableName::Id,
        TableName::Name,
        // ...other columns
    ])
    .and_where(Expr::col(TableName::Id).eq(self.id.to_string()))
    .to_string(SqliteQueryBuilder);

let item = service.db_adapter.query_one::<Item>(&select_query, vec![])?;
```

Make sure to include all columns that are needed for the model. For complex queries with joins, you may need to use aliases and select specific columns.

#### 4.3 Update Operations

```rust
// Old Diesel approach
let updated_item = diesel::update(table.filter(id.eq(&self.item.id)))
    .set(&item_changeset)
    .returning(Item::as_returning())
    .get_result(conn)?;

// New SeaQuery approach
let mut query = Query::update();
query.table(TableName::Table)
    .value(TableName::UpdatedAt, now.to_string());

// Add optional fields if they exist
if let Some(name) = &self.item.name {
    query.value(TableName::Name, name.clone());
}
// ...other optional fields

query.and_where(Expr::col(TableName::Id).eq(self.item.id.to_string()));
let sql = query.to_string(SqliteQueryBuilder);

service.db_adapter.execute(&sql, vec![])?;

// Retrieve the updated item
let select_query = Query::select()
    .from(TableName::Table)
    .columns([
        TableName::Id,
        TableName::Name,
        // ...other columns
    ])
    .and_where(Expr::col(TableName::Id).eq(self.item.id.to_string()))
    .to_string(SqliteQueryBuilder);

let item = service.db_adapter.query_one::<Item>(&select_query, vec![])?;
```

For update operations, you'll need to handle each field individually, especially for optional fields.

#### 4.4 Delete Operations

```rust
// Old Diesel approach
let num_deleted = diesel::delete(table.filter(id.eq(&self.id))).execute(conn)?;

// New SeaQuery approach
let delete_query = Query::delete()
    .from_table(TableName::Table)
    .and_where(Expr::col(TableName::Id).eq(self.id.to_string()))
    .to_string(SqliteQueryBuilder);

let affected_rows = service.db_adapter.execute(&delete_query, vec![])?;
```

Delete operations are straightforward, but remember to handle related records if necessary (e.g., cascading deletes or manual deletion of related records).

#### 4.5 Existence Checks

```rust
// Old Diesel approach
let existing = table
    .filter(name.eq(&self.item.name))
    .count()
    .get_result::<i64>(conn)?;
if existing > 0 {
    return Err(Error::UniqueConstraintError);
}

// New SeaQuery approach
let check_query = Query::select()
    .from(TableName::Table)
    .column(TableName::Id)
    .and_where(Expr::col(TableName::Name).eq(self.item.name.clone()))
    .to_string(SqliteQueryBuilder);

let existing = service.db_adapter.query_optional::<Item>(&check_query, vec![])?;
if existing.is_some() {
    return Err(Error::UniqueConstraintError);
}
```

For existence checks, you can use `query_optional` and check if the result is `Some` or `None`.

#### 4.6 Transactions

```rust
// Old Diesel approach
service.conn.transaction(|conn| {
    // Transaction operations
    // ...
    Ok(result)
})?;

// New SeaQuery approach
service.db_adapter.transaction(|db| {
    // Transaction operations
    // ...
    Ok(result)
})?;
```

The database adapter should provide a transaction method that takes a closure, similar to Diesel's transaction method.

### 5. Error Handling

Update error handling to match the new database adapter pattern:

```rust
// Old Diesel approach
if result == 0 {
    Err(Error::NotFoundError)
} else {
    Ok(result)
}

// New SeaQuery approach
if affected_rows == 0 {
    Err(Error::NotFoundError)
} else {
    Ok(affected_rows as usize)
}
```

Be consistent with error handling across all commands. Consider creating helper functions for common error patterns.

### 6. Handling NULL Values

For optional fields that might be NULL:

```rust
// For insert operations with optional fields
match &self.item.description {
    Some(desc) => desc.clone().into(),
    None => sea_query::Value::String(None).into(),
},

// For update operations with nested optionals (Option<Option<T>>)
if let Some(description) = &self.item.description {
    match description {
        Some(desc) => query.value(TableName::Description, desc.clone()),
        None => query.value(TableName::Description, sea_query::Value::String(None)),
    };
}
```

It's important to use `sea_query::Value::String(None).into()` rather than `"NULL".into()` or other approaches, as this properly represents NULL values in the database.

Be careful with NULL values, especially for nested optionals (Option<Option<T>>).

### 7. Updating Tests

Tests need to be updated to use the database adapter pattern instead of direct Diesel operations:

#### 7.1 Test Setup Helper Functions

```rust
// Old Diesel approach
fn create_test_entity(service: &mut AppService) -> Entity {
    let entity = Entity {
        id: Uuid::now_v7().into(),
        name: "Test Entity".to_string(),
        // ...other fields
    };

    diesel::insert_into(schema::entities::table)
        .values(&entity)
        .execute(&mut service.conn)
        .unwrap();

    entity
}

// New SeaQuery approach
fn create_test_entity(service: &mut AppService) -> Entity {
    let entity_id: DbUuid = Uuid::now_v7().into();
    let now = Utc::now().naive_utc();

    let insert_query = Query::insert()
        .into_table(Entities::Table)
        .columns([
            Entities::Id,
            Entities::Name,
            // ...other columns
        ])
        .values_panic([
            entity_id.to_string().into(),
            "Test Entity".into(),
            // ...other values
        ])
        .to_string(SqliteQueryBuilder);

    service.db_adapter.execute(&insert_query, vec![]).unwrap();

    Entity {
        id: entity_id,
        name: "Test Entity".to_string(),
        // ...other fields
    }
}
```

Create helper functions for test setup to avoid duplicating code and make tests more maintainable.

#### 7.2 Test Verification

```rust
// Old Diesel approach
let associations = schema::entity_relations::table
    .filter(schema::entity_relations::entity_id.eq(entity.id))
    .load::<EntityRelation>(&mut service.conn)
    .unwrap();

// New SeaQuery approach
let select_query = Query::select()
    .from(EntityRelations::Table)
    .columns([
        EntityRelations::EntityId,
        EntityRelations::RelatedId,
        // ...other columns
    ])
    .and_where(Expr::col(EntityRelations::EntityId).eq(entity.id.to_string()))
    .to_string(SqliteQueryBuilder);

let associations = service.db_adapter.query_many::<EntityRelation>(&select_query, vec![]).unwrap();
```

Update all test verification code to use the database adapter pattern.

#### 7.3 Test Cleanup

If your tests perform cleanup operations, make sure to update those as well:

```rust
// Old Diesel approach
diesel::delete(schema::entities::table)
    .execute(&mut service.conn)
    .unwrap();

// New SeaQuery approach
let delete_query = Query::delete()
    .from_table(Entities::Table)
    .to_string(SqliteQueryBuilder);

service.db_adapter.execute(&delete_query, vec![]).unwrap();
```

## Testing

After migration, test each command to ensure it works correctly:

1. Run `cargo check` to verify compilation
2. Run unit tests for the migrated commands
3. Test the commands through the GraphQL API
4. Verify that all functionality works as expected, especially edge cases

## Common Issues

1. **Type Annotations**: You may need to add explicit type annotations for IDs and other values:
   ```rust
   let item_id: DbUuid = Uuid::now_v7().into();
   ```

2. **Display Trait**: Ensure all enums used in queries have the `Display` trait implemented.

3. **NULL Handling**: Be careful with optional fields and NULL values in the database.

4. **Error Mapping**: Update error handling to match the new database adapter pattern.

5. **Test Failures**: Tests may fail if they still use Diesel directly. Update all test code to use the database adapter pattern.

6. **String Conversion**: Remember that SeaQuery often requires converting values to strings:
   ```rust
   item_id.to_string().into()
   ```

7. **Column Names**: Make sure the column names in the Iden enum match the actual column names in the database.

8. **Missing Imports**: Double-check that all necessary imports are included, especially for the Iden enums and the DatabaseAdapter trait:
   ```rust
   use crate::adapters::outgoing::database::DatabaseAdapter;
   ```
   This trait is required for accessing the database adapter methods like `query_one`, `query_many`, `execute`, and `transaction`.

## Example Migration

See the following files for examples of completed migrations:
- `src/core/commands/auth/user_commands.rs`
- `src/core/commands/catalog/discount_commands.rs`
- `src/core/commands/catalog/item_commands.rs`

## Migration Strategy

Consider the following strategy for migrating the entire codebase:

1. Start with simpler models that have fewer relationships
2. Migrate one command file at a time
3. Update tests for each command file immediately after migrating it
4. Run tests frequently to catch issues early
5. Update GraphQL queries and mutations after migrating the related command files
6. Perform integration testing after migrating all related files

## Future Considerations

1. **Transaction Support**: The current implementation doesn't support transactions directly. This will need to be addressed in the database adapter.

2. **Query Parameters**: Consider using parameterized queries instead of string interpolation for better security.

3. **Batch Operations**: Implement support for batch operations in the database adapter.

4. **Migration Testing**: Create comprehensive tests to verify the correctness of migrations.

5. **Performance Optimization**: After migration, analyze query performance and optimize as needed.

6. **Error Handling Improvements**: Consider more sophisticated error handling and recovery mechanisms.

7. **Database Abstraction**: Further abstract the database layer to support different database backends more easily.

8. **Documentation**: Keep this migration guide updated as new patterns and best practices emerge.
