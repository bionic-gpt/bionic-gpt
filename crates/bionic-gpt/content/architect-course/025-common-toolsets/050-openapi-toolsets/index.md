# OpenAPI Toolsets

```js
# Register an OpenAPI spec as a toolset
openapi_register(spec_url: string, name: string): string

# List available operations from registered specs
openapi_list_tools(toolset: string): object[]

# Describe a specific operation and params
openapi_describe(operation_id: string): string

# Execute an OpenAPI-backed tool call
openapi_call(operation_id: string, params: object): object
```
