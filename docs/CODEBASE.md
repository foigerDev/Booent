# Codebase Documentation

## Table of Contents
- [Architecture Overview](#architecture-overview)
- [Core Crates](#core-crates)
- [API Structure](#api-structure)
- [Database Layer](#database-layer)
- [Authentication Flow](#authentication-flow)
- [Key Design Patterns](#key-design-patterns)
- [Error Handling](#error-handling)
- [Security Practices](#security-practices)

## Architecture Overview

Booent follows a **modular monolith** architecture with clear separation of concerns:

```
┌─────────────────────────────────────────────────────────┐
│                    REST API (Axum)                      │
│              apps/api - Port 8000                       │
└────────────────────┬────────────────────────────────────┘
                     │
        ┌────────────┼────────────┐
        │            │            │
    ┌───▼──┐    ┌───▼──┐    ┌──▼────┐
    │ Auth │    │Hotels│    │Bookings
    └───┬──┘    └──────┘    └────────┘
        │
┌───────▼─────────────────────────┐
│    Database (PostgreSQL)        │
│  - Users                        │
│  - Sessions                     │
│  - Hotels, Bookings, etc.      │
└─────────────────────────────────┘
```

### Key Principles
- **Domain-driven design**: Each crate handles a business domain
- **Dependency injection**: State shared via app context
- **Error propagation**: Using `error-stack` for rich error contexts
- **Type safety**: Leveraging Rust's type system for compile-time guarantees
- **Async-first**: All I/O operations are non-blocking

## Core Crates

### 1. **auth** - Authentication & Session Management
**Location**: `crates/auth/`

**Purpose**: Handles user authentication, JWT generation, and session management

**Key Components**:
- `user_management::services`
  - `user_sign_up()` - Register new users via OAuth
  - `user_login()` - Authenticate existing users
  - `user_auth_core()` - Core token generation logic

- `user_management::jwt` - JWT token generation and validation
- `user_management::token` - Refresh token generation and hashing
- `user_management::verifiers` - External service verification (Google OAuth)

**Key Types**:
```rust
pub struct TokenPair {
    pub access_token: Secret<String>,
    pub refresh_token: Secret<String>,
}

pub struct SessionData {
    pub id: String,
    pub user_id: String,
    pub refresh_token_hash: String,
    pub expires_at: OffsetDateTime,
    pub revoked: bool,
    pub ip_address: Option<String>,
}
```

**Flow**:
1. User submits Google ID token
2. Verify token with Google
3. Find or create user in database
4. Generate access + refresh tokens
5. Create session record linking tokens to user
6. Return tokens (access in body, refresh in HTTP-only cookie)

### 2. **common** - Shared Utilities & Domain Models
**Location**: `crates/common/`

**Purpose**: Provides foundational types and utilities used across all domains

**Key Modules**:

#### `domain_models/`
- `auth.rs` - User, token, and session data structures
- Shared types for Google OAuth integration

#### `db/`
**Traits**:
- `UserRepository` - User CRUD operations
- `SessionRepository` - Session management

**Models**:
- `users.rs` - Database user representation
- `sessions.rs` - Database session representation

**Queries**:
- SQL files in `src/db/queries/` using SQLx macros

#### `errors.rs`
Custom error types with rich context:
```rust
pub enum AuthErrorTypes {
    UserNotFound,
    UserAlreadyRegistered,
    InvalidToken,
    SessionExpired,
    // ...
}
```

#### `encryption.rs`
**Features**:
- AES-256-GCM encryption for sensitive data
- SHA-256 key derivation from encryption key strings
- Secure nonce generation

#### `time.rs`
Utilities for timestamp handling with timezone awareness

#### `consts.rs`
Application-wide constants (token expiry, rate limits, etc.)

### 3. **runtime_config** - Configuration Management
**Location**: `crates/runtime_config/`

**Purpose**: Runtime configuration loading and validation

**Key Components**:
- `RuntimeConfig` - Struct holding all app config
- `builder.rs` - Configuration builder pattern
- Support for environment-specific configs

**Usage**:
```rust
let config = RuntimeConfig::from_env()?;
println!("{:?}", config.admin_api_key);
```

### 4. **hotels** - Hotel Management
**Location**: `crates/hotels/`

**Purpose**: Hotel data management and operations (in progress)

### 5. **bookings** - Booking Management
**Location**: `crates/bookings/`

**Purpose**: Hotel booking operations (in progress)

### 6. **inventory** - Inventory Management
**Location**: `crates/inventory/`

**Purpose**: Room and inventory tracking (in progress)

### 7. **billing** - Payments & Billing
**Location**: `crates/billing/`

**Purpose**: Payment processing and billing operations (in progress)

### 8. **search** - Search Functionality
**Location**: `crates/search/`

**Purpose**: Hotel and booking search capabilities (in progress)

### 9. **admin** - Administrative Operations
**Location**: `crates/admin/`

**Purpose**: Admin management and operations (in progress)

## API Structure

### Location
`apps/api/src/`

### Router Setup (`app.rs`)
```rust
pub fn build_app(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .nest("/user", user_routes())
        .with_state(state)
}
```

### API Routes (`routes/`)

#### Authentication Routes
**File**: `routes/auth.rs`

**Endpoints**:
- `POST /user/login` - Authenticate user with Google token
  - Headers: `api_key` (required)
  - Body: `{ "id_token": "google_token" }`
  - Response: `{ "access_token": "jwt_token" }`
  - Cookie: Refresh token (HTTP-only, Secure, SameSite=Strict)

- `POST /user/signup` - Register new user
  - Same headers and body as login
  - Creates user in database before returning tokens

#### Health Check
**File**: `routes/health.rs`

- `GET /health` - Service health status

### Request/Response Models (`api_models/`)
**File**: `api_models/auth.rs`

```rust
#[derive(Deserialize)]
pub struct LoginRequest {
    pub id_token: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub access_token: String,
}
```

### Application State (`app_state.rs`)
```rust
pub struct AppState {
    pub db_pool: PgPool,
    pub config: RuntimeConfig,
}
```

## Database Layer

### Schema
**Users Table**:
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY,
    auth_provider VARCHAR,
    auth_provider_user_id VARCHAR,
    email VARCHAR,
    created_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ
);
```

**Sessions Table**:
```sql
CREATE TABLE sessions (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    refresh_token_hash VARCHAR,
    expires_at TIMESTAMPTZ,
    revoked BOOLEAN,
    created_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ
);
```

### Migrations
**Location**: `migrations/`

**Running Migrations**:
```bash
sqlx migrate run --database-url $DATABASE_URL
```

**Current Migrations**:
1. `20260304172306_user_table` - Users table creation
2. `20260308133537_create_sessions_table` - Sessions table

### Repository Pattern
All database interactions go through trait-based repositories:

```rust
#[async_trait]
pub trait UserRepository {
    async fn find_user_by_provider_identity(...) -> Result<Option<UserData>>;
    async fn create_user(...) -> Result<UserData>;
}

impl UserRepository for PgPool {
    // Implementation using SQLx
}
```

## Authentication Flow

### Sign Up Flow
```
Client                          API                      Database
  │                              │                          │
  ├─POST /user/signup ──────────>│                          │
  │ { id_token }                 │                          │
  │                              ├─Verify Google token─────>│
  │                              │                          │
  │                              ├─Check user exists───────>│
  │                              │ (find_user_by_provider)  │
  │                              │<─ None ──────────────────┤
  │                              │                          │
  │                              ├─Create user ───────────>│
  │                              │<─ UserData ──────────────┤
  │                              │                          │
  │                              ├─Generate access token────│
  │                              ├─Generate refresh token───│
  │                              ├─Create session ────────>│
  │                              │<─ SessionData ─────────--┤
  │                              │                          │
  │<─ 200 OK ──────────────────<─┤
  │ access_token (body)           │
  │ refresh_token (cookie)        │
```

### Login Flow
```
1. Verify Google OAuth token
2. Find user by provider identity
3. If not found → Error (UserNotFound)
4. If found → Generate new session and tokens
5. Return tokens with 200 OK
```

## Key Design Patterns

### 1. Repository Pattern
Access layer abstraction for database operations:
```rust
#[async_trait]
pub trait UserRepository {
    async fn find_user_by_provider_identity(
        &self,
        provider: &str,
        provider_user_id: &str,
    ) -> Result<Option<UserData>>;
}
```

### 2. Service Layer Pattern
Business logic encapsulation:
```rust
pub async fn user_sign_up(
    db: &PgPool,
    config: RuntimeConfig,
    claims: GoogleUserClaims,
) -> Result<TokenPair>;
```

### 3. Dependency Injection via State
Share dependencies through Axum state:
```rust
Router::new()
    .with_state(Arc::new(AppState {
        db_pool,
        config,
    }))
```

### 4. Error Context Propagation
Using `error-stack` for rich error information:
```rust
result
    .change_context(errors::AuthErrorTypes::InvalidToken)
    .attach_printable("Token validation failed")
```

## Error Handling

### Error Hierarchy
```
Report<AuthErrorTypes>
├── UserNotFound
├── UserAlreadyRegistered
├── InvalidToken
├── SessionExpired
├── ApiAuthorizationFailed
├── EncryptionFailed
├── DecryptionFailed
└── InternalServerError
```

### Error Middleware
**File**: `common/src/errors.rs`

Converts domain errors to HTTP responses:
```rust
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::Auth(e) => (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Unauthorized" }))
            ),
            // ...
        }
    }
}
```

## Security Practices

### 1. **Token Management**
- Access tokens: Short-lived (15 minutes), in response body
- Refresh tokens: Long-lived (7 days), in HTTP-only cookie
- All tokens use encryption with random nonces

### 2. **Encryption**
- **Algorithm**: AES-256-GCM
- **Key derivation**: SHA-256 from encryption key string
- **Nonce**: Randomly generated per encryption

### 3. **Password & Secrets**
- All secrets wrapped in `secrecy::Secret<T>`
- Prevents accidental logging of sensitive data
- Secrets never appear in error messages

### 4. **Database Security**
- Foreign key constraints enforce data integrity
- User ID validation before session creation
- Refresh token hashing (never store plain tokens)

### 5. **API Authentication**
- API key validation on all protected routes
- Headers: `api_key` (from config)

### 6. **CORS & Headers**
- Refresh token in HTTP-only cookie to prevent XSS
- Secure flag for HTTPS-only transmission
- SameSite=Strict to prevent CSRF

## Code Quality Tools

### Formatting
```bash
cargo +nightly fmt
```

### Linting
```bash
cargo clippy --workspace
```

### Testing
```bash
cargo test --workspace
```

### Type Checking
```bash
cargo check --workspace
```

## Development Workflow

### 1. Make Changes
```bash
# Edit code
vim crates/auth/src/lib.rs
```

### 2. Format & Lint
```bash
cargo +nightly fmt
cargo clippy --all-targets
```

### 3. Test
```bash
cargo test -p auth
```

### 4. Build
```bash
cargo build
```

### 5. Run
```bash
cargo r  # Run the API server
```

## Common Issues & Solutions

### Issue: Foreign Key Constraint Violation
**Cause**: Trying to create session for non-existent user

**Solution**: Call `create_user()` before `create_session()`

### Issue: Encryption "Invalid Length" Error
**Cause**: Encryption key is not 32 bytes

**Solution**: Use SHA-256 key derivation (now implemented)

### Issue: Token Type Inference Errors
**Cause**: Missing explicit type annotations for `Nonce<U12>`

**Solution**: Use `.try_into()` with explicit type: `let nonce: &Nonce<U12> = bytes.try_into()?`

## Next Steps for Development

1. **Implement Hotel Management** - `crates/hotels/`
2. **Implement Bookings** - `crates/bookings/`
3. **Add Search Engine** - `crates/search/`
4. **Implement Billing** - `crates/billing/`
5. **Add Admin Panel** - `crates/admin/`
6. **Frontend Integration** - React/Vue application

## Resources

- [Axum Web Framework](https://github.com/tokio-rs/axum)
- [SQLx Database Driver](https://github.com/launchbadge/sqlx)
- [Error Stack Library](https://docs.rs/error-stack/)
- [Secrecy Crate](https://docs.rs/secrecy/)
- [PostgreSQL Docs](https://www.postgresql.org/docs/)
