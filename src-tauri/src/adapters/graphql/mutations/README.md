# GraphQL Mutations

This directory contains all GraphQL mutations for the application, following the hexagonal architecture with CQRS pattern. The mutations are organized by domain and follow a consistent structure for maintainability and scalability.

## Directory Structure

```
mutations/
├── auth/                 # Authentication related mutations
├── catalog/             # Catalog related mutations
├── sales/              # Sales related mutations
├── mod.rs              # Root mutation object definitions
└── README.md           # This file
```

## Adding a New Mutation

Follow these steps to add a new mutation:

1. **Choose or Create Domain Directory**
   - Place your mutation in an existing domain directory (e.g., `auth/`, `catalog/`, `sales/`)
   - If creating a new domain, create a new directory and add it to `mod.rs`

2. **Create Mutation File**
   - Name the file using snake_case: `your_feature_mutations.rs`
   - Follow the standard mutation file structure:
   ```rust
   use crate::{
       core::{
           commands::{your_domain::your_commands::*, Command},
           models::your_domain::your_model::*,
           types::db_uuid::DbUuid,
       },
       AppState,
   };
   use juniper::FieldResult;

   pub fn your_mutation(input: YourInput, context: &AppState) -> FieldResult<YourOutput> {
       let mut service = context.service.lock().unwrap();
       let res = YourCommand { input }.exec(&mut service)?;
       Ok(res)
   }
   ```

3. **Update Root Mutation Object**
   - Add your mutation to `mod.rs`
   - Follow the existing pattern:
   ```rust
   #[graphql_object(context = AppState)]
   impl Mutation {
       fn your_mutation(input: YourInput, context: &AppState) -> FieldResult<YourOutput> {
           your_domain::your_feature_mutations::your_mutation(input, context)
       }
   }
   ```

## Best Practices

1. **Type Safety**
   - Use strongly typed input and output models
   - Leverage the type system for validation where possible

2. **Error Handling**
   - Use `FieldResult<T>` for all mutation returns
   - Properly propagate errors using the `?` operator
   - Provide meaningful error messages

3. **Command Pattern**
   - Each mutation should correspond to a command in the core layer
   - Follow CQRS principles - mutations are for write operations only

4. **Naming Conventions**
   - Use descriptive names that clearly indicate the mutation's purpose
   - Follow Rust naming conventions (snake_case for functions)
   - Be consistent with existing mutation names
