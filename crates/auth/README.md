# Auth Crate

## Purpose
Handles authentication for hotels and admins using OTP-based login.
No passwords. No guest authentication in V1.

## Responsibilities
- OTP request & verification
- Session / token management
- Authenticated user context

## APIs
- POST /auth/request-otp
- POST /auth/verify-otp
- GET /auth/me
- POST /auth/logout

## Depends On
- db
- utils

## Does NOT Handle
- Authorization rules
- Billing
- Hotel approval
