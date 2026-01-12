# Inventory Crate

## Purpose
Controls room types, pricing, and availability calendars.

## Responsibilities
- Room CRUD
- Daily availability & price updates
- Date blocking

## APIs
- POST /rooms
- GET /rooms
- PUT /rooms/{room_id}
- GET /availability
- PUT /availability

## Depends On
- auth
- hotels
- db

## Does NOT Handle
- Booking creation
- Billing
- Search visibility
