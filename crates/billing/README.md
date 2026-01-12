# Billing Crate

## Purpose
Handles commission-free billing (₹29/day per hotel).

## Responsibilities
- Billing status checks
- Payment history
- Suspension on non-payment

## APIs
- GET /billing/status
- GET /billing/history
- POST /billing/pay

## Depends On
- hotels
- db

## Does NOT Handle
- Booking creation
- Search ranking

## Notes
Booking logic must never depend on billing logic.
