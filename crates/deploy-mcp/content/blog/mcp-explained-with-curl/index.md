## LLM's and Tool Calls

Before we look at MCP we need to understand the concept of tool calls with LLM's. For this I'm using [Groq](https://groq.com/) as they provide a more or less free API which we can access.

Get yourself an API key from Groq or any provider and store it in an env VAR.

```sh
export API_KEY=12435......
```

```sh
curl https://api.groq.com/openai/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $API_KEY" \
  -d '{
    "model": "meta-llama/llama-4-maverick-17b-128e-instruct",
    "messages": [
      {"role": "user", "content": "Whats the price of bitcoin?"}
    ]
  }'
```

```
{"id":"chatcmpl-8e59efde-ffe3-4dff-8dc9-a9ac0d790475","object":"chat.completion","created":1759303720,"model":"meta-llama/llama-4-maverick-17b-128e-instruct","choices":[{"index":0,"message":{"role":"assistant","content":"The price of Bitcoin is constantly changing, so I'll give you the current price. According to the current data, the price of 1 Bitcoin is around $63,000. However, please note that cryptocurrency prices are highly volatile, and the price may fluctuate rapidly. For the most up-to-date price, I recommend checking a reliable cryptocurrency exchange or a financial website, such as Coinbase, Binance, or Yahoo Finance."},"logprobs":null,"finish_reason":"stop"}],"usage":{"queue_time":0.175443542,"prompt_tokens":16,"prompt_time":0.000240366,"completion_tokens":84,"completion_time":0.126543503,"total_tokens":100,"total_time":0.126783869},"usage_breakdown":null,"system_fingerprint":"fp_565109a0df","x_groq":{"id":"req_01k6farqj0fvbbs5wes79em332"},"service_tier":"on_demand"}
```

### Adding Tool Call Definitions

```sh
curl https://api.groq.com/openai/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $API_KEY" \
  -d '{
    "model": "meta-llama/llama-4-maverick-17b-128e-instruct",
    "messages": [
      {"role": "user", "content": "Whats the price of bitcoin?"}
    ],
    "tools": [
      {
        "type": "function",
        "function": {
          "name": "get_bitcoin_price",
          "description": "Get the current price of Bitcoin in USD.",
          "parameters": {
            "type": "object",
            "properties": {
              "currency": {
                "type": "string",
                "enum": ["USD", "EUR", "GBP"],
                "description": "The fiat currency to convert the price of Bitcoin into"
              }
            },
            "required": ["currency"]
          }
        }
      }
    ]
  }'
```

```
{"id":"chatcmpl-3cd32d57-0419-4f18-a467-d0ed9127dc3b","object":"chat.completion","created":1759303895,"model":"meta-llama/llama-4-maverick-17b-128e-instruct","choices":[{"index":0,"message":{"role":"assistant","tool_calls":[{"id":"5bqm6jjj7","type":"function","function":{"name":"get_bitcoin_price","arguments":"{\"currency\":\"USD\"}"}}]},"logprobs":null,"finish_reason":"tool_calls"}],"usage":{"queue_time":0.09003323,"prompt_tokens":699,"prompt_time":0.015107538,"completion_tokens":25,"completion_time":0.041136215,"total_tokens":724,"total_time":0.056243753},"usage_breakdown":null,"system_fingerprint":"fp_c527aa4474","x_groq":{"id":"req_01k6fay2gbe1mbvar8nxxm2mt9"},"service_tier":"on_demand"}
```

### Let's run a Bitcoin MCP server

Use deploy to create an MCP server and retrieve your MCP url.

### Getting lists of tools from an MCP server

```sh
curl -v \
  -H "Content-Type: application/json" \
  -H "Accept: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}' \
  http://localhost:7703/v1/mcp/blockchain/cd66871f-af4d-4461-ac60-9d72f0aa2fd1
```

```sh
{"jsonrpc":"2.0","id":1,"result":{"tools":[{"description":"Get Bitcoin prices by currency","inputSchema":{"properties":{},"required":[],"type":"object"},"metadata":{"connectionId":5,"integrationId":9},"name":"getTicker"}]}}/workspace
```

### Calling a tool in an MCP server

```sh
curl -X POST http://localhost:7703/v1/mcp/blockchain/cd66871f-af4d-4461-ac60-9d72f0aa2fd1 \
    -H "Content-Type: application/json" \
    -H "Accept: application/json" \
    -d '{
      "jsonrpc": "2.0",
      "id": "example-session-id",
      "method": "tools/call",
      "params": {
        "name": "getTicker"
      }
    }'
```

```sh
{"jsonrpc":"2.0","id":"example-session-id","result":{"output":{"ARS":{"15m":158108901.58,"buy":158108901.58,"last":158108901.58,"sell":158108901.58,"symbol":"ARS"},"AUD":{"15m":173185.7,"buy":173185.7,"last":173185.7,"sell":173185.7,"symbol":"AUD"},"BRL":{"15m":609674.12,"buy":609674.12,"last":609674.12,"sell":609674.12,"symbol":"BRL"},"CAD":{"15m":159445.11,"buy":159445.11,"last":159445.11,"sell":159445.11,"symbol":"CAD"},"CHF":{"15m":91056.15,"buy":91056.15,"last":91056.15,"sell":91056.15,"symbol":"CHF"},"CLP":{"15m":110201507.32,"buy":110201507.32,"last":110201507.32,"sell":110201507.32,"symbol":"CLP"},"CNY":{"15m":815774.86,"buy":815774.86,"last":815774.86,"sell":815774.86,"symbol":"CNY"},"CZK":{"15m":2367784.85,"buy":2367784.85,"last":2367784.85,"sell":2367784.85,"symbol":"CZK"},"DKK":{"15m":727265.09,"buy":727265.09,"last":727265.09,"sell":727265.09,"symbol":"DKK"},"EUR":{"15m":97422.73,"buy":97422.73,"last":97422.73,"sell":97422.73,"symbol":"EUR"},"GBP":{"15m":85049.36,"buy":85049.36,"last":85049.36,"sell":85049.36,"symbol":"GBP"},"HKD":{"15m":891636.37,"buy":891636.37,"last":891636.37,"sell":891636.37,"symbol":"HKD"},"HRK":{"15m":528533.76,"buy":528533.76,"last":528533.76,"sell":528533.76,"symbol":"HRK"},"HUF":{"15m":37913724.7,"buy":37913724.7,"last":37913724.7,"sell":37913724.7,"symbol":"HUF"},"INR":{"15m":10163745.02,"buy":10163745.02,"last":10163745.02,"sell":10163745.02,"symbol":"INR"},"ISK":{"15m":14979950.46,"buy":14979950.46,"last":14979950.46,"sell":14979950.46,"symbol":"ISK"},"JPY":{"15m":16853999.94,"buy":16853999.94,"last":16853999.94,"sell":16853999.94,"symbol":"JPY"},"KRW":{"15m":160773360.24,"buy":160773360.24,"last":160773360.24,"sell":160773360.24,"symbol":"KRW"},"NGN":{"15m":168824779.28,"buy":168824779.28,"last":168824779.28,"sell":168824779.28,"symbol":"NGN"},"NZD":{"15m":197008.57,"buy":197008.57,"last":197008.57,"sell":197008.57,"symbol":"NZD"},"PLN":{"15m":415172.29,"buy":415172.29,"last":415172.29,"sell":415172.29,"symbol":"PLN"},"RON":{"15m":494999.28,"buy":494999.28,"last":494999.28,"sell":494999.28,"symbol":"RON"},"RUB":{"15m":9366340.94,"buy":9366340.94,"last":9366340.94,"sell":9366340.94,"symbol":"RUB"},"SEK":{"15m":1073886.62,"buy":1073886.62,"last":1073886.62,"sell":1073886.62,"symbol":"SEK"},"SGD":{"15m":147576.13,"buy":147576.13,"last":147576.13,"sell":147576.13,"symbol":"SGD"},"THB":{"15m":3703900.89,"buy":3703900.89,"last":3703900.89,"sell":3703900.89,"symbol":"THB"},"TRY":{"15m":4764322.39,"buy":4764322.39,"last":4764322.39,"sell":4764322.39,"symbol":"TRY"},"TWD":{"15m":3489573.08,"buy":3489573.08,"last":3489573.08,"sell":3489573.08,"symbol":"TWD"},"USD":{"15m":114583.17,"buy":114583.17,"last":114583.17,"sell":114583.17,"symbol":"USD"}}}}
```

### Passing the results back to the model

### That's Agentic AI Folks!