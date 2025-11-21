# Custom Integrations (OpenAPI)

We use OpenAPI specifications to let admins connect internal systems to assistants. If your platform exposes an HTTP API, you can describe it once and instantly turn it into a safe, typed tool your LLM can call.

## Sample OpenAPI Snippet

```yaml
openapi: 3.0.1
info:
  title: Support Desk API
  version: "1.0"
paths:
  /support/cases:
    post:
      summary: Create support case
      operationId: createSupportCase
      security:
        - bearerAuth: []
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: "#/components/schemas/SupportCaseRequest"
      responses:
        "200":
          description: Case created
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/SupportCaseResponse"
components:
  securitySchemes:
    bearerAuth:
      type: http
      scheme: bearer
      bearerFormat: JWT
  schemas:
    SupportCaseRequest:
      type: object
      required: [customer_id, summary, priority]
      properties:
        customer_id: { type: string }
        summary: { type: string }
        priority: { type: string, enum: [low, medium, high] }
    SupportCaseResponse:
      type: object
      properties:
        case_id: { type: string }
        status: { type: string }
```

## Tool Definition Outline

When the platform ingests the spec above it emits a tool definition similar to:

```json
{
  "name": "create_support_case",
  "description": "Create a support ticket for the current customer in the Support Desk API.",
  "parameters": {
    "type": "object",
    "properties": {
      "customer_id": { "type": "string" },
      "summary": { "type": "string" },
      "priority": { "type": "string", "enum": ["low", "medium", "high"] }
    },
    "required": ["customer_id", "summary", "priority"]
  }
}
```

This definition is delivered to the LLM alongside the user’s request, so the model knows exactly which inputs are allowed.

## Tool Call Workflow

1. **Admin uploads spec**: The OpenAPI document is stored, validated, and linked to an assistant with the required authentication (for example, a Bearer token stored in Secrets Manager).
2. **Model plans**: When a user asks for something like “Create a high-priority ticket for ACME,” the LLM inspects available tools, chooses `create_support_case`, and fills out the JSON arguments.
3. **Runtime executes**: The platform injects the Bearer token into the HTTP `Authorization` header, sends the request to `/support/cases`, and enforces timeouts/retries.
4. **Response grounded**: The API response (`case_id`, `status`) is returned to the model, which cites the real IDs in its follow-up message to the user.

### Authentication Notes

- **Bearer tokens** are the default: store them as secrets bound to the integration so agents never see raw credentials.
- **Additional schemes** (mTLS, API keys, OAuth) can also be modelled in the OpenAPI `securitySchemes` block; the runtime handles the handshake and injects headers automatically.
- **Per-tenant isolation** ensures each assistant only calls endpoints paired with its namespace and credentials.

With this pattern, any internal REST service can be surfaced as a deterministic tool. That keeps enterprise data in your environment while giving assistants controlled access to the systems that matter.
