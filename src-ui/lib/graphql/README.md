# GraphQL Code Generation

This directory contains generated GraphQL types and utilities for type-safe GraphQL operations in our application.

## Setup

GraphQL code generation is configured in `codegen.ts` and uses the following tools:
- `@graphql-codegen/cli`: Main code generation tool
- `@graphql-codegen/client-preset`: Preset for client-side GraphQL operations

## Configuration

### Scalar Types

We have configured the following custom scalar types:

```typescript
scalars: {
    DbUuid: {
        input: 'string',
        output: 'string',
    },
    LocalDateTime: {
        input: 'string',
        output: 'string',
    },
    Money: {
        input: 'string',
        output: 'number',
    }
}
```

### Document Mode

```typescript
documentMode: 'string'
```

This setting ensures that GraphQL documents are generated as string literals TypedDocumentString that is easier for executor implementation.

## Generated Files

- `graphql.ts`: Contains all generated TypeScript types and interfaces
- `gql.ts`: Contains the tagged template literal for GraphQL queries
- `fragment-masking.ts`: Contains utilities for fragment handling

## Custom Executor File
- `execute.ts`: Contains the `gql` function for executing GraphQL queries

## Usage

1. Write your GraphQL queries in `.gql` files:
```graphql
query GetTaxes($first: Int!, $offset: Int!) {
    taxes(first: $first, offset: $offset) {
        id
        name
        rate
        description
        createdAt
        updatedAt
    }
    totalTaxes
}
```

2. Import and use the generated types:
```typescript
import { GetTaxesDocument, Tax } from '@/lib/graphql/graphql'
import { gql } from '@/lib/graphql/execute'

// Type-safe query execution
const result = await gql(GetTaxesDocument, { first: 10, offset: 0 })
```

## Response Handling

All GraphQL responses follow this pattern:
- Queries return an array with the first element containing the data
- Always use proper TypeScript types for type safety
- Handle loading and error states appropriately

Example:
```typescript
const result = await gql(GetTaxesDocument, { first: size, offset })
setTaxes(result.taxes)
setTotalTaxes(result.totalTaxes)
```

## Best Practices

1. Always use generated types for type safety
2. Keep queries in separate `.gql` files for better organization
3. Use meaningful names for queries and mutations
4. Document complex queries or mutations

## Regenerating Types

Run the codegen command whenever you make changes to GraphQL queries or schema:
```bash
yarn codegen
```

## Troubleshooting

1. If types are not updating:
   - Ensure you've run the codegen command
   - Check that your `.gql` files are in the correct location
   - Verify that the schema is up to date

2. If you get type errors:
   - Check that you're using the correct input/output types for scalars
   - Verify that your queries match the schema
   - Ensure you're handling nullable fields properly
