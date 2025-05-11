---
trigger: glob
globs: src-ui/app/**/*.tsx
---

# UI Page Structure and Organization

This rule provides guidelines for structuring UI pages and components in the src-ui app directory.

## Page Organization

Each feature page should follow this organization pattern:

```
src-ui/app/dash/[section]/[feature]/
├── page.tsx                  # Main page component
├── [feature].gql             # GraphQL queries and mutations
├── add_[feature]_modal.tsx   # Modal for adding new items
├── edit_[feature]_modal.tsx  # Modal for editing items
└── delete_[feature]_modal.tsx # Modal for confirming deletion
```

## Component Structure

### Main Page Component

The main page.tsx should follow this structure:
1. Import dependencies
2. Define necessary types/interfaces
3. Define main functional component
4. Initialize state variables
5. Define data transformation functions
6. Define data fetching functions
7. Define CRUD operation handler functions
8. Define UI structure
9. Include all modals as child components
10. Export the component as default

### GraphQL File

- Place all related GraphQL queries and mutations in a separate .gql file
- Never embed GraphQL strings directly in the component files
- Name queries and mutations consistently:
  - Query: GetXxx, GetAllXxx, GetXxxById, etc.
  - Mutation: CreateXxx, UpdateXxx, DeleteXxx

### Modal Components

Each modal component should:
1. Accept props for controlling visibility (isOpen)
2. Accept props for the item being operated on
3. Accept callback functions (onSave, onClose, onDelete)
4. Handle its own form state
5. Be focused on a single responsibility (add/edit/delete)

## Design Patterns

- Use Carbon Design components for consistent UI
- Use DataTable component for list views
- Use modals for CRUD operations
- Implement proper loading states and error handling
- Maintain separation of concerns between components
- Follow TypeScript best practices for type safety
- Format dates, currency, etc. using the appropriate utility functions

## Example Implementation

See existing implementations in:
- src-ui/app/dash/purchases/expenses
- src-ui/app/dash/sales/customers
