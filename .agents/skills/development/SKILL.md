---
name: development
description: Use Bionic's persistent local development environment, watcher-first compilation workflow, and existing k3d services. Trigger for running, compiling, or diagnosing the local application environment.
---

# Development

Use this skill when compiling changes, validating the local application, or
diagnosing the development environment.

The user owns and manages the persistent development environment. Codex must
not start it, attach to it, send input to it, or otherwise take control of it.

## Persistent Environment

The user starts the normal development environment with `just dev`. Codex must
never run `just dev`. This creates or attaches to the persistent tmux session
`bionic-dev` through `.devcontainer/dev-tmux.sh`. The session has one window
named `app`:

- `bionic-dev:app.0` runs `just wa`, which uses `cargo watch` to compile and restart the Rust web server.
- `bionic-dev:app.1` runs `just wp`, which watches and bundles TypeScript assets.
- `bionic-dev:app.2` runs `just wt`, which watches and rebuilds Tailwind CSS.

Supporting services, including PostgreSQL, run in the local `k3d` cluster and
are exposed through the configured environment. Reuse those services when the
development environment is available. Codex may use already exposed services,
but must not create, redeploy, restart, or replace the cluster, PostgreSQL, or
other supporting services unless the user explicitly requests environment
management.

## Watcher-First Feedback

Read-only inspection is allowed. Check whether the session is running:

```bash
tmux has-session -t bionic-dev
```

Inspect recent watcher output without attaching or sending input:

```bash
tmux capture-pane -p -t bionic-dev:app.0 -S -200
tmux capture-pane -p -t bionic-dev:app.1 -S -200
tmux capture-pane -p -t bionic-dev:app.2 -S -200
```

Use the existing watcher as the primary compilation feedback loop when it
covers the changed code. Captured output must relate to the most recent edit;
old successful output is not evidence that a new change compiled. If a watcher
reports an error, fix it and capture the relevant pane again.

Watcher compilation proves only that the watched target compiled. It does not
prove tests, runtime behavior, browser behavior, or service integration.
Run focused tests and application-level checks when they are relevant.

Run an independent `cargo check`, `cargo build`, or workspace test only when:

- `bionic-dev` is not running;
- the watcher does not cover the affected crate or target;
- watcher output is unavailable or inconclusive;
- a separate validation step requires it; or
- comprehensive verification was requested.

If the session is absent, do not initialize it automatically. Use an
appropriate non-interactive `cargo check`, focused build, or focused test as a
fallback where possible.

Do not run `just dev`, `just stop`, `dev-init`, `dev-setup`, or deployment and
restart recipes as part of normal agent work. Do not rely on interactive shell
aliases. Use complete underlying commands or existing non-interactive
`Justfile` recipes, and inspect `Justfile` before using an unfamiliar recipe.
