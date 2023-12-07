+++
title = "Kubernetes Preparation"
weight =15
sort_by = "weight"
+++

We use [Pulumi](https://www.pulumi.com/) for infrastructure as code. Here we walk through the process of what it would like like to setup a Kubernetes cluster ready for a BionicGPT deployment.

So I usually have a git repository for managing all the infrastructure I need for multiple projects. 

Then for each application I setup a kubernetes namespace and I give the application what it needs via secrets. 

The application then has a `Pulumi.yaml` in it's own github repository which deploys what it needs and pulls in the secrets.

I have tried to reproduce that structure in the one github repository.

## infra-as-code

All the example code is located [here](https://github.com/bionic-gpt/bionic-gpt/tree/main/infra-as-code) and the folder structure looks like the one below.



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

## Running the Installation

OK. Let's run `pulumi up` and see what we get.

```sh
$ pulumi up
Previewing update (dev)

View in Browser (Ctrl+O): https://app.pulumi.com/ianpurton/bionic-gpt/dev/previews/ff5575cc-0623-4687-a4e2-eeaeefa84048

     Type                                         Name                     Plan       
 +   pulumi:pulumi:Stack                          bionic-gpt-dev           create     
 +   ├─ random:index:RandomPassword               readonly_password        create     
 +   ├─ random:index:RandomPassword               migration_password       create     
 +   ├─ kubernetes:core/v1:Namespace              bionic-gpt               create     
 +   ├─ random:index:RandomPassword               authentication_password  create     
 +   ├─ kubernetes:core/v1:Namespace              cloud-native-pg          create     
 +   ├─ random:index:RandomPassword               application_password     create     
 +   ├─ kubernetes:helm.sh/v3:Release             cloudnative-pg           create     
 +   ├─ kubernetes:core/v1:Secret                 migrations-secret        create     
 +   ├─ kubernetes:postgresql.cnpg.io/v1:Cluster  bionic-gpt-db-cluster    create     
 +   └─ kubernetes:core/v1:Secret                 database-urls            create     

Resources:
    + 11 to create

Do you want to perform this update? yes
Updating (dev)

View in Browser (Ctrl+O): https://app.pulumi.com/ianpurton/bionic-gpt/dev/updates/1

     Type                                         Name                     Status              
 +   pulumi:pulumi:Stack                          bionic-gpt-dev           created (1s)        
 +   ├─ random:index:RandomPassword               authentication_password  created (0.76s)     
 +   ├─ random:index:RandomPassword               readonly_password        created (1s)        
 +   ├─ kubernetes:core/v1:Namespace              bionic-gpt               created (1s)        
 +   ├─ random:index:RandomPassword               migration_password       created (2s)        
 +   ├─ random:index:RandomPassword               application_password     created (2s)        
 +   ├─ kubernetes:core/v1:Namespace              cloud-native-pg          created (3s)        
 +   ├─ kubernetes:core/v1:Secret                 migrations-secret        created (1s)        
 +   ├─ kubernetes:helm.sh/v3:Release             cloudnative-pg           created (15s)       
 +   ├─ kubernetes:postgresql.cnpg.io/v1:Cluster  bionic-gpt-db-cluster    created (0.79s)     
 +   └─ kubernetes:core/v1:Secret                 database-urls            created (0.74s)     

Resources:
    + 11 created

Duration: 30s
```

## Getting familiar with k9s

[k9s](https://k9scli.io/) is a terminal based UI to interact with your Kubernetes clusters. Fire it up.

```sh
k9s
```

It looks something like the image below and gives you the ability to see running pods and view the logs.

![Adding secrets to bionic](../k9s.jpeg)

## Connecting to the database

We create a Kubernetes Secret with all our database users and passwords. You can check it out with.

```sh
kubectl get secret database-urls -o jsonpath='{.data.application-url}' --namespace bionic-gpt | base64 --decode

postgres://application:gpBrTQNNyQOY1plcW5Yj@bionic-gpt-db-cluster-rw:5432/app?sslmode=require
```

Let's forward a port to the database

```sh
kubectl port-forward service/bionic-gpt-db-cluster-rw 5455:5432 --namespace=bionic-gpt
```

And now we can connect using details from the postgres URL above.

```sh
psql postgres://application:gpBrTQNNyQOY1plcW5Yj@localhost:5455/app?sslmode=require
```

```sh
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
        LANGUAGE = (unset),
        LC_ALL = (unset),
        LANG = "en_US.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
psql (15.3 (Debian 15.3-0+deb12u1), server 16.0 (Debian 16.0-1.pgdg110+1))
WARNING: psql major version 15, server major version 16.
         Some psql features might not work.
SSL connection (protocol: TLSv1.3, cipher: TLS_AES_256_GCM_SHA384, compression: off)
Type "help" for help.

app=> 
```