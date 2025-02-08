# Backend Architecture

## Overview

The backend of our POS system is built using Rust with a hexagonal architecture pattern and implements a variation of Command Query Responsibility Segregation (CQRS) pattern. This document outlines the core architectural decisions and patterns used in the project.

## CQRS Implementation

Our CQRS implementation separates read and write operations to optimize for different use cases while maintaining a clean and maintainable codebase.

### Command Side (Write Operations)

Commands represent intentions to change the system state. They are handled by dedicated command handlers located in `src-tauri/src/core/commands`. This is where all write operations and state mutations are processed.

```rust
// Actual project structure
src-tauri/src/
├── core/
│   ├── commands/    # All write operations are handled here
│   │   ├── order/   # Order-related commands
│   │   ├── inventory/
│   │   └── payment/
```

### Query Side (Read Operations)

Queries are responsible for reading and returning data. They are implemented using GraphQL and located in `src-tauri/src/adapters/graphql`. This provides a flexible and efficient way to fetch data with exactly the fields needed.

```rust
// Actual project structure
src-tauri/src/
├── adapters/
│   └── graphql/     # All read operations via GraphQL
```

### Domain Models

Our domain models are centralized in `src-tauri/src/core/models`. These models represent the core business entities and are used by both commands and queries.

```rust
// Actual project structure
src-tauri/src/
├── core/
│   ├── models/      # Centralized domain models
│   │   ├── order.rs
│   │   ├── product.rs
│   │   └── transaction.rs
```

## Core Components

### Domain Layer

Contains the core business logic and domain models:

```rust
src-tauri/
├── core/
│   ├── models/      # Domain entities
│   ├── types/       # Custom types
│   └── errors/      # Domain-specific errors
```

### Infrastructure Layer

Handles external concerns and implements the ports defined in the application layer:

```rust
src-tauri/
├── infrastructure/
│   ├── adapters/      # External service adapters
│   └── repositories/  # Data storage implementations
```

