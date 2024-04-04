# Introduction

This project is based on the [Rust on Nails](https://rust-on-nails.com/) architecture.

## Setup for Development

This project uses [Devpod](https://devpod.sh/). DevPod is a tool used to create reproducible developer environments. We use K3s to host our development environment as this is also where we do most development.

## Install K3s

```sh
sudo curl -sfL https://get.k3s.io | INSTALL_K3S_EXEC='server --write-kubeconfig-mode="644"' sh -
```

## Install DevPod

Go to https://devpod.sh/docs/getting-started/install and install the CLI.

Then run

```sh
devpod go https://github.com/bionic-gpt/bionic-gpt
```

## Sanity check your dev environment.

Open up the terminal using Use the `View > Terminal` menu command or ``CTRL/CMD ` ``

You can type the following commands in the Linux command prompt.

* rustc --version
* npm -v 
* psql -V

## Connecting to the cluster

Use the following command to inject the k3s.yaml into the containers kube config.

```sh
POD_NAME=$(kubectl -n devpod get pods --field-selector=status.phase=Running -o jsonpath='{.items[*].metadata.name}' | grep 'bionic-gpt' | head -n 1)
kubectl -n devpod exec $POD_NAME -- mkdir -p /home/vscode/.kube
kubectl -n devpod cp /etc/rancher/k3s/k3s.yaml $POD_NAME:/home/vscode/.kube/config
HOST_IP=$(hostname -I | awk '{print $1}')
kubectl -n devpod exec $POD_NAME -- sed -i "s/127.0.0.1/${HOST_IP}/g" /home/vscode/.kube/config
```

## Initialise all the services

```sh
cargo run --bin k8s-operator -- install --development --no-operator
```

Then also run the operator

```sh
cargo run --bin k8s-operator -- operator
```

## Running Database Migrations

We use [dbmate](https://github.com/amacneil/dbmate) to manage database migrations.

```
$ dbmate status
[ ] 20220410155201_initial_setup.sql
[ ] 20220410155211_authentication.sql
[ ] 20220410155233_rbac_and_authorization.sql
[ ] 20220410155252_teams.sql
[ ] 20220728091159_rls_setup.sql
[ ] 20220808093939_auth_and_readonly_policies.sql
[ ] 20220808094314_tenancy_isolation.sql
[ ] 20230801121853_chats.sql
[ ] 20230804140530_documents_and_datasets.sql
[ ] 20230807094835_prompts.sql
[ ] 20230810114756_models.sql

Applied: 0
Pending: 11
```

Create all the database tables with

`dbmate up`

## Update any of the git submodules

The website uses a zola theme. This will need to be loaded with

`git submodule init`

`gsu`

## Load and Serve a Model

1. `ollama pull llama2`
1. `ollama run llama2` then `/bye`

Test the above with

```sh
curl http://llm-api:11434/v1/chat/completions \
    -H "Content-Type: application/json" \
    -d '{
        "model": "llama2",
        "messages": [
            {
                "role": "system",
                "content": "You are a helpful assistant."
            },
            {
                "role": "user",
                "content": "Hello!"
            }
        ]
    }'
```

## Starting the services

We have created a `.bash_alias` file in the `.devcontainer` folder. Open up 3 consoles in visual studio code and run the following in each one.

1. `wp` - Watch Pipeline - compiles the web assets such as typescript, scss and processes images.
1. `wt` - Watch TailwindCSS - Runs tailwinf to create an output.css file.
1. `wa` - Stands for watch application - compiles and runs the axum server and will recompile on file chnages.
1. `wz` - Watch Zola - runs the static site generator.
1. `we` - Watch Embeddings - runs the embeddings job.

## Problems with permissions

If you get an error such as `Permission Denied` on the target folder run the following

`sudo chmod 777 -R target`

## Accessing the web front end.

`https://localhost:7700`