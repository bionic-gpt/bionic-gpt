# Self-contained page references

Use [mod.rs](mod.rs) as the module entrypoint. It wires all four page examples to
[support.rs](support.rs), which defines every local type, component, route, and render
helper. Keep this directory structure when copying; no other skill is required.

External Rust dependencies: Dioxus 0.7 with `macro`, `html`, and `signals` features;
`dioxus-ssr` 0.7; Axum 0.8; `axum-extra` 0.12 with `typed-routing`; and Serde 1 with `derive`.
These versions describe the reference API, not an instruction to upgrade a project.
There is intentionally no Cargo manifest or executable server.

Build Tailwind/Daisy UI CSS with these Rust files included in its content scan and
serve it as `/assets/app.css`, or adapt Layout to the project's existing stylesheet.
Serve the included [dialogs.js](dialogs.js) as `/assets/dialogs.js`. The examples
use no custom images, fonts, generated assets, localization service, or database crate.
Without the stylesheet the markup remains semantic; dialog opening requires the script.

The handler supplies authorized team IDs and view models. List visibility and
`can_edit`/`can_create` only control presentation. The server must repeat authorization
for GET edit and every mutation, validate names, and protect POST forms using the
project's CSRF/session policy. IDs are carried by typed routes; the form submits `name`.

Suggested integration: export these modules from the destination's page crate and call
`blank_page::page`, `list::page`, `form::page`, or `crud::page` from its handlers.
Replace demonstration text and routes deliberately. The suggested page crate name is
not a dependency. All four examples can share this directory's support module.
