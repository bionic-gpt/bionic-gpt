# agent-runtime

This crate implements the server-side agent orchestration runtime for Large Language Model (LLM)
requests in the Bionic application. It handles two main use cases:

- UI-driven chat and synthesis calls from the web app.

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

### 2) Text-to-speech
Route: `/app/synthesize` (POST)

- Resolves the TextToSpeech model and forwards the request body.

## How it works (high level)

- Axum handlers live in this crate and are wired in `lib.rs`.
- Database access uses the generated queries from the `db` crate.
- Prompt building happens in `context_builder.rs` with token-aware trimming.
- Chat history is converted to internal chat message structures in
  `chat_converter.rs`, then mapped into `rig` messages in
  `ui_chat_orchestrator.rs`.
- Streaming responses use `rig` streaming primitives and are mapped to UI SSE
  events.
- Tool execution uses the `tool-runtime` crate to call external tools and store
  results back into the conversation.
- Limits are enforced via `limits.rs` based on model TPM usage.
- Moderation (for guarded models) happens in `moderation.rs` by calling a
  configured guard model.

## Key modules

- `ui_chat_orchestrator.rs`: UI chat streaming, tools, moderation, persistence.
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

- UI routes rely on upstream auth to provide user identity headers/cookies.
- This crate uses `rig` as the chat streaming pipeline.
