# Admin Crate

## Purpose
Internal platform control and governance.

## Responsibilities
- Hotel approval / rejection
- Zone listing limits
- Manual suspension

## APIs
- GET /admin/hotels/pending
- POST /admin/hotels/{id}/approve
- POST /admin/hotels/{id}/reject
- PUT /admin/zones/{zone_id}/limit

## Depends On
- hotels
- billing
- db

## Access
Admin-only routes.
