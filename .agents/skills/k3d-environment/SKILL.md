---
name: k3d-environment
description: Set up or reconfigure Bionic's local k3d Kubernetes environment. Trigger only when the user explicitly asks to initialize, configure, or set up the local k3d environment.
---

# k3d Environment

Use this skill only for an explicit request to create, initialize, configure,
or set up the local k3d environment. Do not trigger it for ordinary builds,
tests, watcher startup, diagnosis, or application deployment.

Run the scripts from the repository root:

```bash
bash .agents/skills/k3d-environment/dev-init.sh
bash .agents/skills/k3d-environment/dev-setup.sh
bash .agents/skills/k3d-environment/get-config.sh
```

`dev-init.sh` deletes and recreates the `k3d-bionic` cluster, then updates the
user kubeconfig. `dev-setup.sh` installs and deploys the development and
Selenium Stack resources. `get-config.sh` changes the user's kubeconfig,
including disabling TLS verification for local development.

Prerequisites:

- k3d, kubectl, stack, iproute2 and sudo are installed.
- The commands are run from a checkout of this repository.
- The user understands that these scripts change local Kubernetes resources or
  host configuration.

These scripts are state-changing and must never run automatically. Confirm the
user explicitly requested the relevant operation before executing it. Do not
run `dev-init.sh` merely because the cluster is unavailable; it destroys the
existing cluster.
