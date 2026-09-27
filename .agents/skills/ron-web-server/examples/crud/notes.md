# Crud

[handlers.rs](handlers.rs) contains GET edit and POST update/delete. Both selected
team and owner are checked. Invalid updates are authorized before rendering errors;
valid mutations repeat the check under the demonstration store's lock. Missing records
never produce success redirects. Check prepopulation, wrong owner, a swapped team even
for a multi-team member, validation errors, successful update/delete, and repeated delete.
Creation is in the adjacent local form example.

Read [local wiring and dependencies](../notes.md). The entrypoint is [mod.rs](../mod.rs),
shared contracts are [support.rs](../support.rs), and rendering is [ui/mod.rs](../ui/mod.rs).
These are reference modules, not standalone executable crates.
