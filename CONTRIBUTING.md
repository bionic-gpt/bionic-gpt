# Introduction

This project is based on the [Rust on Nails](https://rust-on-nails.com/) architecture.

## This project depends on k3s

We use k3s to run the services we depend on i.e. unstructured and various models.


```sh
sudo curl -sfL https://get.k3s.io | INSTALL_K3S_EXEC='server --write-kubeconfig-mode="644"' sh -
```

Extract the kubeconfig for use by other K8's tools.

```sh
mkdir -p ~/.kube && cp /etc/rancher/k3s/k3s.yaml ~/.kube/config
```

## Setup for Development

This project uses the [Visual Studio Code Remote - Containers](https://code.visualstudio.com/docs/remote/containers) extension so we can define the runtime and development stack with code. The configuration is in the `.devcontainer`.

Make sure you have Docker Desktop installed and Visual Studio Code Remote. Make sure you have the Remote Containers extension installed. 

After you have run `git clone` on this repository open the folder for the project in Visual Studio Code.

Then click on the green square in the bottom left hand corner of VSCode. (It's the gree square with < and > in the screenshot above). A menu pops down, choose `Remote-Containers: Reopen in Container`

It will take a while for the containers to download.

## Sanity check your dev environment.

Open up the terminal using Use the `View > Terminal` menu command or ``CTRL/CMD ` ``

You can type the following commands in the Linux command prompt.

* rustc --version
* npm -v 
* psql -V

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

## Expose port on k3s

Run the script in a terminal on the host (i.e. not in the devcontainer). This will open up ports so we can access the services from our devcontainer.

```sh
cat <<EOF > open-ports.sh
# Push commands in the background, when the script exits, the commands will exit too
kubectl -n bionic-gpt port-forward --address 0.0.0.0 pod/bionic-db-cluster-1 5432 & \
kubectl -n bionic-gpt port-forward --address 0.0.0.0 deployment/mailhog 8025 & \
kubectl -n bionic-gpt port-forward --address 0.0.0.0 deployment/llm-api 11435:11434 & \

echo "Press CTRL-C to stop port forwarding and exit the script"
wait
EOF
chmod +x ./open-ports.sh
./open-ports.sh
rm ./open-ports.sh
```

## Load and Serve a Model

1. `ollama pull llama2`
1. `ollama run llama2` then `/bye`

Test the above with

```sh
curl http://localhost:11434/v1/chat/completions \
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