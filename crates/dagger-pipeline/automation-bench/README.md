# AutomationBench HTTP adapter

This adapter exposes AutomationBench's simulated APIs through ordinary OpenAPI
documents and one HTTP service. It does not reimplement any SaaS behavior.

The Dagger pipeline checks out AutomationBench, generates the OpenAPI documents,
runs validation/tests, and builds the resulting image. CI publishes the image
to GHCR; local builds export it to the host image store.

## Local build

```bash
cargo run -p dagger-pipeline -- \
  automationbench \
  --automationbench-ref main \
  --tag bionic-gpt-automationbench:local

docker run --rm -p 8080:8080 bionic-gpt-automationbench:local
```

The generated OpenAPI documents are exported to `automationbench-openapi/`.
The image contains the exact upstream revision in
`/app/automationbench-commit.txt`.

Initialize a world and call Gmail through the generated API:

```bash
curl -X POST localhost:8080/admin/world -H content-type:application/json -d '{"initial_state": {}}'
curl 'localhost:8080/api/gmail/gmail/v1/users/me/messages'
```

Reset directly to a named benchmark task:

```bash
curl -X POST localhost:8080/benchmark/reset \
  -H 'content-type: application/json' \
  -d '{"task":"simple.email_sf_contact_phone_update"}'
```

The container keeps one shared `WorldState`; `/admin/reset` replaces it and
`/admin/world` returns its current serialized value. Generated OpenAPI files
are served directly from `/openapi/<service>.yaml`; the files themselves are
also included in the image.
