# IAM.md

## Shop Identity & Secrets Management

### Current Issues
- Auth tokens and Turso URLs are currently sourced from environment variables, which is not secure or scalable for multi-tenant or cloud deployments.
- There is no formal shop identity or secure secret storage mechanism.

### Recommendations

#### 1. Shop Identity Establishment
- Each shop should have a unique, cryptographically secure identifier (UUIDv7 recommended for time-ordering).
- On first launch or registration, generate and persist this shop ID securely.
- Use the shop ID as the primary subject for all sync, access, and audit operations.

#### 2. Secure Secrets Storage
- Never store tokens or sensitive URLs in plaintext or environment variables in production.
- Options for secure storage:
  - On Tauri/desktop: Use OS-native secure storage (Keychain on macOS, Credential Vault on Windows, Secret Service on Linux) via Tauri plugin.
  - On cloud/server: Use a secrets manager (e.g., AWS Secrets Manager, HashiCorp Vault, or managed DB with encrypted fields).
- Secrets (auth tokens, Turso URLs, refresh tokens) should be encrypted at rest and only decrypted in-memory when needed.

#### 3. Secrets Access Pattern
- Provide a secure API or abstraction for retrieving secrets in the app.
- Enforce least-privilege: only processes/services that require the secret should have access.
- Rotate secrets regularly and support secret revocation.

## IAM (Identity & Access Management) Architecture

### Should IAM be a Separate Microservice?

#### When to Separate IAM
- If you anticipate multi-tenant SaaS, federated identity, or integration with external IdPs (Google, Microsoft, etc.).
- If you want to centralize authentication, authorization, and audit logging for multiple services.
- If you need to scale IAM independently or enforce strict security boundaries.

#### When to Keep IAM Embedded/Monolithic
- For simple, single-tenant, or offline-first deployments.
- If the app runs mostly on the client (Tauri) and does not expose external APIs.

#### Recommendation
- Start with a modular IAM layer within the backend (src-tauri/src/core/iam or src-tauri/src/adapters/iam).
- Use clean hexagonal ports/adapters for authentication, authorization, and shop identity logic.
- Design the IAM module so it can be extracted as a microservice later if needed.
- For cloud sync or multi-shop scenarios, consider a dedicated IAM service (microservice) that issues JWTs or similar tokens, manages roles, and handles SSO/OAuth.

### IAM Core Features Checklist

- Shop registration and identity issuance
- Secure storage and retrieval of secrets (per shop)
- Authentication (token-based, OAuth, or passwordless)
- Role-based access control (RBAC) for shop users
- Audit logging for sensitive operations
- API for user management (create, update, revoke users/roles)
- Support for secret rotation and revocation

### Implementation Plan

1. Define shop identity model and registration flow.
2. Implement secure secret storage using platform-native APIs.
3. Abstract secret access behind an interface/port.
4. Build IAM module with:
   - Authentication provider(s)
   - Authorization logic (RBAC, scopes)
   - Audit/event sourcing integration
5. Plan for migration to a microservice if/when scaling needs arise.

---

## User Registration Flow (New Shop/User Onboarding)

1. **User initiates registration**
   - User provides required info (email, shop name, etc.) via UI.
2. **Shop identity generation**
   - System generates a unique Shop ID (UUIDv7).
3. **Secrets provisioning**
   - Auth token and Turso URL are securely generated/assigned for the shop.
4. **Secure secret storage**
   - Secrets are encrypted and stored using platform-native secure storage (Tauri plugin or cloud secrets manager).
5. **User account creation**
   - User credentials (hashed password or OAuth identity) are stored in the IAM system.
6. **Role assignment**
   - User is assigned an initial role (e.g., Owner or Admin) for the shop.
7. **Audit log entry**
   - Registration and identity issuance are logged for audit purposes.
8. **Session/token issuance**
   - User receives an authentication token/session, enabling access to the application.

---

## Responsibility for Shop ID, Turso URL, and Turso Token Generation

**Who generates them?**
- The IAM microservice (or a dedicated provisioning microservice) is responsible for generating and managing the Shop ID, Turso URL, and Turso token.

### Rationale
- **Security:** Centralized generation prevents client-side tampering or leakage.
- **Auditability:** All identity and resource creation is logged centrally.
- **Scalability:** Enables automation for multi-tenant SaaS or cloud deployments.
- **Consistency:** Ensures all shops are provisioned with the same process and policies.

### Typical Flow
1. User registers via the app.
2. IAM microservice generates a unique Shop ID (UUIDv7).
3. IAM provisions a Turso database instance for the shop by calling Turso’s API.
4. IAM securely stores the resulting Turso URL and token (in encrypted storage).
5. IAM returns the Shop ID, Turso URL, and token to the client (or stores them in secure storage for the client to retrieve as needed).
6. All actions are audit-logged.

**Summary:**
> In a robust architecture, Shop ID, Turso URL, and Turso token should always be generated and managed by the IAM (or provisioning) microservice—not by the client or frontend.

## User Login Flow (Existing Shop/User)

1. **User initiates login**
   - User submits credentials (password, OAuth, etc.) via UI.
2. **Authentication**
   - IAM module verifies credentials against stored identity provider.
3. **Shop identity lookup**
   - System retrieves the Shop ID associated with the user.
4. **Secrets retrieval**
   - Auth token and Turso URL are securely fetched from encrypted storage for the shop context.
5. **Role and permissions check**
   - User roles and permissions are loaded for session context.
6. **Audit log entry**
   - Login event is recorded for audit and security monitoring.
7. **Session/token issuance**
   - User receives a new authentication token/session for app access.

---

**Diagrammatic Summary:**

- Registration: UI → IAM Port → Identity Generation → Secure Storage → Role Assignment → Audit → Session
- Login: UI → IAM Port → Auth Check → Shop Lookup → Secret Fetch → Role Load → Audit → Session
- User logs in → IAM returns Shop B ID → App detects mismatch with Shop A ID
    ↓
  App clears/isolate Shop A data → Fetches Shop B secrets/data → Sets up Shop B context
    ↓
  User works in Shop B environment (no access to Shop A data)

These flows ensure secure, scalable onboarding and authentication, with clear separation of concerns and compliance with hexagonal architecture principles.
