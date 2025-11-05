## Summary

1. The Problem
1. Old RAG
1. Tool Calls
1. Agentic RAG

## Legacy RAG


#### 1) Embed the user's question

```sh
# 1) Get an embedding for the user prompt (only store its ID/handle)
# - Never print the full vector in logs or UI
# - Use any small embedding model you like
curl https://api.openai.com/v1/embeddings \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "text-embedding-3-small",
    "input": "How do I replace the rotor blades on the Falcon X2 drone?"
  }' \
| jq -r '.data[0].embedding' > /tmp/embedding.json
# Persist as a parameter only (not shown to readers)

```

#### 2) Retrieve top-k relevant chunks with pgvector

```sql
-- 2) Vector search (pgvector)
-- - $1 is the embedding parameter (binary/JSON array bound by your client)
-- - <-> is the distance operator; choose ops class per metric (cosine/L2/IP)
-- - LIMIT k keeps the context small
SELECT
  id,
  text
FROM chunks
ORDER BY embedding <-> $1
LIMIT 5;

```

#### 3) Ask the model again, now with retrieved context

```sh
# 3) Compose a grounded prompt (system or user content) with the retrieved chunks
# - Keep a short, explicit instruction to "cite from context only"
# - Keep history light to avoid runaway token growth
curl https://api.openai.com/v1/chat/completions \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "gpt-4.1",
    "messages": [
      {"role":"system","content":"Answer using the provided context only. If unsure, say so."},
      {"role":"user","content":"How do I replace the rotor blades on the Falcon X2 drone?"},
      {"role":"user","content":"CONTEXT:\n- <chunk 1 text>\n- <chunk 2 text>\n- <chunk 3 text>\n- <chunk 4 text>\n- <chunk 5 text>"}
    ]
  }'

```

### Problems

1. **Fixed retrieval** → Same `k` every time → either too much or too little context.
2. **Model has no control** → It can’t decide when or how to search.
3. **One-shot search** → If retrieval misses, answer is wrong.
4. **Chat history bloat** → Context gets bigger and messier each turn.
5. **Manual tuning required** → Humans keep tweaking chunk size / k / prompts.


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

