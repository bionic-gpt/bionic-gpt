# Blank Page

Use [page.rs](page.rs) for a minimal layout plus content. Its only local dependency
is [support.rs](../support.rs), loaded by [mod.rs](../mod.rs). The layout, navigation,
render helper, and typed index path are included here; nothing is read from an app repo.
Pass the authorized team ID from a loader. Check title, breadcrumbs, and content.

Read [local dependencies and wiring](../notes.md) before integrating. These are
reference modules, not separately runnable crates.
