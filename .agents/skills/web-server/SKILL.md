---
name: web-server
description: Implement or review Bionic Axum HTTP handlers, routes, authorization, rendering, and form actions. Use for backend web behavior under crates/web-server.
---

# Web Server

Use this skill for HTTP behavior under `crates/web-server`.

## Handler Structure

- Organize routes under `crates/web-server/handlers/<feature>/`.
- Put GET request handling in `loader.rs`.
- Put POST and form mutations in `actions.rs`, with functions named `action_*`.
- Use `mod.rs` to re-export handlers and define the feature `routes()` helper.
- Wire typed paths from `crates/web-pages/routes.rs` into the matching handler.

Loaders should authenticate and authorize before reading data, call the
appropriate generated database queries, and render the matching page function
from `crates/web-pages`. Actions should validate input, authorize the mutation,
call database functions, and return the established redirect or rendered error
path.

Follow existing Axum extractors, error types, transaction handling, and
authorization helpers. Do not trust team, user, ownership, or conversation
identifiers supplied by clients or model tools; derive and verify them through
the authenticated context.

## Verification

Use the `development` skill's watcher-first workflow for compilation and run
focused handler tests for changed behavior. For database-backed handlers, apply
migrations and run `cargo check -p db` when required by the database skill.
Run Clippy, an independent build, or broader tests when the development skill's
conditions require them. Do not start or manage the development environment.
Report checks that require unavailable services.
