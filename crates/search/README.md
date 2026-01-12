# Search Crate

## Purpose
Guest-facing hotel discovery with enforced business rules.

## Responsibilities
- Hotel search by city, zone, dates
- Enforce zone listing limits
- Hide unpaid or paused hotels

## APIs
- GET /search
- GET /hotels/{hotel_id}

## Depends On
- hotels
- inventory
- billing

## Does NOT Handle
- Booking creation
- Authentication
- Admin actions
