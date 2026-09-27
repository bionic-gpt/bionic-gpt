# List

[page.rs](page.rs) demonstrates empty and populated collections, local `ItemCard`,
and typed add/edit links. All models and components are in [support.rs](../support.rs).
Supply server-derived `can_create` and per-item `can_edit`; readable items need not be
editable. Check empty text, multiple cards, and users without edit permission.

Read [local dependencies and wiring](../notes.md) before integrating. These are
reference modules, not separately runnable crates.
