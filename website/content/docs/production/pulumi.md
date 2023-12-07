+++
title = "Installing The Application"
weight = 60
sort_by = "weight"
+++

Now we have a namespace in our cluster with hte pre-requisites we can install the application.

We provide a [Pulumi.yaml](https://github.com/bionic-gpt/bionic-gpt/blob/main/Pulumi.yaml) file that will install the corresponding components into the `bionic-gpt` namespace.

A snippet of the file is shown below.

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
    ...
```

## Pulumi up

Run `pulumi up` and check that all the deployments are running correctly.

If the wind is blowing in ther right direction you'll hopefully get something that looks like below.

```sh
$ pulumi up
Please choose a stack, or create a new one: bionic-gpt-app
Previewing update (bionic-gpt-app)

View in Browser (Ctrl+O): https://app.pulumi.com/ianpurton/bionic-gpt/bionic-gpt-app/previews/...

     Type                              Name                       Plan       
 +   pulumi:pulumi:Stack               bionic-gpt-bionic-gpt-app  create     
 +   ├─ kubernetes:apps/v1:Deployment  unstructured-deployment    create     
 +   ├─ kubernetes:apps/v1:Deployment  embeddings-deployment      create     
 +   ├─ kubernetes:apps/v1:Deployment  app-deployment             create     
 +   ├─ kubernetes:apps/v1:Deployment  auth-deployment            create     
 +   ├─ kubernetes:core/v1:Service     envoy-service              create     
 +   ├─ kubernetes:core/v1:Service     app-service                create     
 +   ├─ kubernetes:core/v1:Service     auth-service               create     
 +   ├─ kubernetes:core/v1:Service     unstructured-servce        create     
 +   ├─ kubernetes:core/v1:Service     embeddings-service         create     
 +   └─ kubernetes:apps/v1:Deployment  envoy-deployment           create     

Resources:
    + 11 to create

Do you want to perform this update? yes
Updating (bionic-gpt-app)

View in Browser (Ctrl+O): https://app.pulumi.com/ianpurton/bionic-gpt/bionic-gpt-app/updates/12

     Type                              Name                       Status              
 +   pulumi:pulumi:Stack               bionic-gpt-bionic-gpt-app  created (0.58s)     
 +   ├─ kubernetes:core/v1:Service     embeddings-service         created (10s)       
 +   ├─ kubernetes:apps/v1:Deployment  envoy-deployment           created (1s)        
 +   ├─ kubernetes:core/v1:Service     auth-service               created (10s)       
 +   ├─ kubernetes:core/v1:Service     app-service                created (11s)       
 +   ├─ kubernetes:core/v1:Service     unstructured-servce        created (11s)       
 +   ├─ kubernetes:apps/v1:Deployment  app-deployment             created (4s)        
 +   ├─ kubernetes:apps/v1:Deployment  embeddings-deployment      created (3s)        
 +   ├─ kubernetes:apps/v1:Deployment  unstructured-deployment    created (4s)        
 +   ├─ kubernetes:core/v1:Service     envoy-service              created (12s)       
 +   └─ kubernetes:apps/v1:Deployment  auth-deployment            created (5s)        

Resources:
    + 11 created

Duration: 15s
```

## Accessing the Bionic-GPT in your cluster

