# Step-by-Step Plan: Integrate IAM Login in Lightning POS

## 1. Requirements & Overview
- The POS app login page should accept a **username** and **password**.
- On login, credentials are sent to the **IAM service**.
- The IAM service responds with a **Turso URL** and **token**.
- These credentials are used for all subsequent backend/database operations.
- The solution must be **type-safe**, **secure**, and fit the **hexagonal architecture**.

## 2. Frontend Changes (`src-ui`)

### 2.1. Update Login Page
- Refactor the login form to collect `username` and `password`.
- Add a submit handler to:
  - Call a Tauri command (e.g., `loginWithIam`) with credentials.
  - Handle loading and error states.
- Use Carbon components and `data-testid` attributes for testability.

### 2.2. Define Types
- Create TypeScript interfaces for:
  - **Login request:**
    ```ts
    interface LoginRequest {
      username: string;
      password: string;
    }
    ```
  - **Login response:**
    ```ts
    interface LoginResponse {
      tursoUrl: string;
      tursoToken: string;
    }
    ```

### 2.3. State Management
- Store `tursoUrl` and `tursoToken` in a centralized, in-memory state (e.g., React Context or Zustand).
- Ensure these values are cleared on logout.

### 2.4. Credential Injection
- Refactor all API/database calls to use the stored credentials.
- Inject `tursoUrl` and `tursoToken` into the backend call context (e.g., as part of Tauri command params).

### 2.5. Security
- **Never persist tokens in plain localStorage.**
- Use in-memory storage or secure Electron APIs if persistence is needed.

## 3. Backend Changes (`src-tauri`)

### 3.1. Implement Tauri Command
- Add a new Tauri command (e.g., `login_with_iam`) in Rust:
  - Accepts `username` and `password`.
  - Calls the IAM service (HTTP/gRPC) to get Turso credentials.
  - Returns `{ turso_url, turso_token }` to the frontend.
  - Handle errors and map them to user-friendly messages.

### 3.2. App Service Refactoring
- Refactor app/database initialization to accept dynamic `turso_url` and `turso_token`.
- Ensure the app service can be re-initialized or reconnected post-login with new credentials.
- Use dependency injection to keep the codebase modular and testable.

### 3.3. Type Safety
- Use Rust structs and enums for request/response types.
- Propagate errors using domain-specific error types.

## 4. Application Flow
- On app start, check if credentials exist in memory:
  - If not, show the login page.
  - If yes, initialize the app service/database with credentials.
- On logout, clear credentials and reset the app state.

## 5. Testing

### 5.1. Frontend
- Write integration tests for the login flow using Playwright:
  - Test successful login, error states, and logout.
  - Use `data-testid` selectors for robust targeting.

### 5.2. Backend
- Add unit tests for the new Tauri command.
- Test error handling and IAM service integration.

## 6. CI/CD
- Update CI scripts to:
  - Run integration tests for the login flow.
  - Mock IAM service or use a test IAM instance for end-to-end tests.

## 7. Documentation
- Document the login flow and credential management in the project README.
- Include security notes and troubleshooting tips.

## 8. Rollout
- Deploy changes to a staging environment.
- Validate with manual and automated tests.
- Roll out to production after successful validation.

## 9. Future Enhancements
- Add support for token refresh/expiration handling.
- Implement multi-user session management if needed.
- Integrate audit logging for login events.
