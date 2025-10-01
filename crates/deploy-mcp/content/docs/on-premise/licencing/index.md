# Licence

Store your licence in a file called `licence.yaml`

# Apply the Deploy MCP Licence

```sh
kubectl -n deploy-mcp apply -f licence.yaml

kubectl rollout restart deployment deploy-mcp -n deploy-mcp
```


## Getting a Key

Please see the [Contact Us](https://deploy-mcp.com/contact/) page and set up a call so we can properly asses your requirements.
