---
name: ron-database
description: Work with this repository's PostgreSQL database. Use for general database queries, schema inspection, migrations, SQL CRUD, Cornucopia bindings, or database authorization.
---

# Database workflow

This skill is for the repository at `/workspace`. Use its configured PostgreSQL database and the tools below; do not spend time rediscovering the database crate, migration directory, or generator.

## Connect to PostgreSQL

The devcontainer configures these URLs in `.devcontainer/devcontainer.json`. If they are not set in the current shell, load them without printing their values:

```sh
export DATABASE_URL="$(sed -n 's/^[[:space:]]*\"DATABASE_URL\":[[:space:]]*\"\([^\"]*\)\".*/\1/p' .devcontainer/devcontainer.json)"
export APP_DATABASE_URL="$(sed -n 's/^[[:space:]]*\"APP_DATABASE_URL\":[[:space:]]*\"\([^\"]*\)\".*/\1/p' .devcontainer/devcontainer.json)"
```

Database connections may require network access; a restricted shell can fail to reach the configured host even when the database is running. Do not print database URLs or credentials into logs or commit them.

- `DATABASE_URL` connects as `db-owner`. Use it for schema inspection, migrations, and Cornucopia code generation.
- `APP_DATABASE_URL` connects as `application_user`. Use it to check application-role access and row-level security behavior.

The shell aliases in `.devcontainer/bash_aliases` are:

- `db` — `psql "$DATABASE_URL"`
- `dbapp` — `psql "$APP_DATABASE_URL"`
- `dbmate` — `dbmate --no-dump-schema --migrations-dir /workspace/crates/db/migrations`

Use `db` for an interactive owner session, `dbapp` for an application-role session, or run `psql "$DATABASE_URL" -c '<query>'` for a one-off read. For example, count ordinary tables across non-system schemas with:

```sh
psql "$DATABASE_URL" -c "SELECT count(*) FROM information_schema.tables WHERE table_type = 'BASE TABLE' AND table_schema NOT IN ('pg_catalog', 'information_schema');"
```

Use these configured URLs for this workspace rather than inventing a connection string.

## Change the schema with dbmate

1. Create a timestamped migration:

   ```sh
   dbmate --no-dump-schema --migrations-dir /workspace/crates/db/migrations new descriptive_migration_name
   ```

   In an interactive devcontainer shell, the `dbmate` alias supplies those two options, so `dbmate new descriptive_migration_name` is equivalent. Use the full command above when aliases are not loaded. Do not create schema changes in application startup code or edit old migrations that may already have been applied.

2. Edit the newly created SQL file under `crates/db/migrations/`. Use explicit schema names (for example, `iam.users` or `projects.projects`) and include a reversible `-- migrate:up` / `-- migrate:down` pair following neighboring migrations. For enum changes, check PostgreSQL's transaction rules before using a new enum value in the same migration.

3. Inspect pending migrations and apply them to the configured development database:

   ```sh
   dbmate --no-dump-schema --migrations-dir /workspace/crates/db/migrations status
   dbmate --no-dump-schema --migrations-dir /workspace/crates/db/migrations up
   ```

   `dbmate up` changes the database. Run it only when applying the migration is part of the requested task. To deliberately roll back the latest migration, inspect its down section, then run `dbmate --no-dump-schema --migrations-dir /workspace/crates/db/migrations down`; do not use the `dbdown` alias, which repeatedly rolls migrations back until none remain.

4. If queries reference the changed schema, update their SQL as described below, then run the DB crate compile check so Cornucopia validates the SQL against the migrated database.

## Add typed CRUD queries with Cornucopia

1. Put application SQL in a suitable file under `crates/db/queries/` (create a new module file there if needed). Keep each statement named with Cornucopia's `--! query_name` annotation. Use `--: TypeName(...)` annotations for shared/generated record types and `:parameter_name` placeholders for parameters. Follow the nearest query file's conventions; examples include `crates/db/queries/projects.sql` and `crates/db/queries/integrations.sql`.

2. Write the SQL with the access boundary in the query itself: scope reads and mutations by the verified user's permitted team/record, use the repository's `current_app_user()` helper where appropriate, and do not trust a caller-supplied user ID as identity. Check related authorization code and policies when unsure.

3. Compile the database crate from the workspace root:

   ```sh
   cargo check -p db
   ```

   `crates/db/build.rs` runs `cornucopia live -q crates/db/queries -d "$OUT_DIR/cornucopia" "$DATABASE_URL"` as part of the build. It generates Rust bindings in Cargo's `OUT_DIR`; do not edit or commit generated bindings. The live database schema must already include any migration used by the SQL. Fix SQL, annotations, or type mismatches in the query source and rerun the check.

4. Call generated functions through `db::queries::<module>::<query_name>()`; methods take generated parameter structs where applicable and execute against the app's PostgreSQL client/transaction. Add the call at the existing persistence boundary, following nearby Rust code for client use and error handling.

5. For mutations, use `RETURNING` when the caller needs the new row or identifier. For `UPDATE`/`DELETE` without `RETURNING`, inspect the affected-row count when zero rows means not found or not authorized. Model a query that can return no row as optional in Rust; do not assume it returns a record.

## Verify database changes

- After a migration, inspect `dbmate status` using the configured alias or the full command above; run the relevant DB crate compile check after adding or changing Cornucopia SQL.
- For authorization-sensitive queries, verify both allowed and denied cases using the application role (`dbapp` / `APP_DATABASE_URL`) and test team boundaries. Confirm any RLS identity setting is transaction-local and set on the same connection as the protected query.
- Do not claim a migration or query was executed unless the command completed successfully. Report when the configured database is unavailable and distinguish static review from database-backed verification.

## Where to look in this repository

- Migrations: `crates/db/migrations/`
- Cornucopia SQL: `crates/db/queries/`
- Generator invocation: `crates/db/build.rs`
- Database module and generated include: `crates/db/lib.rs`
- Connection variables and aliases: `.devcontainer/devcontainer.json`, `.devcontainer/bash_aliases`
