# Apply the Bionic Licence

```sh
kubectl -n bionic-gpt create secret generic bionic-gpt-license \
  --from-literal=LICENCE='{"user_count":15,"end_date":"2026-01-01T00:00:00Z","signature":"MCwCFHCz9kQ4kP3hAgMBAiEAm8=="}'
```

## Add the licence as an env var to the deployment

```sh
kubectl -n bionic-gpt patch deployment bionic-gpt --type=json \
  -p='[{"op":"add","path":"/spec/template/spec/containers/0/env/-","value":{"name":"LICENCE","valueFrom":{"secretKeyRef":{"name":"bionic-gpt-license","key":"LICENCE"}}}}]'
```

## Getting a Key

Please see the [Contact Us](https://bionic-gpt.com/contact/) page and set up a call so we can properly asses your requirements.
