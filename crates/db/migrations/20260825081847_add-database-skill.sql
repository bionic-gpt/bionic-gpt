-- migrate:up
WITH skill AS (
    INSERT INTO context.skills (
        name,
        description,
        visibility,
        is_system
    )
    VALUES (
        'database',
        'Create, query, update, or maintain SQLite databases and structured persistent data. Use for requests involving databases, tables, records, stored state, or data that must be reused later.',
        'Company',
        true
    )
    RETURNING id
)
INSERT INTO context.skill_files (
    skill_id,
    relative_path,
    contents
)
SELECT
    id,
    'SKILL.md',
    convert_to($skill$# Database

Use this skill when the user asks to inspect, query, create, or modify a SQLite
database inside the Bashkit sandbox. Use the `sqlite` command for SQL work. This
skill does not provide access to remote databases or the application's Postgres
database.

## Workflow

1. Identify relevant database files in `/home/user/attachments` or
   `/home/user/output`. Do not assume a file's schema from its name.
2. Inspect an existing database with `.tables`, `.schema`, row counts, and small,
   targeted samples before writing analytical queries.
3. Use `:memory:` for temporary SQL analysis. If an uploaded database must be
   changed, copy it to `/tmp` first so the source evidence remains unchanged.
4. When the user requests a durable database artifact, write it to
   `/home/user/output/<name>.sqlite`. Files elsewhere do not persist across
   `run_bash` calls.
5. Prefer targeted queries and structured output such as `-json`, `-markdown`,
   or `-header`. Avoid dumping large tables when aggregates or limited samples
   answer the question.
6. Before reporting results, check join cardinality, NULL handling, relevant
   column types, filters, and row counts. State material assumptions and report
   unsupported or inconclusive operations instead of guessing.

## Safety and limits

- Do not modify an uploaded database in place.
- Confirm the exact target before destructive statements such as `DROP`,
  `DELETE`, or schema replacement, and use a transaction when practical.
- Keep generated databases within the runtime's output-file limit.
- Bashkit supports a bounded subset of the sqlite3 shell. `ATTACH`, `DETACH`,
  extensions, and an interactive REPL are unavailable.
$skill$, 'UTF8')
FROM skill;

-- migrate:down
DELETE FROM context.skills
WHERE is_system = true
AND name = 'database';
