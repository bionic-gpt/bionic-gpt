---
name: database
description: Manage Bionic PostgreSQL schemas, migrations, SQL queries, generated Cornucopia code, and database authorization. Use for schema changes, database-backed features, query changes, persistence, or data access reviews.
---

# Database

Use this skill for changes under `crates/db`, migrations, database queries, or
Rust code that persists or reads application data.

## Repository Conventions

- Migrations live in `crates/db/migrations` and application SQL lives in `crates/db/queries`.
- `crates/db/build.rs` runs Cornucopia against `DATABASE_URL` and generates code under `OUT_DIR`.
- Query modules and public database types are exposed from `crates/db/lib.rs`.
- Keep CRUD SQL in `crates/db/queries`; call generated query functions from Rust rather than duplicating SQL in runtime modules.
- Never edit generated Cornucopia output under `OUT_DIR`.

Create and apply migrations explicitly:

```bash
dbmate --no-dump-schema --migrations-dir crates/db/migrations new <migration-name>
dbmate --no-dump-schema --migrations-dir crates/db/migrations up
```

Use these direct database commands when needed:

```bash
psql "$DATABASE_URL"
psql "$APP_DATABASE_URL"
```

Do not use the interactive `db`, `dbapp`, or `dbmate` shell aliases.

## SQL and Cornucopia

- Add `--: StructName()` before a query when defining a result struct.
- Name queries with `--! query_name : StructName`.
- Let Cornucopia infer parameters from `:parameter_name`; do not declare them manually.
- Use `field_name?` for nullable result fields.
- Use `(:days || ' days')::INTERVAL` for dynamic intervals.
- Add explicit casts for nullable parameters when PostgreSQL cannot infer their type, then verify the generated type.
- Queries using `ON CONFLICT DO NOTHING` may return no row; model their result as optional rather than requiring `.one()`.
- `TIMESTAMPTZ` fields map to `chrono::DateTime<FixedOffset>` in this repository. Convert from `time::OffsetDateTime` explicitly at boundaries.

## Authorization and Safety

- Set the authenticated database context inside the transaction before queries using `current_app_user()` or row-level authorization.
- Scope user-facing queries to the authenticated user and accessible teams.
- Never accept `user_id`, `team_id`, or ownership fields from model or tool arguments.
- Conversation-derived context must verify current ownership and team membership.
- Keep privileged due-task queries separate from user-facing CRUD queries for background schedulers.
- When adding enum values with `ALTER TYPE ... ADD VALUE`, do not reference the new value in the same migration transaction. Use a follow-up migration.

## Verification

For migration-backed changes:

1. Apply migrations with `dbmate --no-dump-schema --migrations-dir crates/db/migrations up`.
2. Run `cargo check -p db` to verify migration-visible queries and Cornucopia generation.
3. Run focused tests, authorization-isolation tests, and constraint/idempotency tests.
4. Run Clippy and `cargo build` before broader workspace tests.

If PostgreSQL is unavailable, do not hand-edit generated code or claim the
database change is validated. Report the unavailable database-dependent checks.
