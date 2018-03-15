## Core Standardfile Backend Definitions

We define a trait `StandardFileStorage` to be implemented by each of the `backend` packages.

This also contains any of the `diesel` models/schemas which are universal for each `backend` as

Note: Any changes to this Model/Schema's should result in a new diesel migration for each of the `backend`s.
