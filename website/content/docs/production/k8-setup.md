+++
title = "Kubernetes Setup"
weight =15
sort_by = "weight"
+++

We use [Pulumi](https://www.pulumi.com/) as an exmple of installing Bionic GPT using infrastructure as code. Here we walk through the process of what it would like like to setup a Kubernetes cluster ready for a BionicGPT deployment.

## infra-as-code

All the example code is located [here](https://github.com/bionic-gpt/bionic-gpt/tree/main/infra-as-code)



```sh
.
├── .devcontainer/
│   └── ...
├── crates/
│   └── ...
├── infra-as-code/
│   ├── node_modules/
│   │   └── ...
│   └── .gitignore
│   └── index.ts
│   └── package-lock.json
│   └── package.json
│   └── tsconfig.json
├── .gitignore
├── Cargo.toml
└── Cargo.lock
```

## Installing Pulumi

Everything you need to use Pulumi is installed into our `devcontainer`.

You'll need to create a Pulumi [https://www.pulumi.com/](https://www.pulumi.com/) account which is free so that you can get an API key.

## Configuring namespaces and adding a Postgres Operator

Create an `cluster-setup.ts` and add the following.

```typescript
import * as k8s from "@pulumi/kubernetes"
import { Release } from "@pulumi/kubernetes/helm/v3";

export function setupCluster() : Release {
    // Setup a namespace for Cloud Native Pg https://github.com/cloudnative-pg/cloudnative-pg
    const databaseNameSpace = new k8s.core.v1.Namespace('cloud-native-pg', {
        metadata: {
            name: 'cloud-native-pg'
        },
    })

    // Install the Postgres operator from a helm chart
    const cloudnativePg = new k8s.helm.v3.Release("cloudnative-pg", {
        chart: "cloudnative-pg",
        namespace: databaseNameSpace.metadata.name,
        repositoryOpts: {
            repo: "https://cloudnative-pg.github.io/charts",
        }
    });

    return cloudnativePg
}
```

This will be setup that is re-usable across applications, as often in Kubernetes we can install more than one application per cluster.

Change your `index.ts` to look like the following.

```typescript
import * as k8s from "@pulumi/kubernetes"
import * as kx from "@pulumi/kubernetesx"
//import { setupDatabase } from './database'
import { setupCluster } from './cluster-setup'


// Add a postgres operator and anything else apllications need
const cloudnativePg = setupCluster()

// Setup a namespace for our application
const applicationNameSpace = new k8s.core.v1.Namespace('rust-on-nails', {
    metadata: {
        name: 'rust-on-nails'
    },
})
```

OK. Let's run `pulumi up` and see what we get.

```sh
$ pulumi up
Previewing update (dev)

View Live: https://app.pulumi.com/ianpurton/infra-as-code/dev/previews/18c545e4-d7d3-4dbe-bae7-6fc4302304eb

     Type                              Name               Plan       
 +   pulumi:pulumi:Stack               infra-as-code-dev  create     
 +   ├─ kubernetes:core/v1:Namespace   rust-on-nails      create     
 +   ├─ kubernetes:core/v1:Namespace   cloud-native-pg    create     
 +   └─ kubernetes:helm.sh/v3:Release  cloudnative-pg     create     


Resources:
    + 4 to create

Do you want to perform this update? yes
Updating (dev)

View Live: https://app.pulumi.com/ianpurton/infra-as-code/dev/updates/1

     Type                              Name               Status             
 +   pulumi:pulumi:Stack               infra-as-code-dev  created (3s)       
 +   ├─ kubernetes:core/v1:Namespace   rust-on-nails      created (0.36s)    
 +   ├─ kubernetes:core/v1:Namespace   cloud-native-pg    created (0.59s)    
 +   └─ kubernetes:helm.sh/v3:Release  cloudnative-pg     created (14s)      


Resources:
    + 4 created

Duration: 24s
```

## Getting familiar with k9s

[k9s](https://k9scli.io/) is a terminal based UI to interact with your Kubernetes clusters. Fire it up.

```sh
k9s
```

It looks something like the image below and gives you the ability to see running pods and view the logs.

![Adding secrets to cloak](../k9s.jpeg)

## Creating a Database and Users

Create a `database.ts` and add the following code under the code we already created above. 

This code is responsible for creating a namespace called `rust-on-nails` we then install Postgres into that name space and setup a Kubernetes secret called `database-urls` so that our application can connect to the database.

```typescript
import * as pulumi from "@pulumi/pulumi"
import * as k8s from "@pulumi/kubernetes"
import * as kx from "@pulumi/kubernetesx"
import * as random from "@pulumi/random"
import { Release } from "@pulumi/kubernetes/helm/v3";

export function setupDatabase(
    applicationNameSpace: k8s.core.v1.Namespace, 
    cloudnativePg: Release) {

    // Create all the role passwords
    const migrationPassword = new random.RandomPassword("migration_password", {
        length: 20,
        special: false,
    });
    const applicationPassword = new random.RandomPassword("application_password", {
        length: 20,
        special: false,
    });
    const readonlyPassword = new random.RandomPassword("readonly_password", {
        length: 20,
        special: false,
    });
    const authenticationPassword = new random.RandomPassword("authentication_password", {
        length: 20,
        special: false,
    });

    const DATABASE_NAME = "app"
    const MIGRATIONS_ROLE = "migrations"

    const migrationsSecret = new kx.Secret("migrations-secret", {
        type: "kubernetes.io/basic-auth",
        metadata: {
            namespace: applicationNameSpace.metadata.name,
            name: "migrations-secret"
        },
        stringData: {
            "username": MIGRATIONS_ROLE,
            "password": migrationPassword.result,
        }
    })

    const pgCluster = new k8s.apiextensions.CustomResource('nails-db-cluster', {
        apiVersion: 'postgresql.cnpg.io/v1',
        kind: 'Cluster',
        metadata: {
            name: 'nails-db-cluster',
            namespace: applicationNameSpace.metadata.name,
        },
        spec: {
            instances: 1,
            bootstrap: {
                initdb: {
                    database: DATABASE_NAME,
                    // Bootstrap uses the secrets we created
                    // above to give us a user
                    owner: migrationsSecret.stringData.username,
                    secret: {
                        name: migrationsSecret.metadata.name
                    },
                    postInitSQL: [
                        // Add users here.
                        pulumi.all([applicationPassword.result])
                            .apply(([password]) =>
                                `CREATE ROLE application LOGIN ENCRYPTED PASSWORD '${password}'`),
                        pulumi.all([authenticationPassword.result])
                            .apply(([password]) =>
                                `CREATE ROLE authentication LOGIN ENCRYPTED PASSWORD '${password}'`),
                        pulumi.all([readonlyPassword.result])
                            .apply(([password]) =>
                                `CREATE ROLE readonly LOGIN ENCRYPTED PASSWORD '${password}'`)
                    ]
                }
            },
            storage: {
                size: '1Gi'
            }
        }
    }, {
        dependsOn: cloudnativePg
    })

    let migrationsUrl = pulumi.all([migrationPassword.result, pgCluster.metadata.name])
        .apply(([password, host]) =>
            `postgres://${MIGRATIONS_ROLE}:${password}@${host}-rw:5432/${DATABASE_NAME}?sslmode=require`)

    let authenticationUrl = pulumi.all([authenticationPassword.result, pgCluster.metadata.name])
        .apply(([password, host]) =>
            `postgres://authentication:${password}@${host}-rw:5432/${DATABASE_NAME}?sslmode=require`)

    let readonlyUrl = pulumi.all([readonlyPassword.result, pgCluster.metadata.name])
        .apply(([password, host]) =>
            `postgres://readonly:${password}@${host}-rw:5432/${DATABASE_NAME}?sslmode=require`)

    let applicationUrl = pulumi.all([applicationPassword.result, pgCluster.metadata.name])
        .apply(([password, host]) =>
            `postgres://application:${password}@${host}-rw:5432/${DATABASE_NAME}?sslmode=require`)


    // Create a database url secret so our app will work.
    new kx.Secret("database-urls", {
        metadata: {
            namespace: applicationNameSpace.metadata.name,
            name: "database-urls"
        },
        stringData: {
            "migrations-url": migrationsUrl,
            "application-url": applicationUrl,
            "authentication-url": authenticationUrl,
            "readonly-url": readonlyUrl,
        }
    })
}
```

Finally extend the `index.ts` and add the following to the end. This will call the database function and also create our deployment.

```typescript
setupDatabase(applicationNameSpace, cloudnativePg)

const applicationPods = new kx.PodBuilder({
    containers: [{
        name: "application",
        image: `ghcr.io/purton-tech/nails-example:latest`,
        imagePullPolicy: 'IfNotPresent',
        ports: { http: 3000 },
        env: [
            {
                name: 'APP_DATABASE_URL', valueFrom: {
                    secretKeyRef: {
                        name: 'database-urls',
                        key: 'application-url'
                    }
                }
            },
        ]
    }],
    initContainers: [{
        // This runs the migrations when the pod starts.
        name: "application-migrations",
        image: `ghcr.io/purton-tech/nails-example-migrations:latest`,
        imagePullPolicy: 'IfNotPresent',
        env: [
            {
                name: 'DATABASE_URL', valueFrom: {
                    secretKeyRef: {
                        name: 'database-urls',
                        key: 'migrations-url'
                    }
                }
            },
        ]
    }]
})

new kx.Deployment("application", {
    metadata: {
        name: "application",
        namespace: applicationNameSpace.metadata.name
    },
    spec: applicationPods.asDeploymentSpec({ replicas: 1 }) 
})
```

You also need to uncomment the `//import { setupDatabase } from './database'` from the top of the `index.ts`.

Note. Your images will need to have been created in your Github repo.

Run `pulumi up` to apply our latest configuration.

```
Updating (dev)

View in Browser (Ctrl+O): https://app.pulumi.com/ianpurton/nails-example/dev/updates/3

     Type                                         Name               
     pulumi:pulumi:Stack                          nails-example-dev   
 +   ├─ kubernetes:core/v1:Namespace              rust-on-nails      created
 +   ├─ random:index:RandomPassword               app_password       created
 +   ├─ kubernetes:core/v1:Secret                 database-urls      created
 +   ├─ kubernetes:core/v1:Secret                 app-secret         created
 +   └─ kubernetes:postgresql.cnpg.io/v1:Cluster  nails-db-cluster   created


Resources:
    + 5 created
    3 unchanged

Duration: 10s
```

## Connecting to the database

```sh
kubectl port-forward service/nails-db-cluster-rw 5455:5432 --namespace=rust-on-nails
```

You'll need to get the database password from the `database-urls` secret.

```sh
psql -p 5455 -h 127.0.0.1 -U app app
```