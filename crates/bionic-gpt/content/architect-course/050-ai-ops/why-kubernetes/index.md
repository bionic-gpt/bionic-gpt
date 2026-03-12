# Why Kubernetes

You’re learning how a GenAI platform comes together: LLM inference, embedding pipelines, vector stores, schedulers, UI, worker pools, and observability all run as separate containers with distinct lifecycles. Docker Compose is great for a laptop demo, but the course prepares you to make architectural decisions later—decisions that assume Kubernetes fluency. We highlight Kubernetes early so every future lab builds intuition around the platform you’ll eventually deploy to.

## Fast Scan for Builders

- **_One API for every role_**: Deploy inference servers, middleware, UI pods, and monitoring agents with the same manifest syntax so teams can version and review your stacks like any other code.
- **_Horizontal scaling without drama_**: Use `kubectl scale` or autoscalers to add GPU-backed inference pods when prompts spike, instead of editing Compose YAML and hoping the node has headroom.
- **_Self-healing primitives_**: Liveness/readiness probes restart stuck workers automatically, keeping RAG pipelines healthy even when a dataset load panics at 2 A.M.
- **_Secret & config discipline_**: `ConfigMap` and `Secret` objects keep API keys, model weights, and prompt variants separated from images so you don’t bake sensitive data into containers.
- **_Built-in multi-tenancy_**: Namespaces isolate experiments, staging, and production clusters so you can run regulated workloads next to prototype agents without accidental cross-talk.
- **_GitOps-friendly_**: Everything is declarative; your team can peer-review cluster changes, roll back safely, and plug into Argo or Flux for automated promotions.

## Why Not Just Docker Compose?

1. Compose is node-bound—every container must live on the same host, so scaling is gated by laptop RAM and a single SSD. Kubernetes schedules pods across any nodes you add.
2. Compose has no native autoscaling, secrets, or health probes. You end up writing scripts that Kubernetes already ships.
3. Running clusters per user with Compose means managing dozens of `.env` variations. Kubernetes lets you templatize deployments and assign RBAC policies so each user only touches their namespace.

## Bottom Line

This lab mirrors the structure of enterprise GenAI programs, so we anchor everything to the platform those programs rely on: Kubernetes. Docker Compose remains perfect for a quick local test, but whenever you need reliability, upgrades, shared GPUs, or policy controls, a lightweight Kubernetes distribution (K3s, k3d, or managed cloud) is the only realistic answer. Treat Kubernetes as essential infrastructure now and the rest of the course will feel consistent with real-world deployments.
