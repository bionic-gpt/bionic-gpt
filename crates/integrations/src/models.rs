//! Data models for integrations.

use serde::{Deserialize, Serialize};

/// Definition of a function that can be called by an LLM.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionDefinition {
    /// The name of the function.
    pub name: String,
    /// A description of what the function does.
    pub description: String,
    /// The parameters of the function, as a JSON Schema object.
    pub parameters: serde_json::Value,
}

/// A tool that can be used by an LLM.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tool {
    /// The type of the tool, currently only "function" is supported.
    pub r#type: String,
    /// The function definition for this tool.
    pub function: FunctionDefinition,
}

/// Function information for a tool call.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCallFunction {
    /// The name of the function to call.
    pub name: String,
    /// The arguments to pass to the function, as a JSON string.
    pub arguments: String,
}

/// A call to a tool by an LLM.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCall {
    /// The ID of the tool call.
    pub id: String,
    /// The type of the tool call, currently only "function" is supported.
    pub r#type: String,
    /// The function to call.
    pub function: ToolCallFunction,
}

/// The result of a tool call.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolResult {
    /// The ID of the tool call that this result is for.
    pub tool_call_id: String,
    /// The role of the message, should be "tool".
    pub role: String,
    /// The name of the function that was called.
    pub name: String,
    /// The result of the function call, as a string.
    pub content: String,
}

/// A message in a conversation.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    /// The role of the message (e.g., "user", "assistant", "tool").
    pub role: String,
    /// The content of the message.
    pub content: String,
    /// The ID of the tool call that this message is responding to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    /// The tool calls made by this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    /// The name of the function that was called.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// A completion request to an LLM.
#[derive(Serialize, Deserialize, Debug)]
pub struct Completion {
    /// The model to use for the completion.
    pub model: String,
    /// Whether to stream the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// The maximum number of tokens to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    /// The messages in the conversation.
    pub messages: Vec<Message>,
    /// The temperature to use for sampling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// The tools available to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    /// The tool choice for the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<serde_json::Value>,
}
