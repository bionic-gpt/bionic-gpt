# Self-contained handler references

[mod.rs](mod.rs) wires every typed route to a loader/action and supplies `Arc<Store>`.
[support.rs](support.rs) defines all identity, error, form, validation, and storage
contracts. [ui/mod.rs](ui/mod.rs) owns the local Dioxus rendering modules;
[ui/support.rs](ui/support.rs) defines their components, view models, and typed paths.
Nothing imports another skill or a destination application's helpers.

External dependencies: Axum 0.8 with form support; `axum-extra` 0.12 with
`typed-routing`; Serde 1 with `derive`; Dioxus 0.7 with `macro`, `html`, and `signals`;
and `dioxus-ssr` 0.7. The standard library supplies the demonstration store.
These describe the reference API; adapt to a destination's compatible versions.
There is no Cargo manifest, main function, or production authentication implementation.

## Integration boundaries

- Mount `routes(Arc::new(Store::default()))` under verified authentication middleware.
  That middleware inserts `Identity` into request extensions. The custom extractor
  returns 401 when it is absent. Do not substitute unchecked headers or static users.
- Team membership is part of that trusted identity snapshot. The demo lets members
  read/create and owners update/delete. Production adapters must recheck current
  database membership and operation permissions during persistence operations.
- `Store` is an explicitly non-durable demonstration adapter. Its lock makes ownership
  checks and writes atomic. Replace its methods with generated SQL scoped by verified
  user, selected team, and record ID; use one authenticated transaction, check affected
  rows, and commit before returning success. Do not treat the mutex as a DB transaction.
- Serve Tailwind/Daisy UI CSS at `/assets/app.css` and the local
  [dialog script](ui/dialogs.js) at `/assets/dialogs.js`, or adapt those paths to the
  asset pipeline. Include local Rust files in the CSS content scan.
- Add the destination's session/CSRF protection around POST routes. The example does
  not implement sessions, token verification, CSRF tokens, or a stylesheet build.

## Request contract

GET list/new/edit render pages. POST create/update accept a single `name` field;
IDs come from typed paths, owner identity from middleware. Invalid names render HTTP
422 with entered values and errors. Missing/malformed form data uses Axum's extractor
rejection. Successful writes return a 303 redirect. Missing identity is 401; denied
team/owner is 403; absent or wrong-team record is 404; storage failure is 500.
Read the relevant [loader](loader/notes.md), [form](form/notes.md), or
[CRUD](crud/notes.md) notes. These references can be copied as one module tree into
an unrelated Rust project; no original repository is needed.
