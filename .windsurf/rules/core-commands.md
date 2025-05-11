---
trigger: glob
globs: src-tauri/src/core/commands/**/*.rs
---

# Core Commands: Implementation & Testing

We use command pattern to only mutate any data in our system. This rule provides concise guidelines for implementing and testing commands in the Rust backend, following the Command pattern.

Always add test cases to cover all cases for every command file.

## Command Implementation

### Structure
```rust
// Command Definition
pub struct CreateEntityCommand {
    pub entity: EntityNewInput, // Entity name takes precedence before adjectives
}

// Command Implementation
impl Command for CreateEntityCommand {
    type Output = Entity;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            // Check constraints if needed
            // Create entity with UUID and timestamps
            // Insert and return entity
        })
    }
}
```

### Patterns
- Use transactions for data integrity
- For Create: generate UUID with `Uuid::now_v7()`, set timestamps
- For Update: check entity exists first, update timestamps
- For Delete: return affected rows count
- Check unique constraints before inserts
- Follow hexagonal architecture principles

## Testing Commands

### Test Module Structure
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

    #[test]
    fn test_create_entity() {
        let mut service = AppService::new(":memory:");
        // Create command and execute
        // Assert fields match expected values
    }

    // Additional tests for update, delete, error cases
}
```

### Essential Tests
1. **Creation**: Test with all fields and with minimal fields
2. **Unique Constraints**: Test duplicate creation fails
3. **Updates**: Test full and partial updates
4. **Optional Fields**: Test setting/removing optional fields
5. **Non-existent Updates**: Test updating missing entities fails
6. **Deletion**: Test successful deletion and non-existent deletion

### Testing Tips
- Use in-memory SQLite with `:memory:` connection string
- Test both success and error cases
- Verify database state after operations
- For uniqueness tests, verify appropriate error type is returned

## Examples
See implemented commands in:
- `src-tauri/src/core/commands/purchases/supplier_commands.rs`
- `src-tauri/src/core/commands/purchases/purchase_category_commands.rs`
- `src-tauri/src/core/commands/purchases/expense_commands.rs`
