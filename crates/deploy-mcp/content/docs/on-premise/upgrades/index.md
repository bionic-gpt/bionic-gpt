Before doing an upgrade it's a good idea to check if you have created database backups and can restore a Deploy MCP installation if anything goes wrong.

Assuming that all works the upgrade procedure is actually similar to the installation procedure.

## Install the Deploy MCP CLI for the version you want to upgrade to

```sh
export DEPLOY_MCP_VERSION={{ version() }}
curl -OL https://github.com/deploy-mcp/deploy-mcp/releases/download/${DEPLOY_MCP_VERSION}/deploy-mcp-cli-linux && chmod +x ./deploy-mcp-cli-linux && sudo mv ./deploy-mcp-cli-linux /usr/local/bin/deploy-mcp
```

Try it out

```sh
deploy-mcp -V
```

## Run the install

**Important** You need to run the same command for an upgrade that you did for an install including all command line parameters.

i.e.

```sh
deploy-mcp install --pgadmin etc etc
```

The deploy-mcp cli will upgrade the kubernetes operator and all the deployments. After a short time the pods will have restarted and you will be on the latest version.

The deploy-mcp upgrade process automatically handles database schema migrations.