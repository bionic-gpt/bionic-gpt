---
name: automationbench-eval
description: Set up and run the pinned AutomationBench API adapter, retrieve its OpenAPI services, and provision them as Bionic integrations for a selected local team.
---

# AutomationBench evaluation

Use this skill when the user asks to inspect or provision the local
AutomationBench evaluation environment.

## Defaults

- Image: `ghcr.io/bionic-gpt/automationbench-api:4a8e1061254004d9dac807054eed33fad7d1ff14`
- OpenAPI files: `/app/openapi` in the image

The adapter is deployed as the `automationbench-api` service by the local
Stack `dev` profile. Bionic reaches it inside the cluster at
`http://automationbench-api:8080`. It is intentionally not exposed on a host
port by this skill.

The adapter has one shared world; use `/admin/world` to seed an AutomationBench
`initial_state` and `/admin/reset` between evaluations when operational access
is available through the local cluster tooling.

## Provisioning integrations

The reusable importer is `scripts/import-integrations.sh`. It extracts every
OpenAPI YAML from the pinned image and inserts authentication-free, team-visible
OpenAPI integrations into Bionic PostgreSQL.

Before running it, identify the target team and user, inspect existing rows,
and obtain explicit authorization immediately before replacement. The importer
only deletes rows for the selected team and must never affect other teams.

Required variables are `BIONIC_DATABASE_URL`, `BIONIC_TEAM_ID`, and
`BIONIC_USER_ID`; `AUTOMATIONBENCH_IMAGE` may override the pinned image. After
provisioning, verify the row count, names, visibility, and that no connections
were created.
