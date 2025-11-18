# Administer the Database with pgAdmin

[pgAdmin](https://www.pgadmin.org/) is installed  by passing the parameter `--pgadmin` to bionic at install time.

## Getting the pgAdmin Logon Password

To get the login credentials

```sh
kubectl get secret -n bionic-gpt pgadmin -o jsonpath='{.data.email}' | base64 --decode
kubectl get secret -n bionic-gpt pgadmin -o jsonpath='{.data.password}' | base64 --decode
```

## Accessing the Database

```sh
kubectl get secret -n bionic-gpt database-urls -o jsonpath='{.data.readonly-url}' | base64 --decode
```