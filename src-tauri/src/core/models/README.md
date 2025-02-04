# Model Organization Guidelines

This document outlines the guidelines for creating and organizing models in our codebase.

## Directory Structure

Models are organized by domain in separate directories:

```
models/
├── auth/           # Authentication related models
├── catalog/        # Product catalog related models
├── sales/          # Sales related models
└── ...
```

## Model File Structure

Each model should be defined in its own file following the pattern: `<domain>/<model_name>_model.rs`

Example:
```
sales/
├── cart_model.rs
├── customer_model.rs
└── sales_order_model.rs
```

## Model Components

Each model file should contain the following components:

### 1. Base Model Struct

- Matches exactly with the database schema
- Uses appropriate types (DbUuid, NaiveDateTime, Money, etc.)
- Includes all necessary derives for Diesel

```rust
#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = items)]
pub struct Item {
    pub id: DbUuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
```

### 2. Input Structs

#### New Input
- Used for GraphQL mutations
- Contains only the fields needed for creation
- Named as `{Model}NewInput`

```rust
#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ItemNewInput {
    pub name: String,
    pub description: Option<String>,
}
```

#### Update Input
- Used for GraphQL mutations
- Contains optional fields for updates
- Named as `{Model}UpdateInput`
- For nullable fields, use `Option<Option<T>>`

```rust
#[derive(Debug, Clone, GraphQLInputObject)]
pub struct ItemUpdateInput {
    pub id: DbUuid,
    pub name: Option<String>,
    pub description: Option<Option<String>>,
}
```

### 3. Changeset Struct

- Used for Diesel updates
- Contains all updateable fields
- Named as `{Model}UpdateChangeset`
- Includes `updated_at` field

```rust
#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = items)]
pub struct ItemUpdateChangeset {
    pub id: DbUuid,
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub updated_at: NaiveDateTime,
}
```

### 4. Enums (if needed)

- Use DbEnum for database mapping
- Include GraphQLEnum for GraphQL integration
- Add AsExpression when needed
- Named consistently with the model

```rust
#[derive(Debug, Clone, Copy, DbEnum, GraphQLEnum, AsExpression)]
#[diesel(sql_type = Text)]
pub enum ItemState {
    Active,
    Inactive,
    Deleted,
}
```

## Common Types

Use the following types consistently across models:

- `DbUuid` for IDs
- `NaiveDateTime` for timestamps
- `Money` for monetary values
- `String` for text fields
- `Option<T>` for nullable fields
- `Option<Option<T>>` for nullable fields in update inputs, first "option" is to determine if the field is to be updated, second "option" is to determine the new value is null


## Best Practices

1. Always match model fields exactly with the database schema
2. Use appropriate derives for each struct's purpose
3. Keep models focused on data structure and persistence
4. Use clear and consistent naming conventions
5. Document complex fields or business logic
6. Implement proper type safety using Rust's type system
7. Keep models independent of business logic implementation

## Schema Updates

When updating the database schema:

1. Update the schema.rs file first
2. Update the corresponding model to match exactly
3. Update all related input and changeset structs
4. Ensure all type mappings are correct
5. Add any new enums if required
