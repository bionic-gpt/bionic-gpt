# Backing up your Bionic and Keycloak Databases

Maintaining reliable backups is crucial for any Kubernetes cluster to ensure data integrity and facilitate disaster recovery. In this guide, we'll walk through how to update your clusters' backup configurations using the `kubectl patch` command. This method allows you to seamlessly integrate S3-compatible object storage for backups with client-side encryption for multiple clusters.

## Prerequisites

Before proceeding, ensure you have the following:

- **kubectl Installed**: Ensure you have `kubectl` installed and configured to communicate with your Kubernetes cluster.
- **Appropriate Permissions**: You must have the necessary permissions to patch resources within the specified namespaces.
- **S3-Compatible Storage Details**: Obtain your bucket name, endpoint URL, and credentials for accessing your S3-compatible storage.
- **Encryption Key**: Have a secret containing your encryption key ready.

## Step-by-Step Guide

### 1. Understand the `kubectl patch` Command

The `kubectl patch` command allows you to update Kubernetes resources declaratively. In this scenario, we're patching two cluster resources—`bionic-gpt-db-cluster` and `keycloak-db-cluster`—to update their backup configurations.

### 2. Secure Your Credentials

Before applying any patches, ensure that your Kubernetes secrets (`s3-creds` and `backup-encryption-key`) are securely created and stored. This guarantees that both clusters can access the necessary credentials and encryption keys for their backup processes.

#### Creating the S3 Credentials Secret

Create a Kubernetes secret named `s3-creds` in the `bionic-gpt` namespace containing your S3-compatible access and secret keys:

```sh
kubectl create secret generic s3-creds \
  --namespace bionic-gpt \
  --from-literal=S3_COMPATIBLE_ACCESS_KEY=your_access_key \
  --from-literal=S3_COMPATIBLE_SECRET_KEY=your_secret_key
```

#### Creating the Encryption Key Secret

Create another Kubernetes secret named `backup-encryption-key` in the `bionic-gpt` namespace containing your encryption key:

```sh
kubectl create secret generic backup-encryption-key \
  --namespace bionic-gpt \
  --from-literal=ENCRYPTION_KEY=your_encryption_key
```

*Replace `your_access_key`, `your_secret_key`, and `your_encryption_key` with your actual credentials.*

> **Security Best Practices:**
>
> - **Restrict Secret Access**: Limit access to these secrets to only the necessary service accounts and namespaces.
> - **Encrypt Secrets at Rest**: Ensure your Kubernetes cluster is configured to encrypt secrets at rest.
> - **Use RBAC**: Implement Role-Based Access Control (RBAC) to manage permissions effectively.

### 3. Apply the Patch to Both Clusters

With the necessary secrets in place, you can now apply the backup configuration patches to both `bionic-gpt-db-cluster` and `keycloak-db-cluster`. Both clusters will use the same S3 bucket and credentials for their backups.

#### Patch Payload

The following JSON payload configures the backup settings to use an S3-compatible object store with client-side encryption:

```json
{
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
}
```

*Replace `YOUR_BUCKET_NAME` and `YOUR_ENDPOINT_URL` with your actual S3 bucket name and endpoint URL.*

#### Patching `bionic-gpt-db-cluster`

Apply the patch to the `bionic-gpt-db-cluster` in the `bionic-gpt` namespace:

```sh
kubectl patch cluster bionic-gpt-db-cluster \
  -n bionic-gpt \
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

#### Patching `keycloak-db-cluster`

Apply the same patch to the `keycloak-db-cluster` in the `keycloak-namespace` namespace:

```sh
kubectl patch cluster keycloak-db-cluster \
  -n keycloak-namespace \
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

> **Note:** Ensure that both clusters (`bionic-gpt-db-cluster` and `keycloak-db-cluster`) reside in their respective namespaces (`bionic-gpt` and `keycloak-namespace`). Adjust the namespace in the commands if necessary.

### 4. Automate Patching Multiple Clusters (Optional)

If you have multiple clusters to patch, you can automate the process using a simple shell script. Here's an example:

```sh
#!/bin/bash

# Define an array of clusters and their namespaces
declare -A clusters
clusters=( 
  ["bionic-gpt-db-cluster"]="bionic-gpt" 
  ["keycloak-db-cluster"]="keycloak-namespace" 
  # Add more clusters and namespaces as needed
)

# Define common patch payload
read -r -d '' PATCH_PAYLOAD << EOM
{
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
}
EOM

# Iterate over clusters and apply patch
for cluster in "${!clusters[@]}"; do
  namespace=${clusters[$cluster]}
  echo "Patching cluster: $cluster in namespace: $namespace"
  kubectl patch cluster "$cluster" \
    -n "$namespace" \
    --type merge \
    -p "$PATCH_PAYLOAD"
done
```

> **Usage:**
>
> 1. Save the script to a file, e.g., `patch-clusters.sh`.
> 2. Make it executable: `chmod +x patch-clusters.sh`.
> 3. Run the script: `./patch-clusters.sh`.

This script allows you to easily add more clusters by updating the `clusters` array.

### 5. Verify the Update

After applying the patches to both clusters, verify that the backup configurations have been updated correctly:

#### For `bionic-gpt-db-cluster`

```sh
kubectl get cluster bionic-gpt-db-cluster -n bionic-gpt -o yaml
```

#### For `keycloak-db-cluster`

```sh
kubectl get cluster keycloak-db-cluster -n keycloak-namespace -o yaml
```

Look for the `backup` section in each output to ensure all fields reflect your intended configuration.

### 6. Test the Backup Configuration

It's essential to verify that backups are functioning as expected. Depending on your backup tool, you can perform a test backup and restore to ensure everything is configured correctly.

#### Example: Initiate a Manual Backup

```sh
kubectl exec -n bionic-gpt deployment/bionic-gpt-db -- backup-command --trigger-backup
kubectl exec -n keycloak-namespace deployment/keycloak-db -- backup-command --trigger-backup
```

*Replace `backup-command` with the actual command used to trigger backups in your environment.*

#### Check Backup Status

```sh
kubectl get backups -n bionic-gpt
kubectl get backups -n keycloak-namespace
```

Ensure that the backups complete successfully and are stored in your specified S3 bucket.

## Best Practices

- **Use Separate Namespaces for Secrets**: Consider storing secrets in a dedicated namespace with strict access controls to enhance security.
  
- **Rotate Credentials Regularly**: Periodically update your S3 credentials and encryption keys to minimize security risks.
  
- **Validate Patch Syntax**: Ensure your JSON payload is correctly formatted to prevent patch failures. Tools like [jq](https://stedolan.github.io/jq/) can help validate JSON structures.
  
- **Backup Before Patching**: Always back up your current cluster configuration before making changes. This allows you to revert if something goes wrong.
  
- **Monitor Backup Processes**: Implement monitoring and alerting for your backup processes to promptly detect and address any issues.

## Conclusion

Updating your Kubernetes clusters' backup configurations using `kubectl patch` is a powerful way to integrate secure and reliable storage solutions across multiple clusters. By following the steps outlined above, you can ensure that your backups are stored safely in an S3-compatible object store with client-side encryption, enhancing your clusters' resilience and data protection.

For further customization and advanced configurations, refer to the [Kubernetes documentation](https://kubernetes.io/docs/home/) and your backup tool's specific guidelines.