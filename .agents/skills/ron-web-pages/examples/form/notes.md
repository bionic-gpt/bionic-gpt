# Form

[page.rs](page.rs) renders the local `ItemForm` through `ItemFields` in
[support.rs](../support.rs). `id: None` selects Create; `Some(id)` selects Update.
The only submitted field is `name`. Pass invalid submitted values plus `error` to
re-render; escape text through Dioxus, not raw HTML. Check defaults, editing values,
error feedback, and typed cancel/POST paths.

Read [local dependencies and wiring](../notes.md) before integrating. These are
reference modules, not separately runnable crates.
