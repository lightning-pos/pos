---
trigger: always_on
---

You are an AI coding agent. When I ask you to do or implement something, you should start implementing the features by creating or editing the necessary files. You don't have to ask my permission to edit the files.

You are NOT supposed to stop until you completely implement everything I ask you to do.

Every module has its own README.md file. If you are not sure about how to implement a module, you should read the README file and update your context.

READ existing files in any module you are working on to take reference and create context

## Project Package Manager
- Always use only `pnpm` to add/update packages in package.json
- Always use only `pnpm` package manager when running scripts in the package.json

This project has 2 package.json files
- root: For managing tauri
- src-ui: For managing next js frontend

## Terminal Commands
- Ensure you are always running commands from the project's root directory
- You are allowed to `cd` in to a specific folder but only for executing the command. Always come back to the project's root folder

## Code Style and Guidelines (General)
- Use functional and declarative patterns. Avoid using classes.
- Use hexagonal (ports and adapter) architecture for a clean code base.

## Naming Conventions - TypeScript (src-ui)
- Use camelCase for variable names
- Use PascalCase for component names
- Use UPPERCASE for constants
- Use kebab-case for folder names (eg. src-ui)
- Use snake_case for file names

## UI Styling
- Use carbon for ui components
- Use tailwind for styling and layout adjustments
- Implement responsive design using tailwind css

Make BEST developer experience and end user experience a priority.

Make everything type safe whenever possible and utilize best practices for not repeating ourselves.
