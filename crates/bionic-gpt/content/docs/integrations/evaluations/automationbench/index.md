# AutomationBench

[AutomationBench](https://github.com/zapier/AutomationBench) is a benchmark for evaluating AI agents on realistic business workflows. Read the [AutomationBench white paper](https://arxiv.org/abs/2604.18934) and the [source repository](https://github.com/zapier/AutomationBench).

This guide runs the simulated SaaS APIs as one local service. Bionic connects to the generated OpenAPI specifications like any other integration, and all services share one simulated world.

## Start the API container

```bash
docker pull ghcr.io/bionic-gpt/integration-simulator:latest
docker run --rm --name integration-simulator -p 8880:8080 \
  ghcr.io/bionic-gpt/integration-simulator:latest
```

Check readiness with `curl http://localhost:8880/health`.

## Download the OpenAPI specifications

The versioned simulator image includes the OpenAPI specifications used by the service:

```bash
container=$(docker create ghcr.io/bionic-gpt/integration-simulator:latest)
docker cp "$container:/specs" ./integration-simulator-openapi
docker rm "$container"
```

## Load every service into Bionic

Run [`import-automationbench.sh`](import-automationbench.sh) against a local evaluation database:

```bash
BIONIC_DATABASE_URL='postgresql://db-owner:testpassword@localhost:30001/bionic-gpt?sslmode=disable' \
BIONIC_TEAM_ID=5 BIONIC_USER_ID=1 ./import-automationbench.sh
```

The helper is idempotent, creates team-visible authentication-free OpenAPI integrations, and does not create API-key or OAuth connections. The adapter exposes `/admin/world` for loading an `initial_state` and `/admin/reset` for starting over.

## Reset for a benchmark task

For repeatable evaluation runs, reset the shared world from the task name rather than constructing the state by hand:

```bash
curl -X POST http://localhost:8880/benchmark/reset \
  -H 'Content-Type: application/json' \
  -d '{"task":"simple.email_sf_contact_phone_update"}'
```

The response includes the task name, the services allowed for that task, and the initialized world. This replaces the current world for every caller of the container, so reset before starting each evaluation. Use `/admin/reset` for an empty world or `/admin/world` when you already have a serialized `initial_state` object.
