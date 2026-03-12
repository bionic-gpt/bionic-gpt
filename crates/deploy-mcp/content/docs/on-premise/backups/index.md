# Backing up Deploy MCP Data

Maintaining reliable backups is essential to keep Deploy MCP state recoverable after accidental deletion, operator error, or infrastructure failure. The Deploy MCP Kubernetes operator provisions a PostgreSQL cluster for conversation history and configuration metadata; this guide shows how to point that database at hardened, encrypted object storage using `kubectl patch`.

## Prerequisites

Ensure you have:

- `kubectl` configured against the cluster that runs Deploy MCP.
- Permissions to read and patch resources in the `deploy-mcp` namespace.
- Credentials for your S3-compatible object store (for example AWS S3, MinIO, or Cloudflare R2).
- A client-side encryption key you control.

## 1. Create Secrets for Storage and Encryption

First create a secret containing the object storage access keys:

```sh
kubectl create secret generic s3-creds \
  --namespace deploy-mcp \
  --from-literal=S3_COMPATIBLE_ACCESS_KEY=your_access_key \
  --from-literal=S3_COMPATIBLE_SECRET_KEY=your_secret_key
```

Next create the encryption key secret:

```sh
kubectl create secret generic backup-encryption-key \
  --namespace deploy-mcp \
  --from-literal=ENCRYPTION_KEY=your_encryption_key
```

Replace the placeholder values with your actual credentials. Lock down these secrets with RBAC and, if possible, encrypt secrets at rest in your cluster.

## 2. Patch the Deploy MCP Database Cluster

The Deploy MCP operator creates a `deploy-mcp-db-cluster` custom resource. Patch it so backups ship to your object storage with client-side encryption:

```sh
kubectl patch cluster deploy-mcp-db-cluster \
  -n deploy-mcp \
  --type merge \
  -p '{
    "spec": {
      "backup": {
        "barmanObjectStore": {
          "destinationPath": "s3://YOUR_BUCKET_NAME",
          "endpointURL": "https://YOUR_ENDPOINT_URL",
          "s3Credentials": {
            "accessKeyId": {
              "name": "s3-creds",
              "key": "S3_COMPATIBLE_ACCESS_KEY"
            },
            "secretAccessKey": {
              "name": "s3-creds",
              "key": "S3_COMPATIBLE_SECRET_KEY"
            }
          },
          "encryption": {
            "clientSide": {
              "encryptionKeySecret": {
                "name": "backup-encryption-key",
                "key": "ENCRYPTION_KEY"
              }
            }
          }
        }
      }
    }
  }'
```

Swap in your bucket name and endpoint URL. Specify a region if your object store requires it by adding `region: YOUR_REGION` under `barmanObjectStore`.

## 3. Validate the Configuration

Confirm the patch applied correctly:

```sh
kubectl get cluster deploy-mcp-db-cluster -n deploy-mcp -o yaml | grep -A10 backup:
```

You should see the destination path, endpoint, and secrets reflected in the resource status.

## 4. Trigger a Test Backup

Run a manual backup to confirm credentials and permissions are correct:

```sh
kubectl exec -n deploy-mcp deployment/deploy-mcp-db \
  -- backup-command --trigger-backup
```

Replace `backup-command` with the backup tool’s CLI entry point configured in your environment. Monitor the associated Kubernetes job or pod logs to verify the run succeeds.

Check the backup controller status:

```sh
kubectl get backups -n deploy-mcp
```

You should see the new backup marked as completed.

## 5. Automate Ongoing Verification

- Schedule recurring restore drills from the object store into a staging namespace.
- Monitor backup jobs with your observability stack so failed runs trigger alerts.
- Rotate S3 credentials and encryption keys regularly, updating the secrets and reapplying the patch when they change.

With these controls in place, your Deploy MCP data stays protected and compliant with enterprise retention and disaster recovery requirements.
