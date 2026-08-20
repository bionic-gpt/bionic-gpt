# Understanding Tool Calls

- **Why tool calls exist.** They hand off work to *real systems* when the model can’t know the answer.
- **What you get.** **Accurate**, *traceable* results aligned with systems you already trust.
- **What this page covers.** When to use a **tool** vs. *text-only* response, and how to encode arguments reliably.

We cover the request lifecycle, how structured responses are marshalled in Rust, and what telemetry you can collect to monitor usage.

## Example: Asking “What is the price of Bitcoin?” without Tools

When you omit the `tools` array, the model must answer using its internal context, which usually results in a generic response. The example below uses the same `granite4:tiny-h` model referenced in the Ollama quick start guide:

```sh
curl http://localhost:11434/api/chat \
  -H "Content-Type: application/json" \
  -d '{
        "model": "granite4:tiny-h",
        "stream": false,
        "messages": [
          {
            "role": "system", 
            "content": "You are a precise assistant that admits uncertainty."
          },
          {
            "role": "user", 
            "content": "What is the price of Bitcoin today in USD?"
          }
        ]
      }'
```

Typical output looks like:

```json
{
  "role": "assistant",
  "content": "I’m sorry, but I don’t have access to real‑time data, so I can’t provide today’s Bitcoin price. For the most up‑to‑date figure, please check a reliable financial source such as a cryptocurrency exchange (e.g., Coinbase, Binance), a market data site (e.g., CoinMarketCap, CoinGecko), or a financial news outlet. If you’d like, I can share information about how Bitcoin’s price has historically behaved or explain the factors that influence its value. Let me know how I can help!"
}
```

## Example: Asking “What is the price of Bitcoin?” with Tools

The `chat` endpoint in Ollama can emit structured tool invocations when the user request requires code. Re-using the `granite4:tiny-h` model, expose a `get_current_datetime` tool so the assistant can call into deterministic logic:

```sh
curl http://localhost:11434/api/chat \
  -H "Content-Type: application/json" \
  -d '{
    "model": "granite4:tiny-h",
    "stream": false,
    "messages": [
      {"role": "system", "content": "You are a precise assistant that admits uncertainty."},
      {"role": "user", "content": "What is the price of Bitcoin today in USD?"}
    ],
    "tools": [
      {
        "type": "function",
        "function": {
          "name": "get_bitcoin_price_usd",
          "description": "Returns the current price of Bitcoin in USD.",
          "parameters": {
            "type": "object",
            "properties": {},
            "additionalProperties": false
          }
        }
      }
    ],
    "tool_choice": "auto"
  }'
```

Ollama returns either a natural-language answer or a tool payload. When a tool is selected the `message` looks like:

```json
{
  "role": "assistant",
  "content": "",
  "tool_calls": [
    {
      "name": "get_bitcoin_price_usd",
      "arguments": "{}"
    }
  ]
}
```

- **Tool payload = executable intent.** It includes the *tool name* and *JSON arguments* your code should run.
- **Run → respond.** Execute the tool, send the result back, and the assistant returns a *natural-language* answer.
- **Why tools matter.** With a tool, you get the *real Bitcoin price*; without one, you get *apologies or guesses*.
- **Teaching point.** Use this contrast to justify **deterministic code** behind every real-world capability.
