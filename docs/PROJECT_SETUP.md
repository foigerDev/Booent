# Project Setup Documentation

## Overview
**Booent** - A WhatsApp-native, commission-free platform for independent hotels to manage operations, bookings, and compliance in one unified system.

## Table of Contents
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Database Setup](#database-setup)
- [Configuration](#configuration)
- [Running the Project](#running-the-project)
- [Development Tools](#development-tools)

## Prerequisites

Before setting up the project, ensure you have the following installed:

- **Rust** (nightly): `rustup install nightly`
- **PostgreSQL** 14+ (for database)
- **sqlx-cli**: `cargo install sqlx-cli --no-default-features --features postgres`
- **Node.js** (optional, for frontend development)
- **Docker** (optional, for containerized database)

### Required Tools
```bash
# Install Rust nightly and make it default for this project
rustup install nightly
rustup override set nightly

# Verify installation
cargo --version
rustc --version
```

## Installation

### 1. Clone the Repository
```bash
cd /home/foiger/projects
git clone <repository-url>
cd booent
```

### 2. Install Dependencies
```bash
cargo build --workspace
```

### 3. Verify Toolchain
```bash
# Check formatting
cargo +nightly fmt --check

# Run clippy for linting
cargo clippy --workspace
```

## Database Setup

### 1. Create PostgreSQL Database
```bash
createdb booent_dev
createdb booent_test
```

### 2. Set Database URL
```bash
export DATABASE_URL=postgres://username:password@localhost:5432/booent_dev
```

### 3. Run Migrations
```bash
# Create migrations if needed
sqlx migrate add -r <migration_name>

# Run all pending migrations
sqlx migrate run --database-url $DATABASE_URL

# Currently available migrations:
# - 20260304172306_user_table (creates users table)
# - 20260308133537_create_sessions_table (creates sessions table)
```

### 4. Verify Database
```bash
psql postgres://username:password@localhost:5432/booent_dev -c "\dt"
```

## Configuration

### Environment Variables
Create a `.env` file in the project root:

```bash
# Database
DATABASE_URL=postgres://username:password@localhost:5432/booent_dev

# API Configuration
API_HOST=0.0.0.0
API_PORT=8000
ADMIN_API_KEY=your-secure-api-key

# Google OAuth
GOOGLE_CLIENT_ID=your-google-client-id
GOOGLE_CLIENT_SECRET=your-google-client-secret

# Encryption
ENCRYPTION_KEY=your-32-byte-encryption-key

# Environment
APP_ENV=development
```

### Load Configuration
Configuration is managed by the `runtime_config` crate:
- Development: `config/development.toml`
- Production: `config/production.toml`
- Sandbox: `config/sandbox.toml`

## Running the Project

### Development Mode
```bash
# Start the API server
cargo r

# With watch mode (requires cargo-watch)
cargo watch -x run
```

### Test API Endpoints
```bash
# Health check
curl http://localhost:8000/health

# Login (with API key in header)
curl -X POST http://localhost:8000/user/login \
  -H "Content-Type: application/json" \
  -H "api_key: your-api-key" \
  -d '{"id_token": "google-id-token"}'
```

## Development Tools

### Code Formatting
```bash
# Format all Rust code
cargo +nightly fmt

# Format and check
cargo +nightly fmt -- --check
```

### Linting & Diagnostics
```bash
# Check for common mistakes
cargo clippy --workspace

# Full build with all diagnostics
cargo build --all-targets

# Check types without building
cargo check --workspace
```

### Database Preparation
```bash
# Prepare SQLx macros (offline mode)
cargo sqlx prepare --workspace -- --database-url $DATABASE_URL
```

### Testing
```bash
# Run all tests
cargo test --workspace

# Run specific crate tests
cargo test -p auth
cargo test -p common
cargo test -p api

# Run with output
cargo test -- --nocapture
```

## Project Structure

```
booent/
├── apps/
│   └── api/                    # REST API server (Axum)
│       └── src/
│           ├── main.rs
│           ├── app.rs          # Router setup
│           ├── app_state.rs    # Application state
│           ├── routes/         # API routes
│           └── api_models/     # Request/Response DTOs
├── crates/
│   ├── auth/                   # Authentication & authorization
│   ├── common/                 # Shared utilities & types
│   ├── hotels/                 # Hotel management logic
│   ├── inventory/              # Inventory management
│   ├── bookings/               # Booking management
│   ├── billing/                # Billing & payments
│   ├── search/                 # Search functionality
│   ├── admin/                  # Admin operations
│   └── runtime_config/         # Runtime configuration
├── migrations/                 # SQL migrations
├── config/                     # Configuration files
├── docs/                       # Documentation (this folder)
└── Cargo.toml                  # Workspace manifest
```

## Troubleshooting

### Database Connection Issues
```bash
# Test database connection
psql $DATABASE_URL -c "SELECT 1"

# Check migration status
sqlx migrate info --database-url $DATABASE_URL
```

### Build Errors
```bash
# Clean build
cargo clean
cargo build --workspace

# Check Rust version
rustc --version  # Should be nightly
```

### Foreign Key Constraint Errors
- Ensure users are created before sessions
- Verify database migrations are up-to-date
- Check foreign key constraints: `\d sessions` in psql

## Next Steps

1. Review [CODEBASE.md](./CODEBASE.md) for architecture details
2. Check individual crate READMEs for domain-specific guidance
3. Explore API routes in `apps/api/src/routes/`

## Support

For issues or questions:
1. Check the README in relevant crate directories
2. Review migration files in `migrations/` for schema details
3. Check configuration in `config/` directory
