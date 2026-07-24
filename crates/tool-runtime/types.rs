use serde::{Deserialize, Serialize};

pub type ToolDefinition = rig::completion::ToolDefinition;

pub type Reasoning = rig::message::Reasoning;
pub type ToolCallFunction = rig::message::ToolFunction;
pub type ToolCall = rig::message::ToolCall;
pub type ToolResult = rig::message::ToolResult;
pub type ToolResultContent = rig::message::ToolResultContent;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StoredAssistantToolState {
    #[serde(default)]
    pub reasoning: Vec<Reasoning>,
    #[serde(default)]
    pub tool_calls: Vec<ToolCall>,
}

pub fn parse_tool_calls(tool_calls_json: Option<&str>) -> Vec<ToolCall> {
    parse_assistant_tool_state(tool_calls_json).tool_calls
}

pub fn parse_reasoning(tool_calls_json: Option<&str>) -> Vec<Reasoning> {
    parse_assistant_tool_state(tool_calls_json).reasoning
}

pub fn serialize_assistant_tool_state(
    tool_calls: Option<&[ToolCall]>,
    reasoning: Option<&[Reasoning]>,
) -> Option<String> {
    let tool_calls = tool_calls.unwrap_or_default();
    let reasoning = reasoning.unwrap_or_default();

    if tool_calls.is_empty() && reasoning.is_empty() {
        return None;
    }

    if reasoning.is_empty() {
        return serde_json::to_string(tool_calls).ok();
    }

    serde_json::to_string(&StoredAssistantToolState {
        reasoning: reasoning.to_vec(),
        tool_calls: tool_calls.to_vec(),
    })
    .ok()
}

fn parse_assistant_tool_state(tool_calls_json: Option<&str>) -> StoredAssistantToolState {
    let Some(s) = tool_calls_json else {
        return StoredAssistantToolState::default();
    };

    if let Ok(tool_calls) = serde_json::from_str::<Vec<ToolCall>>(s) {
        return StoredAssistantToolState {
            reasoning: Vec::new(),
            tool_calls,
        };
    }

    serde_json::from_str::<StoredAssistantToolState>(s).unwrap_or_default()
}
