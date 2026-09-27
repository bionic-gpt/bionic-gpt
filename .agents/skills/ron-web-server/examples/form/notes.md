# Form

[handlers.rs](handlers.rs) contains GET new and POST create. Both check team access.
The POST validates the trimmed name, re-renders the original input with HTTP 422 on
failure, and derives ownership from identity on success. The form view and field names
are local. Check blank/overlong names, saved trimmed names, denied teams, and redirects.

Read [local wiring and dependencies](../notes.md). The entrypoint is [mod.rs](../mod.rs),
shared contracts are [support.rs](../support.rs), and rendering is [ui/mod.rs](../ui/mod.rs).
These are reference modules, not standalone executable crates.
