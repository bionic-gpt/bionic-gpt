---
name: rust-on-nails
description: Design Rust on Nails applications and cross-layer features using Axum, server-rendered Dioxus, PostgreSQL, and typed SQL. Use for architecture, crate boundaries, and request/data flow.
---

# Rust on Nails

Use the [Rust on Nails architecture](https://rust-on-nails.com/docs/) as context.
Inspect the destination project's manifests, entrypoints, and development instructions
before choosing exact APIs. Preserve its SQL generator and dependency versions.
This skill works independently; the architecture below does not require other skills.

## Preferred boundaries

- Pages render supplied data with Dioxus SSR. Compose a shared layout, accessible
  controls, and existing design tokens. Do not query the database from views.
- Axum loaders authenticate, authorize, load data, and render HTML. Actions validate
  input, perform authorized writes, commit, and redirect to a typed destination.
- Keep typed paths shared by links, forms, and handler registration. Prefer feature
  modules containing loaders, actions, and a router rather than one global handler file.
- Keep SQL in query files and generate Rust bindings with the project's existing
  generator. Migrations own schema changes; never edit generated bindings.
- Establish authenticated database context inside the same transaction as protected
  queries. Authorization includes selected-team and record ownership checks.
- Prefer native links and POST forms. Add small local browser enhancements only where
  needed, with matching markup and scripts.

## Suggested organization

`web-pages`, `web-server`, `db`, and `web-assets` can be workspace crates, but names
and paths are conventions, not prerequisites. Discover their equivalents in the
project. A feature usually needs route types, a page, a loader/action module, and
SQL; avoid creating empty layers for simple changes.

## Feature walkthrough

For an items list: define its typed path, authenticate the request, resolve authorized
team context, query visible items, and pass view models to a page with empty and
populated states. For creation: POST named form fields, validate, insert using the
server-derived owner and authorized team, commit, and redirect. Re-render invalid
forms with errors and values. Denied access performs no write.

## Verification

Trace links to handlers, form names to inputs, and query results to view models.
Check empty lists, invalid submissions, successful mutations, denied access, and
cross-team record IDs. Use the project's narrow checks and report missing database
prerequisites. Do not set up or deploy infrastructure just to document a feature.

## Optional companion skills

If installed, `ron-web-pages`, `ron-web-server`, `ron-database`, and `ron-auth`
provide more detail. None is required to read or apply this skill.
