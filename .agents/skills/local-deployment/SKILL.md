---
name: local-deployment
description: Deploy one or more locally built Bionic services into the user's k3d cluster. Trigger only when the user explicitly asks for a local, k3d, or Kubernetes deployment.
---

# Local Deployment

Use this skill for an explicit request to deploy Bionic services locally or to
the k3d development cluster.

Run the bundled script from the repository root:

```bash
bash .agents/skills/local-deployment/deploy-to-k3d-locally.sh [service ...]
```

Service names match `spec.services` in the StackApp. Pass several names to
deploy them together, for example `web cli-gateway`; omitting names deploys
`web`. The script builds each local Cargo binary, imports its image into
`k3d-bionic`, patches that service, and waits for its rollout. Web assets and
the CLI gateway's Typst runtime are packaged automatically.

Web deployments enable Bionic debug logging through `LOG_LEVEL` and targeted
Rig request and stream tracing through `RIG_LOG`.

Prerequisites:

- Docker, k3d and kubectl are installed and available as explicit commands.
- The `k3d-bionic` cluster exists.
- kubectl is using the `k3d-k3d-bionic` context.
- The `bionic-gpt` namespace and StackApp are available.

Inspect web traces after deployment with:

```bash
kubectl logs deployment/bionic-gpt --namespace bionic-gpt --tail=200
```

Look for `agent_runtime` debug events, outgoing requests under
`rig::completions`, and incoming SSE frames under `rig::streaming`. These
settings are applied by the local deployment script only and are not added to
production StackApp manifests.
