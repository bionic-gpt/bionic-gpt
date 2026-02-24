use serde::{Deserialize, Serialize};

pub type ToolDefinition = rig::completion::ToolDefinition;

pub type ToolCallFunction = rig::message::ToolFunction;
pub type ToolCall = rig::message::ToolCall;

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct ToolCallResult {
    pub id: String,
    pub result: serde_json::Value,
    pub name: String,
}
