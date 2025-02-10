# UI Architecture Documentation

## Current State Analysis

### Directory Structure
```
src-ui/
├── app/               # Next.js 13+ App Router entries
├── components/        # Shared components (currently underutilized)
├── public/            # Static assets
└── (config files)     # Next.js/Tailwind configuration
```

### Key Principles
1. **Carbon Integration**
   - Use Carbon's `ModalProps` pattern for all complex components
   - Implement responsive layouts using Carbon grid (2px grid system)
   - Follow Carbon's [component guidelines](https://carbondesignsystem.com)

2. **Type Safety**
   - Strict TypeScript interfaces for all props
   - Zod validation for API responses
   - Generic utility types for common patterns

3. **State Management**
   - React Query for server state
   - Carbon's built-in component state where appropriate

## Roadmap
1. Phase 1: Create core/models and infrastructure/api
2. Phase 2: Reorganize components into features/*
3. Phase 3: Implement shared state management
4. Phase 4: Full Carbon theme integration
