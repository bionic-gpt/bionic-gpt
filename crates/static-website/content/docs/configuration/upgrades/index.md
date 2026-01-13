Before doing an upgrade it's a good idea to check if you have created database backups and can restore a Bionic installation if anything goes wrong.

Assuming that all works the upgrade procedure is actually similar to the installation procedure.

## Install the Bionic CLI for the version you want to upgrade to

```sh
export BIONIC_VERSION={{ version() }}
curl -OL https://github.com/bionic-gpt/bionic-gpt/releases/download/${BIONIC_VERSION}/bionic-cli-linux && chmod +x ./bionic-cli-linux && sudo mv ./bionic-cli-linux /usr/local/bin/bionic
```

Try it out

```sh
bionic -V
```

## Run the install

**Important** You need to run the same command for an upgrade that you did for an install including all command line parameters.

i.e.

```sh
bionic install --pgadmin etc etc
```

The bionic cli will upgrade the kubernetes operator and all the deployments. After a short time the pods will have restarted and you will be on the latest version.

The bionic upgrade process automatically handles database schema migrations.