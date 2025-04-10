//! JSON parsing models for OpenAI API responses

use serde::{Deserialize, Serialize};

/// Structs for deserializing the tool call JSON response
#[derive(Debug, Serialize, Deserialize)]
pub struct ToolCallJson {
    pub id: String,
    #[serde(rename = "type")]
    pub call_type: String,
    pub function: ToolCallFunctionJson,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolCallFunctionJson {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeltaJson {
    #[serde(default)]
    pub tool_calls: Vec<ToolCallJson>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChoiceJson {
    pub delta: DeltaJson,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionResponse {
    pub choices: Vec<ChoiceJson>,
}
