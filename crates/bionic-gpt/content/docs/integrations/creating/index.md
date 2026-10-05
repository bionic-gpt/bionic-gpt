# Creating an OpenAPI Integration

If your system has an OpenAPI API, you can connect it to Bionic. Bionic reads the operations in the specification and exposes them as tools that a model can call.

## Prepare the specification

Use an OpenAPI 3.x document in JSON, YAML, or YML format. Give the API a useful `info.title` and `info.description`; Bionic uses the description to help the model decide when and how to use the integration. Each operation needs a unique `operationId` and a clear `summary` or `description`.

Set a server URL and describe authentication when the API requires it. Bionic supports API keys and OAuth2 schemes declared in the OpenAPI document. Keep operation descriptions practical and specific, and include only endpoints the assistant should be able to call.

```yaml
openapi: 3.0.3
info:
  title: Example Customer API
  version: 1.0.0
  description: Search customers and retrieve a customer record.
  x-bionic-slug: example-customers
  x-logo:
    url: https://example.com/logo.svg
servers:
  - url: https://api.example.com
paths:
  /customers:
    get:
      operationId: searchCustomers
      summary: Search customers
      description: Find customers by name or email address.
      parameters:
        - name: query
          in: query
          required: true
          schema:
            type: string
      responses:
        '200':
          description: Matching customers
```

## Add it to Bionic

1. In the admin area, open **OpenAPI Specs**.
2. Choose **New** to paste a specification, or use **Upload OpenAPI Specs** to upload one JSON/YAML file or a ZIP of specifications.
3. Save the spec, then open **Integrations** and add the integration to the team.
4. Configure the required credentials and choose which users or assistants can use it.

The imported spec must have a valid `info.title`; Bionic uses `info.x-bionic-slug` when supplied, otherwise it derives the slug from the title. The `info.x-logo.url` value is used as the integration icon. Review the generated tools and permissions before making an integration available to users.

## Validate and maintain

Validate the document as OpenAPI 3.x before uploading. Confirm that every operation has an `operationId`, request parameters and JSON bodies are described accurately, and the server URL is correct for your environment. Keep the source specification under version control and update it when the API changes.
