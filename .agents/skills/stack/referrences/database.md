# Stack Database

Read this reference when a task changes or diagnoses PostgreSQL infrastructure
managed by Stack.

## Provisioning

Declare PostgreSQL in the base StackApp components:

```yaml
apiVersion: stack-cli.dev/v1
kind: StackApp
metadata:
  name: my-app
  namespace: my-app
spec:
  components:
    db: {}
```

Stack provisions a CloudNativePG cluster in the application's namespace. Do
not add a separate PostgreSQL Deployment or manually duplicate Stack-generated
database secrets.

## Roles and Intended Use

Stack bootstraps roles for different trust levels:

| Role | Intended use |
|---|---|
| `db-owner` | Migrations and administrative schema changes |
| `application_user` | Normal application runtime access |
| `application_readonly` | Read-only application or reporting access |
| `authenticator`, `anon` | PostgREST access |
| `service_role`, `authenticated` | Auth-aware backend workloads |

Use the least-privileged role that supports the service. In particular, do not
run the application with the migration or owner connection.

## Service Wiring

Stack stores generated connection strings in the `database-urls` secret. Wire
the appropriate connection into a service rather than copying a URL into
`env`:

```yaml
spec:
  services:
    web:
      image: ghcr.io/example/my-app:latest
      port: 8080
      database_url: APP_DATABASE_URL
      init:
        image: ghcr.io/example/my-app-migrations:latest
        migrations_database_url: DATABASE_URL

    reports:
      image: ghcr.io/example/reports:latest
      readonly_database_url: READONLY_DATABASE_URL
```

The field names select the privilege level; their values name the environment
variables injected into the container. Preserve a repository's established
environment variable names when modifying an existing service.

## Profiles and Local Access

Expose PostgreSQL only in a development profile when host access is required:

```yaml
spec:
  components:
    db: {}
  profiles:
    dev:
      components:
        db:
          expose_db_port: 30001
          danger_override_password: testpassword
```

`danger_override_password` is suitable only for disposable local development.
Never copy that setting into shared, UAT, or production configuration.

## Inspection and Diagnosis

Resolve the manifest's actual namespace and database cluster name before using
`kubectl`. Useful read-only checks include:

```bash
kubectl -n <namespace> get pods
kubectl -n <namespace> get clusters.postgresql.cnpg.io
kubectl -n <namespace> describe secret database-urls
```

Do not print or decode `database-urls` unless the task genuinely requires a
connection value and the output can be handled safely.

For an explicitly requested interactive database session, derive the pod and
database names from the deployed resources instead of assuming demo names:

```bash
kubectl -n <namespace> exec -it <database-pod> -- psql -d <database>
```

## Change Boundaries

- Stack provisions the cluster, roles, secrets, and service connections.
- Migration tooling owns application schemas and extensions required by the
  application.
- Backup, restore, resizing, storage-class, or production topology changes need
  inspection of the installed Stack and CloudNativePG versions before editing.
- A manifest edit does not authorize deployment. Show or validate the change
  without applying it unless deployment was explicitly requested.