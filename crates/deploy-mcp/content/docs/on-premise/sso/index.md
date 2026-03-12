# Connect Deploy MCP to Your Identity Provider

Deploy MCP relies on your existing enterprise identity platform—Okta, Azure AD, Ping, Google Workspace, or any other OIDC-compliant IdP. There is no embedded identity stack to maintain; instead, you supply the client credentials and issuer metadata so authentication stays under your control.

# Creating an OIDC secret

Store the credentials from your IdP in a Kubernetes secret:

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: oidc-credentials
type: Opaque
data:
  client_id: <base64_encoded_client_id>
  client_secret: <base64_encoded_client_secret>
  issuer_url: <base64_encoded_issuer_url>

```


## Integrating Single Sign On

Reference the secret from the Deploy MCP custom resource so the operator wires the configuration into every MCP service:


```yaml
apiVersion: deploy-mcp.com/v1
kind: Deploy MCP
metadata:
  name: deploy-mcp
  namespace: deploy-mcp 
spec:

  ...
  
  # Single Sign ON
  sso-secret: oidc-credentials

  ...

```
