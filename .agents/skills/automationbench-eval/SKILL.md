---
name: automationbench-eval
description: Load the repository's AutomationBench OpenAPI catalogue and run evaluations against the local simulator.
---

# AutomationBench evaluation

Use this skill when the user asks to load AutomationBench specs or run an
AutomationBench evaluation.

## Source of truth

The OpenAPI source files are committed under:

```text
crates/integration-simulator/specs/*.openapi.json
```

The loader normalizes every imported spec to the local simulator at
`http://integration-simulator:8080/api/<service>`. It also removes OpenAPI
security declarations, security schemes, and explicit access-token parameters
because the simulator is authentication-free. Business parameters are kept.

## Load the global spec catalogue

Run the repository loader from the repository root:

```bash
bash .agents/skills/automationbench-eval/scripts/load-specs.sh
```

The script uses `DATABASE_URL`, falling back to `APP_DATABASE_URL`. A custom
spec directory can be supplied as its first argument:

```bash
bash .agents/skills/automationbench-eval/scripts/load-specs.sh \
  crates/integration-simulator/specs
```

It upserts global rows in `integrations.openapi_specs` using each file's
filename, `info.title`, `info.description`, and `info.x-logo.url`. Rows are
active Application specs and are deliberately non-system so they appear in
the existing integration selection screen. The loader does not need a team
ID or user ID, does not create team integrations, and does not create API-key
or OAuth connections.

The loader is safe to rerun. It updates only rows with matching AutomationBench
slugs and does not delete unrelated specs or integrations.

After loading, open the Bionic integrations selection screen and select only
the services required for the evaluation. The selection screen creates the
team-specific integrations using the already-loaded global definitions.

## Simulator

The AutomationBench API is provided by the `integration-simulator`
service by the local Stack `dev` profile. Bionic reaches it inside the cluster
at `http://integration-simulator:8080`. It has one shared deterministic world.

Reset the world before each evaluation:

```bash
curl -X POST http://localhost:8880/benchmark/reset \
  -H 'Content-Type: application/json' \
  -d '{"task":"simple.email_sf_contact_phone_update"}'
```

Use the host port exposed by the local deployment when it differs from
`8880`. The reset replaces the shared simulator world for all callers.
