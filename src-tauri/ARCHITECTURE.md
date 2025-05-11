# Backend Architecture

## Overview

The backend of Lightning POS is implemented in Rust, following a hexagonal (ports and adapters) architecture with a clear separation of concerns. The system leverages a Command Query Responsibility Segregation (CQRS) pattern, strict type safety, and automation for database interaction. This document describes the current architectural approach, key patterns, and best practices.

---

## Architectural Layers

### 1. Domain Layer
- **Location:** `src/core/models`, `src/core/types`, `src/core/errors`
- **Purpose:** Contains all core business logic, domain models, custom value types (e.g., `Money`, `DbUuid`), and domain-specific errors.
- **Practice:** Models are immutable and strictly typed. All business rules are enforced here.

### 2. Application Layer
- **Location:** `src/core/commands` (write/command side)
- **Purpose:** Handles all state-changing operations (commands). Each command module is responsible for a specific domain (e.g., `auth`, `catalog`, `finance`).
- **Practice:** Commands are invoked by adapters and always operate through the domain models.

### 3. Infrastructure & Adapter Layer
- **Location:** `src/adapters/graphql` (read/query side), `src/core/db`, `src/core/repositories`, `src/core/utils`
- **Purpose:** Implements ports for persistence, external services, and exposes GraphQL for queries. Infrastructure code is kept isolated from business logic.

---

## CQRS Pattern

- **Command Side (Write):**
  - Modules in `src/core/commands` handle all mutations and state changes.
  - Each command is explicit and validated at the domain level.

- **Query Side (Read):**
  - Queries are exposed via GraphQL resolvers in `src/adapters/graphql`.
  - Queries fetch and return data, with no side effects.

**Directory Example:**
```text
src-tauri/src/
├── core/
│   ├── commands/      # Write operations (CQRS Command)
│   ├── models/        # Domain models
│   ├── types/         # Custom types (e.g., Money, DbUuid)
│   ├── repositories/  # Persistence interfaces
│   └── db/            # Database logic and migrations
├── adapters/
│   └── graphql/       # Read operations (CQRS Query)
```

---

## SeaQuery Macro Automation

- **Purpose:** Eliminate boilerplate for database identifiers and type conversions.
- **Macros Used:**
  - `SeaQueryModel`: Auto-generates SeaQuery identifier enums for models.
  - `SeaQueryEnum`: Enables enums to be used directly in SeaQuery expressions.
  - `SeaQueryType`: Supports newtype wrappers (e.g., `DbUuid`, `Money`) for SeaQuery.
- **Location:** `crates/lightning-macros/src/macros/`
- **Benefit:** Ensures type safety, reduces manual code, and keeps DB interaction consistent.

---

## Schema and Migration Process

- **Single Source of Truth:**
  - Domain models in `src/core/models` define the schema.
  - All schema changes begin by updating these models.
- **Migrations:**
  - Managed in `src/core/db/migrations.rs` and `/migrations` directory.
  - Use embedded SQL migrations for performance and testability.
- **Change Flow:**
  1. Update domain model in `models/`.
  2. Add/modify migration in `/migrations`.
  3. Update commands, queries, and types as needed.

---

## Type Safety & Immutability

- All models and types are strictly typed.
- UUIDs are generated using `Uuid::now_v7().into()` for time-based ordering.
- Newtypes (e.g., `DbUuid`, `Money`) enforce domain constraints at compile time.
- Errors are domain-specific and use custom error types.

---

## Testing Strategy

- **Unit Tests:**
  - Located in the same file as the code, in `#[cfg(test)] mod tests`.
  - Each test uses an isolated in-memory SQLite database (`AppService::new(":memory:")`).

- **Integration & E2E:**
  - Playwright-based E2E tests run against the real backend with in-memory DB.
  - Tests are organized for isolation and determinism.

- **Performance:**
  - Embedded SQL migrations and in-memory DBs enable fast, repeatable test runs.

---

## Event Sourcing & Audit Logging

- The architecture is designed to support event sourcing for audit logging. (If not yet implemented, add a note: "Planned for future implementation.")
- All state changes are routed through commands, making it straightforward to add event emission.

---

## Schema Change Process

1. Update the domain model in `src/core/models`.
2. Create or update a migration in `/migrations`.
3. Update command/query handlers and types as needed.
4. Run tests to verify correctness.

---

## Best Practices & Conventions

- Use snake_case for DB field names, PascalCase for Rust types, and UPPERCASE for constants.
- Maintain strict separation between domain, application, and infrastructure code.
- Prefer immutability and pure functions in the domain layer.
- Use derive macros for all database model and enum types.
- All database access goes through repository interfaces.

---

## Example Directory Structure

```text
src-tauri/
├── src/
│   ├── core/
│   │   ├── commands/
│   │   ├── models/
│   │   ├── types/
│   │   ├── repositories/
│   │   └── db/
│   ├── adapters/
│   │   └── graphql/
│   └── main.rs
├── crates/
│   └── lightning-macros/
│       └── src/macros/
│           ├── sea_query_model.rs
│           ├── sea_query_enum.rs
│           └── sea_query_type.rs
├── migrations/
```

---

This architecture ensures maintainability, extensibility, and high developer productivity while enforcing business invariants and type safety at every layer.

