pub type ToolDefinition = rig::completion::ToolDefinition;

pub type ToolCallFunction = rig::message::ToolFunction;
pub type ToolCall = rig::message::ToolCall;
pub type ToolResult = rig::message::ToolResult;
pub type ToolResultContent = rig::message::ToolResultContent;

pub fn parse_tool_calls(tool_calls_json: Option<&str>) -> Vec<ToolCall> {
    tool_calls_json
        .and_then(|s| serde_json::from_str::<Vec<ToolCall>>(s).ok())
        .unwrap_or_default()
}
