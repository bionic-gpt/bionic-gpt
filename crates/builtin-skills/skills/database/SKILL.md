# Database

Use this skill when the user asks to inspect, query, create, or modify a SQLite database inside the Bashkit sandbox. Use the `sqlite` command for SQL work. This skill does not provide access to remote databases or the application's Postgres database.

## Workflow

1. Identify relevant database files in `/home/user/attachments` or `/home/user/output`.
2. Inspect an existing database with `.tables`, `.schema`, row counts, and small targeted samples before writing analytical queries.
3. Use `:memory:` for temporary SQL analysis. Copy an uploaded database to `/tmp` before changing it.
4. Write durable database artifacts to `/home/user/output/<name>.sqlite`.
5. Prefer targeted queries and structured output. Avoid dumping large tables.
6. Check join cardinality, NULL handling, types, filters, and row counts before reporting results.

## Safety

- Do not modify an uploaded database in place.
- Confirm the exact target before destructive statements and use a transaction when practical.
- Bashkit supports a bounded subset of the sqlite3 shell; `ATTACH`, `DETACH`, extensions, and an interactive REPL are unavailable.
