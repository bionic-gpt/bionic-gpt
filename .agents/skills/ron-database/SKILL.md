---
name: ron-database
description: Manage PostgreSQL migrations, typed SQL queries, generated Rust bindings, and database authorization in Rust on Nails applications.
---

# Rust on Nails Database

Discover the destination project's migration directories, SQL generator, build pipeline,
connection roles, and test commands. Do not assume a crate name, environment variable,
Cornucopia/Clorinde version, timestamp mapping, or generated-output location.

## Persistence conventions

- Keep schema changes in migrations and application queries in SQL source files.
  Reuse the project's generator; do not switch generators as part of ordinary work.
- Never edit generated bindings. Regenerate through the existing build/codegen path.
- Verify generated parameter, nullable-result, and timestamp types instead of guessing.
  Query annotations differ by generator/version; follow adjacent SQL files.
- Queries that can return no row require optional-result handling. Mutations must
  distinguish a successful write from zero affected rows.
- Use explicit parameter casts when PostgreSQL cannot infer nullable types. Keep
  enum additions separate from a later migration that uses the new value when required
  by PostgreSQL transaction semantics.
- Discover actual migration commands and roles; avoid interactive aliases. Do not
  run migrations against an arbitrary connection just because an environment variable exists.

## Authorization

Derive the owner from verified identity; treat team and record IDs as selectors to
validate. Scope reads and writes to membership and operation permissions. Set any
RLS identity context transaction-locally before protected queries on that connection.
Inspect policies and application roles; RLS alone does not prove a record matches the
team selected in a route. Keep privileged background access separate.

## Local SQL example

[Schema](examples/schema.sql) and [queries](examples/queries.sql) demonstrate an
items table, memberships, owner-only writes, and parameterized team-scoped queries.
They use plain PostgreSQL positional parameters, not generator-specific annotations.
Each labeled query is a separate statement to adapt into the chosen SQL generator.
The example uses explicit predicates rather than assuming installed RLS helpers.
It is a reference schema, not a migration to apply automatically to an existing project.

## Verification

Use the project's migration test database and narrow compile/codegen check. Verify
constraints, optional results, affected-row handling, denied ownership, nonmember and
cross-team access, and rollback on failure. If adding RLS, also test transaction-context
isolation using the real application role. Report unavailable database-backed checks;
do not claim SQL execution or generated type validation from text review alone.

Optional companions: `ron-auth` and `ron-web-server`, if installed.
