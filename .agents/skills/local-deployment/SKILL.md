---
name: local-deployment
description: Deploy the locally built Bionic web application into the user's k3d cluster. Trigger only when the user explicitly asks for a local, k3d, or Kubernetes deployment.
---

# Local Deployment

Use this skill only for an explicit user request to deploy Bionic locally or to
the k3d development cluster. Do not trigger it for ordinary builds, tests,
development-server work, or environment diagnosis.

Run the bundled script from the repository root:

```bash
bash .agents/skills/local-deployment/deploy-to-k3d-locally.sh
```

The script builds the web assets and web server, creates a temporary Docker
image, imports it into `k3d-bionic`, patches the `bionic-gpt` StackApp, waits
for the rollout, and reports the application URL. The generated image includes
the Debian CA certificate bundle required by the Rust HTTPS clients.

Prerequisites:

- Docker, k3d and kubectl are installed and available as explicit commands.
- The `k3d-bionic` cluster exists.
- kubectl is using the `k3d-k3d-bionic` context.
- The `bionic-gpt` namespace and StackApp are available.

This is a state-changing operation. Confirm the user's request before running
it. Never run it automatically as part of compilation, testing, or diagnosis.
If it fails, report the command output and do not retry or modify the cluster
unless the user requests that.
