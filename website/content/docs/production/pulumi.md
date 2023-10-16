+++
title = "Pulumi.yaml"
description = "Pulumi.yaml"
weight = 60
sort_by = "weight"
+++

One we have a cluster that is ready for BionicGPT we can install the docker containers.

We provide a Pulumi.yaml file that will install the corresponding components into a `bionicgpt` namespace.

```yml
# Pulumi deployment setup

name: bionicgpt
description: BionicGPT Infrastructure as Code
runtime: yaml
variables:
    namespace: bionicgpt
    appLabels:
        app: app
    authLabels:
        app: auth
    version: 1.0.3
    hash-bionicgpt: sha256:4f14c4836bd1b6275b43513ea5cf169c82b1fef4bbf478905ce0a2f2eff7a234
    hash-bionicgpt-envoy: sha256:207236133eb39fc413b68cfaff3e9fa41f6d5137de40aad83fded694ba678eda
    hash-bionicgpt-db-migrations: sha256:6a6c52ee24701d34cc1d9a3602ce62cc22be8ee69bd099a861de4578807df2ae
    db-migrations: ghcr.io/purton-tech/bionicgpt-db-migrations:${version}@${hash-bionicgpt-db-migrations}
    server: ghcr.io/purton-tech/bionicgpt-server:${version}@${hash-bionicgpt}
    envoy: ghcr.io/purton-tech/bionicgpt-envoy:${version}@${hash-bionicgpt-envoy}
```

## Pulumi up

Run `pulumi up` and check that all the deployments are running correctly.