---
name: stack
description: Create or modify StackApp infrastructure-as-code manifests and Stack-managed application infrastructure. Use for Stack components, services, profiles, database wiring, storage, authentication, ingress, secrets, or deployment configuration.
---

# Stack

Use this skill for infrastructure declared with the Stack `StackApp` custom
resource. Stack is language-neutral: configure the infrastructure here, then
use the relevant application skill for framework- or language-specific code.

## Workflow

1. Inspect the existing `*.stack.yaml` manifests and repository deployment
   conventions before editing.
2. Identify whether the change belongs in the base `spec` or an
   environment-specific profile.
3. Prefer a Stack component or service field over adding handwritten
   Kubernetes resources for infrastructure Stack already manages.
4. Preserve existing component, service, namespace, secret, and environment
   variable names unless the requested change requires a migration.
5. Validate the narrowest relevant configuration before proposing a deploy.

Profiles are overlays. Keep shared configuration in the base `spec` and put
only environment differences under `spec.profiles`. Do not duplicate the full
base configuration in each profile.

## References

- For PostgreSQL provisioning, roles, connection secrets, service wiring, or
  development port exposure, read [references/database.md](references/database.md).

Add further capability references, such as storage, authentication, ingress,
and secrets, when their implementation guidance is substantial enough to load
only for those tasks.

## Application Boundaries

Stack owns deployed infrastructure and delivery of configuration to services.
Application skills own schemas, migrations, queries, HTTP handlers, pages, and
business logic. A feature may require both this skill and an application skill.

Do not put application schema design or language-specific client code in a
Stack manifest merely because the application consumes a Stack-managed service.

## Deployment and Safety

Treat `stack init`, `stack deploy`, `kubectl apply`, secret changes, and cluster
configuration changes as state-changing operations. Inspect the active context,
manifest, profile, namespace, and intended environment before running them.
Only deploy or mutate a cluster when the user explicitly requests it.

Do not expose secret values in logs or commit generated credentials. Refer to
secrets by name and use Stack's supported service wiring where available.