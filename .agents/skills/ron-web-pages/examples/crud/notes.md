# Crud

[page.rs](page.rs) pairs an edit form with a separate delete confirmation. It uses
local components from [support.rs](../support.rs) and [dialogs.js](../dialogs.js).
The loader must authorize the record first. The action must recheck owner and team.
Each confirmation has a record-specific dialog/heading ID; its form is separate from
the update form. Check edit values, delete POST, Cancel, Escape, and multiple dialog
IDs. Creation and listing are illustrated by the other local examples.

Read [local dependencies and wiring](../notes.md) before integrating. These are
reference modules, not separately runnable crates.
