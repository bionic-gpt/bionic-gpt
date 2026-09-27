---
name: ron-web-pages
description: Build server-rendered Dioxus pages using opinionated Rust on Nails patterns. Use for layouts, lists, CRUD controls, typed navigation, forms, and validation feedback.
---

# Rust on Nails Web Pages

Inspect the destination project's UI dependencies and nearest page before editing.
Pages accept view models and render HTML; handlers own database I/O and authorization.
Suggested names such as `web-pages` are integration conventions, not assumed paths.

## Preferred patterns

- Compose a shared layout with title, navigation, breadcrumbs, and responsive content.
  Reuse project components when available; this skill includes local minimal components.
- Use Dioxus SSR, Tailwind utilities, and Daisy UI semantic classes. Prefer existing
  component wrappers when the project has them. Keep labels and semantic HTML.
- Use typed paths for links and form actions. Mutations use POST. Align input names
  with the action's form struct and display validation errors with submitted values.
- Use one shared empty/list introduction and consistent item cards. Derive visible
  edit controls from server-supplied permissions; handlers must enforce access again.
- Give each dialog a unique ID and provide its matching local browser behavior.
  Avoid nested forms and use explicit button types.
- Preserve the destination project's localization and asset pipeline. The examples
  use plain English and no custom icons or generated assets.

## Local examples

Read [example wiring and dependencies](examples/notes.md), then only the relevant files:

- [Blank page](examples/blank-page/notes.md): layout and render boundary.
- [List](examples/list/notes.md): empty state, cards, and typed add/edit links.
- [CRUD](examples/crud/notes.md): prepopulated edit form and delete confirmation.
- [Form](examples/form/notes.md): create/update fields and visible errors.

All required Rust helpers, view models, route types, and dialog script live in this
skill. External library and CSS dependencies are listed locally. These are reference
modules without Cargo scaffolding; there is no dependency on another skill or repository.

## Review

Check empty/populated states, create/edit values, invalid input, unique dialog IDs,
permission-controlled buttons, keyboard dismissal, and named form fields. Check the
paired handlers in the destination project before claiming a feature is complete.
Optional companion: `ron-web-server`, if installed.
