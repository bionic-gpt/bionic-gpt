# Secure On-Premise Overview

Deploy MCP is designed to run inside your own Kubernetes footprint so you can deliver Model Context Protocol servers with the same resiliency and controls you expect from business-critical workloads. This approach lets you reuse your existing cluster security posture, identity, and observability while keeping sensitive prompts and data inside your boundary.

## Why Kubernetes

Kubernetes gives us declarative rollouts, automatic healing, and horizontal scaling, all of which are essential when MCP servers must stay responsive even under bursty loads. It also provides strong workload isolation through namespaces, pod security, and network policies, making it straightforward to segment assistants by team or trust level. Whether you operate a lightweight K3s cluster in a lab or a multi-zone EKS or GKE environment, the Deploy MCP operator keeps the control plane consistent and auditable.

## Hardened Container Supply Chain

Every Deploy MCP container image is built from reproducible sources, scanned for vulnerabilities, and signed before publication. We maintain SOC 2 aligned controls across our CI/CD and artifact storage, and surface image provenance so you can verify what is running in production. Images are pushed to `ghcr.io/deploy-mcp`, with digests published in our release notes for deterministic pinning.

## Provide Registry Credentials

Because our registry is private, your cluster must be configured with credentials before the operator can pull images. The typical flow is to create a pull secret in the target namespace and reference it from the service accounts used by Deploy MCP:

```bash
kubectl create secret docker-registry deploy-mcp-registry \
  --docker-server=ghcr.io \
  --docker-username="YOUR_GH_USERNAME" \
  --docker-password="YOUR_GH_PAT" \
  --namespace=deploy-mcp

kubectl patch serviceaccount default \
  --namespace=deploy-mcp \
  --type merge \
  --patch '{"imagePullSecrets": [{"name": "deploy-mcp-registry"}]}'
```

If you operate in a more restricted environment (for example, air-gapped clusters), mirror the images into your internal registry and update the operator configuration to point to your copy.

## Defense in Depth

Deploy MCP integrates cleanly with the rest of your platform controls: use Kubernetes Secrets or external secret managers for credentials, employ network policies to restrict ingress to operator-managed services, and plug pods into your monitoring stack for real-time insight. Combined with the operator’s reconciliation loop, you get a continuously enforced, secure baseline for MCP services that scales with your organisation.
