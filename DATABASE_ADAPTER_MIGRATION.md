# Database Adapter Migration Guide

This document provides a comprehensive guide for migrating command implementations from the old database adapter pattern to the new one. The migration involves several key changes to improve code quality, maintainability, and performance.

## Table of Contents

1. [Overview](#overview)
2. [Key Changes](#key-changes)
3. [Migration Steps](#migration-steps)
4. [Examples](#examples)
5. [Common Issues and Solutions](#common-issues-and-solutions)
6. [Testing](#testing)

## Overview

The database adapter interface has been updated to use sea-query statements directly instead of raw SQL strings and parameters. This change improves type safety, query construction, and maintainability. Additionally, the Command trait implementations now use async/await for better performance with database operations.

## Key Changes

1. **Command Trait Implementation**
   - Changed from synchronous to asynchronous
   - `fn exec(&self, service: &mut AppService) -> Result<Self::Output>` becomes `async fn exec(&self, service: &mut AppService) -> Result<Self::Output>`

2. **Database Adapter Methods**
   - Now accept sea-query statements directly instead of SQL strings and parameters
   - Specific methods for different operations (insert_one, update_one, delete, etc.) instead of generic execute
   - All methods are now async

3. **Transaction Handling**
   - Removed transaction blocks where not necessary
   - Direct database operations instead of wrapping in transactions

4. **Query Construction**
   - Use intermediate variables to avoid temporary value issues
   - Pass sea-query statements directly to database adapter methods

## Migration Steps

Follow these steps to migrate your command implementations:

### 1. Update Imports

Remove unnecessary imports and add required ones:

```rust
// Old
use sea_query::{Expr, Query, SqliteQueryBuilder};

// New
use sea_query::{Expr, Query};
```

### 2. Update Command Trait Implementation

Change the exec method signature to be async:

```rust
// Old
fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
    // ...
}

// New
async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
    // ...
}
```

### 3. Replace Transaction Blocks

Remove unnecessary transaction blocks and use direct database operations:

```rust
// Old
service.db_adapter.transaction(|db| {
    // Database operations
    Ok(result)
})

// New
// Direct database operations
Ok(result)
```

If transactions are necessary, update them to use async/await:

```rust
// Old
service.db_adapter.transaction(|db| {
    // Database operations
    Ok(result)
})

// New
service.db_adapter.transaction(|db| Box::pin(async move {
    // Database operations
    Ok(result)
})).await
```

### 4. Update Query Construction

Use intermediate variables to avoid temporary value issues:

```rust
// Old
let query = Query::select()
    .from(Table::Table)
    .columns([...])
    .and_where(Expr::col(Table::Id).eq(id.to_string()))
    .to_string(SqliteQueryBuilder);

// New
let mut query_builder = Query::select();
let select_stmt = query_builder
    .from(Table::Table)
    .columns([...])
    .and_where(Expr::col(Table::Id).eq(id.to_string()));
```

### 5. Update Database Adapter Method Calls

Use the appropriate database adapter methods for each operation:

```rust
// Old - Select
let result = db.query_optional::<Entity>(&query, vec![])?;

// New - Select
let result = service.db_adapter.query_optional::<Entity>(&select_stmt).await?;

// Old - Insert
let sql = query.to_string(SqliteQueryBuilder);
db.execute(&sql, vec![])?;

// New - Insert
service.db_adapter.insert_many(&insert_stmt).await?;

// Old - Update
let sql = update.to_string(SqliteQueryBuilder);
db.execute(&sql, vec![])?;

// New - Update
service.db_adapter.update_many(&update_stmt).await?;

// Old - Delete
let sql = query.to_string(SqliteQueryBuilder);
let result = db.execute(&sql, vec![])?;

// New - Delete
let result = service.db_adapter.delete(&delete_stmt).await?;
```

### 6. Update Tests

Update test functions to use async/await:

```rust
// Old
#[test]
fn test_command() {
    // Test code
    let result = command.exec(&mut service);
    // Assertions
}

// New
#[tokio::test]
async fn test_command() {
    // Test code
    let result = command.exec(&mut service).await;
    // Assertions
}
```

## Examples

### Example 1: Create Command

#### Old Implementation

```rust
impl Command for CreateEntityCommand {
    type Output = Entity;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.db_adapter.transaction(|db| {
            let now = Utc::now().naive_utc();
            let new_entity = Entity {
                id: Uuid::now_v7().into(),
                name: self.entity.name.clone(),
                created_at: now,
                updated_at: now,
            };

            // Insert the entity
            let query = Query::insert()
                .into_table(Entities::Table)
                .columns([
                    Entities::Id,
                    Entities::Name,
                    Entities::CreatedAt,
                    Entities::UpdatedAt,
                ])
                .values_panic([
                    new_entity.id.to_string().into(),
                    new_entity.name.clone().into(),
                    new_entity.created_at.to_string().into(),
                    new_entity.updated_at.to_string().into(),
                ])
                .to_string(SqliteQueryBuilder);

            db.execute(&query, vec![])?;

            Ok(new_entity)
        })
    }
}
```

#### New Implementation

```rust
impl Command for CreateEntityCommand {
    type Output = Entity;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let now = Utc::now().naive_utc();
        let new_entity = Entity {
            id: Uuid::now_v7().into(),
            name: self.entity.name.clone(),
            created_at: now,
            updated_at: now,
        };

        // Insert the entity
        let mut insert_query = Query::insert();
        let insert_stmt = insert_query
            .into_table(Entities::Table)
            .columns([
                Entities::Id,
                Entities::Name,
                Entities::CreatedAt,
                Entities::UpdatedAt,
            ])
            .values_panic([
                new_entity.id.to_string().into(),
                new_entity.name.clone().into(),
                new_entity.created_at.to_string().into(),
                new_entity.updated_at.to_string().into(),
            ]);

        service.db_adapter.insert_many(&insert_stmt).await?;

        Ok(new_entity)
    }
}
```

### Example 2: Update Command

#### Old Implementation

```rust
impl Command for UpdateEntityCommand {
    type Output = Entity;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.db_adapter.transaction(|db| {
            // Get the existing entity
            let query = Query::select()
                .from(Entities::Table)
                .columns([
                    Entities::Id,
                    Entities::Name,
                    Entities::CreatedAt,
                    Entities::UpdatedAt,
                ])
                .and_where(Expr::col(Entities::Id).eq(self.entity.id.to_string()))
                .to_string(SqliteQueryBuilder);

            let entity = db.query_optional::<Entity>(&query, vec![])?;
            if entity.is_none() {
                return Err(Error::NotFoundError);
            }
            let entity = entity.unwrap();

            let now = Utc::now().naive_utc();

            // Build update query
            let update = Query::update()
                .table(Entities::Table)
                .and_where(Expr::col(Entities::Id).eq(self.entity.id.to_string()))
                .value(Entities::UpdatedAt, now.to_string());

            if let Some(name) = &self.entity.name {
                update.value(Entities::Name, name.clone());
            }

            let sql = update.to_string(SqliteQueryBuilder);
            db.execute(&sql, vec![])?;

            // Return the updated entity
            let updated_entity = Entity {
                id: entity.id,
                name: self.entity.name.clone().unwrap_or(entity.name),
                created_at: entity.created_at,
                updated_at: now,
            };

            Ok(updated_entity)
        })
    }
}
```

#### New Implementation

```rust
impl Command for UpdateEntityCommand {
    type Output = Entity;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        // Get the existing entity
        let mut query_builder = Query::select();
        let select_stmt = query_builder
            .from(Entities::Table)
            .columns([
                Entities::Id,
                Entities::Name,
                Entities::CreatedAt,
                Entities::UpdatedAt,
            ])
            .and_where(Expr::col(Entities::Id).eq(self.entity.id.to_string()));

        let entity = service.db_adapter.query_optional::<Entity>(&select_stmt).await?;
        if entity.is_none() {
            return Err(Error::NotFoundError);
        }
        let entity = entity.unwrap();

        let now = Utc::now().naive_utc();

        // Build update query
        let mut update_query = Query::update();
        let update_stmt = update_query
            .table(Entities::Table)
            .and_where(Expr::col(Entities::Id).eq(self.entity.id.to_string()))
            .value(Entities::UpdatedAt, now.to_string());

        if let Some(name) = &self.entity.name {
            update_stmt.value(Entities::Name, name.clone());
        }

        service.db_adapter.update_many(&update_stmt).await?;

        // Return the updated entity
        let updated_entity = Entity {
            id: entity.id,
            name: self.entity.name.clone().unwrap_or(entity.name),
            created_at: entity.created_at,
            updated_at: now,
        };

        Ok(updated_entity)
    }
}
```

### Example 3: Delete Command

#### Old Implementation

```rust
impl Command for DeleteEntityCommand {
    type Output = i32;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.db_adapter.transaction(|db| {
            let query = Query::delete()
                .from_table(Entities::Table)
                .and_where(Expr::col(Entities::Id).eq(self.id.to_string()))
                .to_string(SqliteQueryBuilder);

            let result = db.execute(&query, vec![])?;

            if result == 0 {
                return Err(Error::NotFoundError);
            }

            Ok(result as i32)
        })
    }
}
```

#### New Implementation

```rust
impl Command for DeleteEntityCommand {
    type Output = i32;

    async fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        let mut delete_query = Query::delete();
        let delete_stmt = delete_query
            .from_table(Entities::Table)
            .and_where(Expr::col(Entities::Id).eq(self.id.to_string()));

        let result = service.db_adapter.delete(&delete_stmt).await?;

        if result == 0 {
            return Err(Error::NotFoundError);
        }

        Ok(result as i32)
    }
}
```

## Common Issues and Solutions

### 1. Temporary Value Issues

**Issue**: Compiler errors about temporary values being dropped while borrowed.

**Solution**: Use intermediate variables to store query builders and statements:

```rust
// Incorrect
let query = Query::select()
    .from(Table::Table)
    .columns([...])
    .and_where(Expr::col(Table::Id).eq(id.to_string()));

// Correct
let mut query_builder = Query::select();
let query = query_builder
    .from(Table::Table)
    .columns([...])
    .and_where(Expr::col(Table::Id).eq(id.to_string()));
```

### 2. Send/Sync Issues with Transaction Closures

**Issue**: Compiler errors about futures not being Send or Sync.

**Solution**: Avoid using transactions when not necessary, or ensure proper ownership and lifetime management:

```rust
// Incorrect
service.db_adapter.transaction(|db| Box::pin(async move {
    // Using db across await points can cause issues
    let result = db.query_optional::<Entity>(&query).await?;
    // More operations with db
    Ok(result)
})).await

// Correct
// Use direct database operations when possible
let result = service.db_adapter.query_optional::<Entity>(&query).await?;
```

### 3. Missing Await Calls

**Issue**: Forgetting to add .await to async method calls.

**Solution**: Ensure all async method calls have .await:

```rust
// Incorrect
let result = service.db_adapter.query_optional::<Entity>(&query)?;

// Correct
let result = service.db_adapter.query_optional::<Entity>(&query).await?;
```

### 4. Test Function Updates

**Issue**: Test functions not updated to be async.

**Solution**: Update test functions to use tokio::test and await:

```rust
// Incorrect
#[test]
fn test_command() {
    let result = command.exec(&mut service);
}

// Correct
#[tokio::test]
async fn test_command() {
    let result = command.exec(&mut service).await;
}
```

## Testing

After migrating your code, follow these steps to ensure everything works correctly:

1. **Run Cargo Check**: Verify that there are no compilation errors.
   ```
   cargo check
   ```

2. **Run Unit Tests**: Ensure all tests pass with the new implementation.
   ```
   cargo test
   ```

3. **Test Database Operations**: Manually test database operations to ensure they work as expected.

4. **Check Performance**: Monitor performance to ensure the new implementation performs well.

By following this guide, you should be able to successfully migrate your command implementations to the new database adapter pattern. If you encounter any issues not covered in this guide, please consult with the team or refer to the example implementations in the codebase.
