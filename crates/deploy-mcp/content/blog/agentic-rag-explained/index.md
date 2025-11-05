## Summary

1. The Problem
1. Old RAG
1. Tool Calls
1. Agentic RAG

## Old RAG

```sh
curl https://api.openai.com/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -d '{
    "model": "gpt-4.1",
    "messages": [
      {"role": "user", "content": "How do I replace the rotor blades on the Falcon X2 drone?"}
    ]
  }'
```

```sql
SELECT * FROM chunks WHERE 
```

```sh
curl https://api.openai.com/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -d '{
    "model": "gpt-4.1",
    "messages": [
      {"role": "user", "content": "How do I replace the rotor blades on the Falcon X2 drone?"}
    ]
  }'
```

### Problems

- Do we do a lookup for each user question
- Do we add the results to the history.
- How much data to provide.

## Tool Calls

```sh
curl https://api.openai.com/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -d '{
    "model": "gpt-5",
    "messages": [
      {"role": "user", "content": "How do I replace the rotor blades on the Falcon X2 drone?"}
    ],
    "tools": [
      {
        "type": "function",
        "function": {
          "name": "lookup_drone_manual_section",
          "description": "Look up information from the drone maintenance manual based on a query.",
          "parameters": {
            "type": "object",
            "properties": {
              "query": {
                "type": "string",
                "description": "User question or topic to search for in the drone manual."
              }
            },
            "required": ["query"]
          }
        }
      }
    ],
    "tool_choice": "auto"
  }'
```

### Response

```json
{
  "id": "chatcmpl-87b9",
  "object": "chat.completion",
  "choices": [
    {
      "index": 0,
      "message": {
        "role": "assistant",
        "tool_call": {
          "id": "call_01F6",
          "type": "function",
          "function": {
            "name": "lookup_drone_manual_section",
            "arguments": "{\"query\":\"replace rotor blades on Falcon X2\"}"
          }
        }
      },
      "finish_reason": "tool_calls"
    }
  ]
}
```

## Agentic RAG

```sql
SELECT * FROM chunks WHERE 
```

## Real World Examples

Drone customer support email.
Compliance

