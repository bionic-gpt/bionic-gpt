---
name: ron-web-server
description: Implement Axum loaders and actions using Rust on Nails typed routing, authorization, validation, HTML rendering, and persistence boundaries.
---

# Rust on Nails Web Server

Discover the project's route definitions, identity extractor, error type, SQL generator,
and state before editing. Preserve established APIs; example module names are suggestions.

## Preferred request flow

- Register typed GET loaders and POST actions per feature. Keep typed paths available
  to rendered links/forms. Put body-consuming extractors last.
- Extract verified identity, authorize the selected team, then authorize the record.
  A client-supplied team or item ID is a selector, not proof of permission.
- Loaders query authorized data and pass view models to SSR functions.
- Actions validate before writes; on validation errors re-render values and errors.
  Never redirect with success when no mutation occurred.
- In database-backed applications establish auth context inside a transaction, perform
  scoped queries, check affected rows, and commit before redirecting. Keep authorization
  and mutation atomic. Propagate database/commit failures without success feedback.
- UI permission checks are presentation only. Do not accept owner identity from forms.

## Local examples

Start with [wiring, dependencies, and adapter boundaries](examples/notes.md):

- [Loader](examples/loader/notes.md): authorized list response.
- [Form](examples/form/notes.md): new form, validation errors, and create redirect.
- [CRUD](examples/crud/notes.md): authorized edit/update/delete.

The skill includes its own routes, view rendering, verified-identity extractor,
error responses, and an explicitly labeled in-memory demonstration store. No helper
is imported from a destination repository or sibling skill. The store demonstrates
atomic authorization/write behavior, not PostgreSQL transactions or durable storage.
Replace its adapter with the project's generated SQL and transactional auth context.

## Review

Check absent identity, nonmember teams, wrong-owner and cross-team IDs, invalid names,
missing records, valid writes, and storage failures. Invalid updates still require
record authorization. Success must follow a completed write/commit. Optional companions:
`ron-auth` and `ron-database`, if installed; neither is required for these examples.
