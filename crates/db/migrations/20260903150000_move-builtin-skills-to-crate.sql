-- migrate:up
-- Built-in skills are now shipped as immutable runtime assets. Preserve
-- database-backed skills while removing the old seeded system rows.
DELETE FROM context.skills
WHERE is_system = true;

-- migrate:down
-- The removed built-in rows are restored by the application crate on restart;
-- this migration intentionally has no database rollback data.
