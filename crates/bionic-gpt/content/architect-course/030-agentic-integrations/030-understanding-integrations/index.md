# Connecting LLMs to External Systems

We can use the same tool calling mechanism to connect our model to external systems.

From the sidebar choose `Integrations` and then `Select Integration`.

You'll see an Integration called `Postgres`.

![alt text](select-integration.png)

Click on `Run Integration` to see the Postgres integration up and running.

![alt text](integration-running.png)

## A look at the Postgres Integration

If you look into the `docker-compose.yml` file you see we added the following entry.

```yml
  ...
  postgres-mcp:
    image: ghcr.io/bionic-gpt/bionicgpt-postgres-mcp:1.11.59
  ...
```

The `external system` is running in our lab.

## Configuring our Integration

Click `Configure` and add the following as an API key. We're aware it's not an API key as such but this is how the Postgres integration works.

```
postgresql://postgres:testpassword@postgres:5432/bionic-gpt?sslmode=disable
```

![alt text](add-api-key.png)

## Next...

Now we have configured our integration, lets use it.
