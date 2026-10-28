use crate::mcp_client::{servers_for_conversation, McpClient};
use crate::types::ToolDefinition;
use crate::{ToolDyn, ToolError};
use db::Pool;
use rig::wasm_compat::WasmBoxedFuture;
use serde::Deserialize;
use serde_json::{json, Value};

pub struct McpAppsTool {
    pool: Pool,
    sub: String,
    conversation_id: i64,
}

impl McpAppsTool {
    pub fn new(pool: Pool, sub: String, conversation_id: i64) -> Self {
        Self {
            pool,
            sub,
            conversation_id,
        }
    }
}

impl ToolDyn for McpAppsTool {
    fn name(&self) -> String {
        "discover_mcp_apps".into()
    }

    fn description(&self) -> String {
        discover_definition().description
    }

    fn parameters(&self) -> Value {
        discover_definition().parameters
    }

    fn call(&self, _args: String) -> WasmBoxedFuture<'_, Result<String, ToolError>> {
        Box::pin(async move {
            let servers = servers_for_conversation(&self.pool, &self.sub, self.conversation_id)
                .await
                .map_err(tool_error)?;
            let mut discovered = Vec::new();
            for server in servers {
                let mut client = McpClient::new(&server.server_url, server.bearer_token.clone())
                    .await
                    .map_err(tool_error)?;
                let tools = client.list_tools().await.map_err(tool_error)?;
                discovered.push(json!({
                    "server_id": server.id,
                    "server": server.name,
                    "tools": tools.into_iter().map(|tool| json!({
                        "name": tool.name,
                        "description": tool.description,
                        "inputSchema": tool.input_schema,
                        "_meta": tool.meta,
                    })).collect::<Vec<_>>()
                }));
            }
            serde_json::to_string(&json!({"servers": discovered})).map_err(ToolError::JsonError)
        })
    }
}

pub struct CallMcpAppTool {
    pool: Pool,
    sub: String,
    conversation_id: i64,
}

impl CallMcpAppTool {
    pub fn new(pool: Pool, sub: String, conversation_id: i64) -> Self {
        Self {
            pool,
            sub,
            conversation_id,
        }
    }
}

#[derive(Deserialize)]
struct CallArgs {
    server_id: i32,
    tool_name: String,
    #[serde(default)]
    arguments: Value,
}

impl ToolDyn for CallMcpAppTool {
    fn name(&self) -> String {
        "call_mcp_app".into()
    }

    fn description(&self) -> String {
        call_definition().description
    }

    fn parameters(&self) -> Value {
        call_definition().parameters
    }

    fn call(&self, args: String) -> WasmBoxedFuture<'_, Result<String, ToolError>> {
        Box::pin(async move {
            let args: CallArgs = serde_json::from_str(&args).map_err(ToolError::JsonError)?;
            let server = servers_for_conversation(&self.pool, &self.sub, self.conversation_id)
                .await
                .map_err(tool_error)?
                .into_iter()
                .find(|server| server.id == args.server_id)
                .ok_or_else(|| tool_error("MCP server is not connected to this conversation"))?;
            let mut client = McpClient::new(&server.server_url, server.bearer_token.clone())
                .await
                .map_err(tool_error)?;
            let tool = client
                .list_tools()
                .await
                .map_err(tool_error)?
                .into_iter()
                .find(|tool| tool.name == args.tool_name)
                .ok_or_else(|| tool_error("MCP tool was not found on the selected server"))?;
            let result = client
                .call_tool(&args.tool_name, args.arguments.clone())
                .await
                .map_err(tool_error)?;
            let result_content = result.get("content").cloned().unwrap_or_else(|| json!([]));
            let structured_content = result
                .get("structuredContent")
                .cloned()
                .unwrap_or(Value::Null);
            let resource_uri = tool
                .meta
                .get("ui")
                .and_then(|ui| ui.get("resourceUri"))
                .and_then(Value::as_str);
            let response = if let Some(resource_uri) = resource_uri {
                json!({
                    "_bionic_mcp_app": {
                        "server_id": server.id,
                        "resource_uri": resource_uri,
                        "server_name": server.name,
                        "tool_name": args.tool_name,
                        "tool_input": args.arguments,
                        "tool_result": result
                    },
                    "content": result_content,
                    "structuredContent": structured_content
                })
            } else {
                result
            };
            serde_json::to_string(&response).map_err(ToolError::JsonError)
        })
    }
}

pub fn discover_definition() -> ToolDefinition {
    ToolDefinition {
        name: "discover_mcp_apps".into(),
        description: "List tools exposed by the MCP apps connected to this conversation. Call this before using an MCP app to get current tool names and input schemas.".into(),
        parameters: json!({"type":"object","properties":{},"additionalProperties":false}),
    }
}

pub fn call_definition() -> ToolDefinition {
    ToolDefinition {
        name: "call_mcp_app".into(),
        description: "Call a tool on a connected MCP app. Use the server_id, tool_name, and input schema returned by discover_mcp_apps.".into(),
        parameters: json!({
            "type":"object",
            "properties":{
                "server_id":{"type":"integer"},
                "tool_name":{"type":"string"},
                "arguments":{"type":"object"}
            },
            "required":["server_id","tool_name","arguments"],
            "additionalProperties":false
        }),
    }
}

fn tool_error(error: impl ToString) -> ToolError {
    ToolError::ToolCallError(Box::new(std::io::Error::other(error.to_string())))
}
