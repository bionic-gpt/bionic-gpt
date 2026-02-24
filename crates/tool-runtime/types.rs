use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct ToolDefinition {
    pub r#type: String,
    pub function: ToolFunctionDefinition,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct ToolFunctionDefinition {
    pub name: String,
    pub description: String,
    pub parameters: Value,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq, Default)]
pub struct ToolCallFunction {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub arguments: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq, Default)]
pub struct ToolCall {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<u32>,
    #[serde(default = "default_tool_call_type")]
    pub r#type: String,
    #[serde(default)]
    pub function: ToolCallFunction,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct ToolCallResult {
    pub id: String,
    pub result: serde_json::Value,
    pub name: String,
}

fn default_tool_call_type() -> String {
    "function".to_string()
}
