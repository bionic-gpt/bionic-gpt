# Agent Guidance

Bionic is a Rust on Nails full-stack application. The backend uses Axum and
PostgreSQL, the authenticated UI is server-rendered with Dioxus, and the
marketing and course site is generated with Rust and `ssg_whiz`.

## Skills

- `database`: migrations, SQL queries, PostgreSQL, Cornucopia, and authorization.
- `web-pages`: server-rendered Dioxus pages and UI components.
- `web-server`: Axum routes, loaders, actions, and handlers.
- `static-sites`: marketing, documentation, course content, and static assets.
- `integration-testing`: k3d, Selenium, and end-to-end tests.
- `development`: persistent local development, watchers, compilation feedback, and service diagnosis.
- `local-deployment`: explicitly requested deployment of the local web application into k3d.

Use the relevant skill before changing code in that area. A task may require
more than one skill. Use `development` for compiling, running, validating, or
diagnosing the local environment.

## Universal Rules

- Inspect the existing code, configuration, and authoritative README files before editing.
- Follow existing patterns and keep changes scoped to the requested behavior.
- Do not rely on interactive shell aliases. Use explicit commands or verified `Justfile` recipes.
- Do not edit generated output or revert unrelated user changes.
- Run the narrowest relevant checks, then report any verification that could not be completed.
