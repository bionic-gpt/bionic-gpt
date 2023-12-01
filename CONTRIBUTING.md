# Introduction

This project is based on the [Rust on Nails](https://rust-on-nails.com/) architecture.

## Setup for Development

This project uses the [Visual Studio Code Remote - Containers](https://code.visualstudio.com/docs/remote/containers) extension so we can define the runtime and development stack with code. The configuration is in the `.devcontainer` folder and uses a `docker-compose` configuration to setup the tools and database ready for development.

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

## Update any of the git submodules

The website uses a zola theme. This will need to be loaded with

`git submodule init`

`gsu`

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