---
name: integration-testing
description: Run and maintain Bionic's Selenium and k3d end-to-end tests. Use for browser workflows, multi-service integration tests, and changes under crates/integration-testing.
---

# Integration Testing

Use this skill for tests under `crates/integration-testing` and workflows that
require the Selenium or k3d test environment.

## Repository Workflow

Read `crates/integration-testing/README.md` and the `integration-testing`
recipe in `Justfile` before running tests. The current workflow uses a separate
Bionic deployment in the `bionic-selenium` namespace and Selenium endpoints
configured by the recipe.

Bootstrap the local k3d environment with the verified recipes:

```bash
just dev-init
just dev-setup
```

`dev-init` creates the `k3d-bionic` cluster and writes its kubeconfig;
`dev-setup` deploys the development and Selenium stacks. Wait for the
deployment reconciliation to complete before running tests.

Verified commands include:

```bash
just get-config
just md-selenium
just integration-testing
just integration-testing documents
```

The recipe supplies the test environment, including database, WebDriver,
application, MailHog, and API URLs. For direct database inspection, use the
explicit connection configured by the environment, for example:

```bash
dbmate --no-dump-schema --migrations-dir crates/db/migrations up
psql "$DATABASE_URL"
```

Do not use the interactive `db` or `dbmate` aliases.

NoVNC is available at `http://localhost:30003` with the environment's documented
password when the Selenium stack is running. Individual test names can be
passed through the existing recipe. For failures, rerun with
`RUST_BACKTRACE=1` and include the backtrace in the report.

## Verification

Run a focused test first, then the relevant full integration suite:

```bash
cargo test -p integration-testing <test-filter> -- --nocapture
just integration-testing
```

These tests require k3d, Kubernetes services, a database, and Selenium. If any
dependency is unavailable, report the exact unavailable validation rather than
claiming end-to-end coverage.

For non-browser workspace tests, the repository's standard recipe is:

```bash
just test
```

It excludes `integration-testing` and `rag-engine` by design.
