# Installing The Supakube Operator

## Step 1 - Install the CLI

```sh
export SUPAKUBE_VERSION=v1.0.10
curl -OL https://github.com/supakube/supakube/releases/download/${SUPAKUBE_VERSION}/supakube-cli-linux && chmod +x ./supakube-cli-linux && sudo mv ./supakube-cli-linux /usr/local/bin/supakube
```

## Step 2 - Install The Operator

```sh
supakube --install-operator
```

