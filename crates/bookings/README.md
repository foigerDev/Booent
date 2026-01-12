# Bookings Crate

## Purpose
Manages the booking lifecycle from creation to completion.

## Responsibilities
- Booking creation
- Hotel confirmation
- Cancellation flow
- Booking state transitions

## APIs
- POST /bookings
- GET /bookings
- GET /bookings/{id}
- POST /bookings/{id}/confirm
- POST /bookings/{id}/cancel

## Depends On
- inventory
- hotels
- db

## Does NOT Handle
- Payments
- Reviews
- Search logic
