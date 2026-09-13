# AutomationBench HTTP adapter

This adapter exposes AutomationBench's simulated APIs through ordinary OpenAPI
documents and one HTTP service. It does not reimplement any SaaS behavior.

The workflow checks out AutomationBench, generates the OpenAPI documents, runs
validation/tests, and publishes the resulting image to GHCR.

## Local build

```bash
python tools/generate.py --automationbench ../AutomationBench --output dist
python tools/validate.py --automationbench ../AutomationBench --output dist
cp -R ../AutomationBench/automationbench dist/automationbench
docker build -t automationbench-api dist
docker run --rm -p 8080:8080 automationbench-api
```

Initialize a world and call Gmail through the generated API:

```bash
curl -X POST localhost:8080/admin/world -H content-type:application/json -d '{"initial_state": {}}'
curl 'localhost:8080/api/gmail/gmail/v1/users/me/messages'
```

The container keeps one shared `WorldState`; `/admin/reset` replaces it and
`/admin/world` returns its current serialized value. Generated OpenAPI files
are served directly from `/openapi/<service>.yaml`; the files themselves are
also included in the image.
