# Loader

[loader.rs](loader.rs) uses the local authenticated identity and store to list only
the selected team's items. It computes per-item edit visibility from ownership and
calls the included list view. The store checks team membership before reading.
Check empty/populated lists, absent identity, and nonmember teams.

Read [local wiring and dependencies](../notes.md). The entrypoint is [mod.rs](../mod.rs),
shared contracts are [support.rs](../support.rs), and rendering is [ui/mod.rs](../ui/mod.rs).
These are reference modules, not standalone executable crates.
