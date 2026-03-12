# Memory

Memory tools let an agent store and retrieve user context across turns and sessions.

```js
# Search memory by semantic query
memory_search(query: string, limit: number): object[]

# List memory entries for an agent/session
memory_list(scope: string, limit: number): object[]

# Store a new memory item
memory_write(scope: string, content: string, metadata: object): string

# Remove a memory item
memory_delete(memory_id: string): string
```

## Example: Save a User Preference

### User prompt

```text
Remember it's my birthday on Jan 20th.
```

### 1) Model requests a memory write

```json
{
  "role": "assistant",
  "content": "",
  "tool_calls": [
    {
      "id": "call_memory_write_01",
      "name": "memory_write",
      "arguments": "{\"scope\":\"user:42\",\"content\":\"User birthday is Jan 20\",\"metadata\":{\"type\":\"profile\",\"field\":\"birthday\",\"confidence\":\"user_stated\"}}"
    }
  ]
}
```

### 2) Dispatcher executes and returns tool result

```json
{
  "role": "tool",
  "name": "memory_write",
  "tool_call_id": "call_memory_write_01",
  "content": "{\"memory_id\":\"mem_01JABCDXYZ\",\"status\":\"stored\"}"
}
```

### 3) Model confirms and asks follow-up questions

```json
{
  "role": "assistant",
  "content": "Got it, I will remember your birthday is Jan 20. Do you want me to store the year too? Also, should I remind you a week before each year?"
}
```

## API Call with Full History

Use the full message history when calling the model after tool execution.

```sh
curl http://localhost:11434/api/chat \
  -H "Content-Type: application/json" \
  -d '{
    "model": "granite4:tiny-h",
    "stream": false,
    "tools": [
      {
        "type": "function",
        "function": {
          "name": "memory_write",
          "description": "Store a memory item for a user/session scope.",
          "parameters": {
            "type": "object",
            "properties": {
              "scope": { "type": "string" },
              "content": { "type": "string" },
              "metadata": {
                "type": "object",
                "additionalProperties": true
              }
            },
            "required": ["scope", "content"]
          }
        }
      }
    ],
    "messages": [
      {
        "role": "system",
        "content": "You are a careful assistant. Store durable user preferences with tools and ask clarifying questions when fields are ambiguous."
      },
      {
        "role": "user",
        "content": "Remember it'\''s my birthday on Jan 20th."
      },
      {
        "role": "assistant",
        "content": "",
        "tool_calls": [
          {
            "id": "call_memory_write_01",
            "name": "memory_write",
            "arguments": "{\"scope\":\"user:42\",\"content\":\"User birthday is Jan 20\",\"metadata\":{\"type\":\"profile\",\"field\":\"birthday\",\"confidence\":\"user_stated\"}}"
          }
        ]
      },
      {
        "role": "tool",
        "name": "memory_write",
        "tool_call_id": "call_memory_write_01",
        "content": "{\"memory_id\":\"mem_01JABCDXYZ\",\"status\":\"stored\"}"
      }
    ]
  }'
```

And the result...

```
Your birthday reminder has been successfully stored! I've saved that it's your birthday on **January 20th**. Let me know if there are any other preferences or details you'd like to add.
```

## Questions to Ask Before Saving Memory

1. Is this personal data the user actually wants stored?
2. Is the value complete (for example, date without year)?
3. Should this memory expire or stay permanent?
4. Should the assistant create reminder behavior from this memory?

## Implementations

- [OpenClaw Memory](https://docs.openclaw.ai/concepts/memory)
- [Mem0 (Startup)](https://mem0.ai/)
- [IronClaw](https://github.com/nearai/ironclaw/blob/main/src/tools/builtin/memory.rs) Source Code in Rust.
