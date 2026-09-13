# AutomationBench

[AutomationBench](https://github.com/zapier/AutomationBench) is a benchmark for evaluating AI agents on realistic business workflows. Read the [AutomationBench white paper](https://arxiv.org/abs/2604.18934) and the [source repository](https://github.com/zapier/AutomationBench).

This guide runs the simulated SaaS APIs as one local service. Bionic connects to the generated OpenAPI specifications like any other integration, and all services share one simulated world.

## Start the API container

```bash
docker pull ghcr.io/bionic-gpt/automationbench-api:4a8e1061254004d9dac807054eed33fad7d1ff14
docker run --rm --name automationbench-api -p 8880:8080 \
  ghcr.io/bionic-gpt/automationbench-api:4a8e1061254004d9dac807054eed33fad7d1ff14
```

Check readiness with `curl http://localhost:8880/health`.

## Download the OpenAPI specifications

The GitHub Actions build publishes all specifications as the `automationbench-openapi` artifact:

```bash
gh run list --workflow build-automationbench.yml --limit 1
gh run download RUN_ID --name automationbench-openapi --dir automationbench-openapi
```

You can also extract them directly from the image:

```bash
container=$(docker create ghcr.io/bionic-gpt/automationbench-api:4a8e1061254004d9dac807054eed33fad7d1ff14)
docker cp "$container:/app/openapi" ./automationbench-openapi
docker rm "$container"
```

## Load every service into Bionic

Run [`import-automationbench.sh`](import-automationbench.sh) against a local evaluation database:

```bash
BIONIC_DATABASE_URL='postgresql://db-owner:testpassword@localhost:30001/bionic-gpt?sslmode=disable' \
BIONIC_TEAM_ID=5 BIONIC_USER_ID=1 ./import-automationbench.sh
```

The helper is idempotent, creates team-visible authentication-free OpenAPI integrations, and does not create API-key or OAuth connections. The adapter exposes `/admin/world` for loading an `initial_state` and `/admin/reset` for starting over.
