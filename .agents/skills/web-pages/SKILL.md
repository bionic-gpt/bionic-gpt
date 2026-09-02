---
name: web-pages
description: Build or modify Bionic's server-rendered Dioxus pages and UI components. Use for page layouts, forms, cards, navigation, modals, and authenticated frontend behavior under crates/web-pages.
---

# Web Pages

Use this skill for server-side UI under `crates/web-pages`.

## Structure

- Each route has a folder under `crates/web-pages`.
- The route's main view is normally `page.rs`; supporting components live beside it or under `components/`.
- Typed paths are defined in `crates/web-pages/routes.rs`.
- Corresponding Axum handlers live under `crates/web-server/handlers`.
- Shared widgets belong under `crates/web-pages/components/`.

Use Dioxus `rsx!` and follow existing component patterns. Use Tailwind and
Daisy UI classes, preferably the provided `daisy_rsx` components. Use Daisy UI
colors and existing design tokens rather than introducing a parallel styling
system.

Buttons that open dialogs or popovers should use the existing
`popover_target`/`trigger_id` pattern. Keep semantic HTML, accessible labels,
responsive layouts, and the existing navigation/footer intact.

Client behavior is bundled from `crates/web-assets/index.ts`. When adding or
changing a DOM hook, verify the corresponding TypeScript module and rendered
markup together.

## Verification

Use the `development` skill's watcher-first workflow for compilation. Run
focused tests and application-level or browser checks for the changed behavior.
Use an independent build only when the development skill's conditions require
it. Do not start or manage the development environment. Do not rely on shell
aliases; inspect `Justfile` if the recipe name or environment is unclear.
