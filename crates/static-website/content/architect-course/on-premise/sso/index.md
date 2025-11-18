# Creating an OIDC secret

We need the credentials form your SSO provider stored in a Kubernetes Secret i.e.

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


```yaml
apiVersion: bionic-gpt.com/v1
kind: Bionic
metadata:
  name: bionic-gpt
  namespace: bionic-gpt 
spec:

  ...
  
  # Single Sign ON
  sso-secret: oidc-credentials

  ...

```