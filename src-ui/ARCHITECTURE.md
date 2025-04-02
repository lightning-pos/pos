# UI Architecture Documentation

## Current State Analysis

### Directory Structure
```
src-ui/
├── app/                  # Next.js 13+ App Router entries
│   └── dash/             # Dashboard features (POS, Sales, Catalog, etc.)
│       └── [feature]/      # Feature-specific pages, components, and .gql files
│           ├── page.tsx      # Main page component for the feature
│           └── *.gql         # GraphQL query/mutation definitions
├── components/           # Shared UI components (e.g., DataTable)
│   └── ui/               # General purpose UI components
├── lib/                  # Core libraries and utilities
│   ├── graphql/          # GraphQL client setup and generated code
│   │   ├── graphql.ts    # Generated types and interfaces
│   │   ├── gql.ts        # Tagged template literal generator
│   │   └── execute.ts    # Custom fetch function for GraphQL queries
│   └── util/             # General utility functions
├── public/               # Static assets
├── schema.graphql        # Main GraphQL schema definition
├── codegen.ts            # GraphQL Codegen configuration
└── (config files)        # Next.js/Tailwind/TS/PNPM configuration
```

### Key Principles
1.  **Next.js App Router**: Utilize the Next.js `app` directory for routing and layouts. Feature-specific routes are nested under `/dash`.
2.  **GraphQL & Codegen**:
    *   Define GraphQL queries and mutations in `.gql` files located alongside the components that use them (e.g., `app/dash/pos/items.gql`).
    *   Use `pnpm codegen` (configured in `codegen.ts`) to generate TypeScript types from `schema.graphql` and `.gql` files into `lib/graphql/graphql.ts`. This uses `@graphql-codegen/client-preset`.
    *   Generated types ensure type safety for query variables and results.
    *   Custom scalar types (`DbUuid`, `LocalDateTime`, `Money`, `Percentage`) are mapped to `string` in `codegen.ts`.
    *   Use the custom `gql` executor from `lib/graphql/execute.ts` to run queries/mutations against the backend. This function likely handles fetching and basic result processing.
3.  **Component Structure**:
    *   Pages reside in `app/dash/[feature]/page.tsx`.
    *   Complex pages are broken down into smaller, feature-specific components colocated within the feature directory (e.g., `app/dash/pos/cart/cart_section.tsx`).
    *   Truly shared, reusable components are placed in `components/ui/`.
    *   Components primarily use Carbon Design System components (`@carbon/react`) for UI elements and layout (`Grid`, `Column`).
    *   Tailwind CSS is used for fine-tuning styles and layout adjustments.
4.  **Type Safety**:
    *   Strict TypeScript is enforced via `tsconfig.json`.
    *   GraphQL Codegen provides type safety for API interactions.
    *   Component props are strongly typed. Zod is mentioned but not explicitly seen in the reviewed files; it might be used in API layers or specific utilities not reviewed.
5.  **State Management**:
    *   Client-side state (e.g., UI state like selected category, shopping cart) is managed using React hooks (`useState`).
    *   Server state fetching appears to be handled directly via the custom `gql` executor within components or potentially custom hooks (though React Query is mentioned in the old doc, its direct usage wasn't seen in the `pos` example). Calls are often made within `useEffect` or event handlers.

## Roadmap (Consider Revising Based on Current State)
1.  ~~Phase 1: Create core/models and infrastructure/api~~ (Partially done via `lib/graphql`)
2.  Organize components/features: Ensure components are correctly colocated or placed in shared `components/ui`.
3.  Refine state management: Evaluate if a more robust client state solution (like Zustand or Jotai) or consistent server state management (React Query) is needed as complexity grows.
4.  Full Carbon theme integration: Ensure consistent use of Carbon tokens and themes.
5.  Improve GraphQL error handling and loading states in the UI.

## Implementing New Features/Pages
1.  Create a new directory under `app/dash/` (e.g., `app/dash/new-feature/`).
2.  Add `page.tsx` for the main component.
3.  Define necessary GraphQL queries/mutations in `.gql` files within the feature directory.
4.  Run `pnpm codegen` to generate/update types in `lib/graphql/graphql.ts`.
5.  Import generated types (e.g., `NewFeatureQueryDocument`, `NewFeatureQuery`) and the `gql` executor from `@/lib/graphql/`.
6.  Use the `gql` function (often within `useEffect` or event handlers) to fetch data or perform mutations.
7.  Build the UI using Carbon components and Tailwind CSS.
8.  Manage component state with `useState` or other React hooks.
9.  Place reusable sub-components within the feature directory or in `components/ui/` if globally applicable.
10. Add necessary routing/links in the dashboard layout (`app/dash/layout.tsx`).
