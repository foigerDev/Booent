# Hotels Crate

## Purpose
Manages hotel onboarding, profiles, and zone allocation.

## Responsibilities
- Hotel creation & updates
- Zone assignment
- Pause / resume listing
- Hotel status management

## APIs
- POST /hotels
- GET /hotels/me
- PUT /hotels/me
- GET /zones
- GET /zones/{zone_id}/availability

## Depends On
- auth
- db

## Does NOT Handle
- Rooms or availability
- Payments
- Search ranking
