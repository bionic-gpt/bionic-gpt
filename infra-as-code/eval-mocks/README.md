# Eval Mocks

The editable OpenAPI specs live in:

```text
openapi/specs/
```

Do not edit files under `openapi/generated/`. That directory is local build
output and is ignored by git.

## Generate the Combined Spec

After editing a spec, generate the combined Mockoon spec:

```bash
just eval-mocks-spec
```

This writes:

```text
openapi/generated/eval-mocks.openapi.yaml
```

## Use with Docker Compose

In `infra-as-code/docker-compose.yml`, uncomment the eval-mocks volume:

```yaml
volumes:
  - ./eval-mocks/openapi/generated:/home/mockoon/data:ro
```

Then restart the mock API service:

```bash
docker compose -f infra-as-code/docker-compose.yml up -d eval-mocks
```

Mockoon will read:

```text
/home/mockoon/data/eval-mocks.openapi.yaml
```

## Test Without Compose

From the repository root:

```bash
just eval-mocks-spec
docker run --rm -p 3100:3100 \
  -v "$PWD/infra-as-code/eval-mocks/openapi/generated:/home/mockoon/data:ro" \
  mockoon/cli:9.7.0 start \
  --data /home/mockoon/data/eval-mocks.openapi.yaml \
  --port 3100 \
  --hostname 0.0.0.0
```

Then test an endpoint:

```bash
curl http://localhost:3100/web/search?query=sovereign%20ai
```
