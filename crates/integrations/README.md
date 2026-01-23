# integrations

This crate provides the tool and integration system used by the app and
`llm-proxy`. It exposes OpenAI-style tool definitions, executes tool calls, and
supports both built-in tools and OpenAPI-based external integrations.

## What it does

- Defines a common `ToolInterface` for all tools.
- Registers built-in tools (time, web, datasets, documents, RAG search).
- Loads system-level OpenAPI specs (web search / code sandbox) and converts them
  into tools.
- Loads external integrations connected to prompts and converts their OpenAPI
  specs into tools.
- Executes tool calls and returns JSON results.

## Key modules

- `tool.rs`: `ToolInterface` trait (definition + execute).
- `tool_registry.rs`: catalog of built-in tools and tool scopes.
- `tool_executor.rs`: resolve tool instances and execute tool calls.
- `bionic_openapi.rs`: OpenAPI v3 parsing and tool definition generation.
- `system_openapi.rs`: system-selected OpenAPI specs (per category).
- `tools/`: built-in tool implementations.
- `token_providers.rs`: auth token providers for OpenAPI tools.

## Tool scopes

`ToolScope` controls where tools are exposed:

- `UserSelectable`: tools users can enable in chat.
- `DocumentIntelligence`: tools enabled when a conversation has attachments.
- `Rag`: dataset tools (list/search datasets and files).

Use `get_tools`, `get_tools_with_system_openapi`, or
`get_chat_tools_user_selected_with_system_openapi` depending on the context.

## Built-in tools

- `time_date`: get current time and date.
- `web`: open URL tool.
- `list_documents`, `read_document`: document tools.
- `list_datasets`, `list_dataset_files`, `search_context`: dataset/RAG tools.

## System OpenAPI tools

System OpenAPI tools are loaded from DB-configured specs and used for
site-wide integrations like Web Search or Code Sandbox:

- `get_system_openapi_tool_definitions` returns tool definitions only.
- `get_system_openapi_tools` returns executable tool instances.

## External integrations

Prompt integrations are stored in the DB. The flow is:

1. Load prompt integrations and their connections.
2. Parse OpenAPI v3 spec into tool definitions.
3. Build executable tools (with auth via token providers).
4. Merge with built-ins, resolving name conflicts by overriding built-ins.

## Executing tool calls

`execute_tool_calls` accepts a list of OpenAI-style tool calls and returns
`ToolCallResult` values. It resolves tool instances (built-in, system OpenAPI,
external OpenAPI) and dispatches each call via `ToolInterface::execute`.

## Testing

- `tool_registry.rs` includes basic tests for tool selection.
- `tool_executor.rs` includes a tool execution test for the time/date tool.
- `tools/open_api_tool.rs` supports HTTP client overrides for tests.

