# agent-runtime

This crate implements the server-side agent orchestration runtime for Large Language Model (LLM)
requests in the Bionic application. It handles two main use cases:

- UI-driven chat and synthesis calls from the web app.
- API-style requests that mimic the OpenAI-compatible endpoints.

It adds application-specific behavior (auth, context, tools, moderation, limits,
logging) around those requests before forwarding them to the configured model
providers.

## What it does

### 1) UI chat streaming (server -> browser)
Routes: `/completions/{chat_id}` (GET/POST)

- Loads conversation state, prompt config, model, and history from the DB.
- Builds the final message list (prompt + truncated history).
- Optionally attaches tool definitions based on model capabilities and user
  selection.
- Optionally moderates the request through a guard model.
- Streams the model response to the browser as SSE.
- Persists assistant output, tool call outputs, and token usage metrics.

### 2) API chat streaming (external client -> model)
Route: `/v1/chat/completions` (GET/POST)

- Validates API key and loads its associated prompt/model.
- Executes prompt templating with the request messages.
- Logs prompt and completion token usage.
- Proxies the request to the model, optionally streaming results to the client.

### 3) Reverse proxy for non-chat API calls
Route: `/v1/{*path}`

- Validates API key and looks up prompt/model.
- Forwards the request to the provider base URL for non-chat endpoints.

### 4) Text-to-speech
Route: `/app/synthesize` (POST)

- Resolves the TextToSpeech model and forwards the request body.

## How it works (high level)

- Axum handlers live in this crate and are wired in `lib.rs`.
- Database access uses the generated queries from the `db` crate.
- Prompt building happens in `context_builder.rs` with token-aware trimming.
- Chat history is converted to OpenAI-compatible message structures in
  `chat_converter.rs`.
- Streaming responses are processed in `stream_assembler.rs` to merge deltas
  and build a snapshot.
- Tool execution uses the `tool-runtime` crate to call external tools and store
  results back into the conversation.
- Limits are enforced via `limits.rs` based on model TPM usage.
- Moderation (for guarded models) happens in `moderation.rs` by calling a
  configured guard model.

## Key modules

- `api_chat_orchestrator.rs`: OpenAI-compatible `/v1/chat/completions` handler.
- `provider_passthrough.rs`: Generic `/v1/*` proxy for other endpoints.
- `ui_chat_orchestrator.rs`: UI chat streaming, tools, moderation, persistence.
- `stream_assembler.rs`: SSE stream processing and delta merging.
- `stream_errors.rs`: Streaming error helpers for UI chat.
- `context_builder.rs`: Prompt assembly and history truncation.
- `limits.rs`: Rate/usage enforcement logic.
- `moderation.rs`: Guard model moderation for chats.
- `jwt.rs`: User identity extraction for UI requests.
- `user_config.rs`: Cookie-backed user config for chat behavior.

## Data flow details

### UI chat streaming
1. Fetch model, prompt, conversation, and chat history.
2. Build prompt messages and token metrics.
3. Add tool definitions (system tools + integrations + attachments).
4. Optionally run moderation and abort on unsafe input.
5. Stream model output (SSE) and save results and tool outputs.

### API chat streaming
1. Validate API key and resolve model + prompt.
2. Execute prompt templating with input messages.
3. Save the initial chat record and prompt usage metrics.
4. Proxy request to model and stream or return full response.
5. Save completion metrics on stream end.

## Running tests

From the workspace root:

```
just test
```

Or run only this crate:

```
cargo test -p agent-runtime
```

## Notes and assumptions

- API key authentication is required for `/v1/*` routes.
- UI routes rely on upstream auth to provide user identity headers/cookies.
- This crate assumes models are OpenAI-compatible for chat and SSE streams.
