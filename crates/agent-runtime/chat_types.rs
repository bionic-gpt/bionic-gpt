use serde::{Deserialize, Serialize};
use tool_runtime::{ToolCall, ToolDefinition};

#[derive(Serialize, Deserialize, Debug)]
pub struct BionicChatCompletionRequest {
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    pub messages: Vec<ChatCompletionMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<serde_json::Value>,
}

pub type ChatCompletionDelta = ChatCompletionGeneric<ChatCompletionChoiceDelta>;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ChatCompletionGeneric<C> {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<C>,
    pub usage: Option<Usage>,
}

#[derive(Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub struct Usage {
    pub prompt_tokens: Option<u32>,
    pub completion_tokens: Option<u32>,
    pub total_tokens: Option<u32>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ChatCompletionChoiceDelta {
    pub index: u64,
    pub finish_reason: Option<String>,
    pub delta: ChatCompletionMessageDelta,
}

impl ChatCompletionDelta {
    pub fn merge(
        &mut self,
        other: ChatCompletionDelta,
    ) -> Result<(), ChatCompletionDeltaMergeError> {
        for other_choice in other.choices.iter() {
            for choice in self.choices.iter_mut() {
                if choice.index != other_choice.index {
                    continue;
                }
                choice.merge(other_choice)?;
            }
        }
        Ok(())
    }
}

impl ChatCompletionChoiceDelta {
    pub fn merge(
        &mut self,
        other: &ChatCompletionChoiceDelta,
    ) -> Result<(), ChatCompletionDeltaMergeError> {
        if self.index != other.index {
            return Err(ChatCompletionDeltaMergeError::DifferentCompletionChoiceIndices);
        }
        if self.delta.role.is_none() {
            if let Some(other_role) = other.delta.role {
                self.delta.role = Some(other_role);
            }
        }
        if self.delta.name.is_none() {
            if let Some(other_name) = &other.delta.name {
                self.delta.name = Some(other_name.clone());
            }
        }
        match self.delta.content.as_mut() {
            Some(content) => {
                if let Some(other_content) = &other.delta.content {
                    content.push_str(other_content)
                }
            }
            None => {
                if let Some(other_content) = &other.delta.content {
                    self.delta.content = Some(other_content.clone());
                }
            }
        };

        match self.delta.tool_calls.as_mut() {
            Some(tool_calls) => {
                if let Some(other_tool_calls) = &other.delta.tool_calls {
                    merge_tool_calls(tool_calls, other_tool_calls);
                }
            }
            None => {
                if let Some(other_tool_calls) = &other.delta.tool_calls {
                    self.delta.tool_calls = Some(other_tool_calls.clone());
                }
            }
        }

        Ok(())
    }
}

fn merge_tool_calls(target_tool_calls: &mut Vec<ToolCall>, incoming_tool_calls: &[ToolCall]) {
    for incoming in incoming_tool_calls {
        let maybe_target = target_tool_calls.iter_mut().find(|existing| {
            match (&existing.id[..], &incoming.id[..]) {
                ("", "") => existing.index == incoming.index,
                (_, "") => existing.index == incoming.index,
                ("", _) => false,
                _ => existing.id == incoming.id,
            }
        });

        let target = if let Some(existing) = maybe_target {
            existing
        } else {
            target_tool_calls.push(incoming.clone());
            target_tool_calls.last_mut().expect("just pushed")
        };

        if target.id.is_empty() && !incoming.id.is_empty() {
            target.id = incoming.id.clone();
        }
        if target.index.is_none() && incoming.index.is_some() {
            target.index = incoming.index;
        }
        if target.r#type.is_empty() && !incoming.r#type.is_empty() {
            target.r#type = incoming.r#type.clone();
        }
        if target.function.name.is_empty() && !incoming.function.name.is_empty() {
            target.function.name = incoming.function.name.clone();
        }

        if !incoming.function.arguments.is_empty()
            && !target
                .function
                .arguments
                .ends_with(&incoming.function.arguments)
        {
            target
                .function
                .arguments
                .push_str(&incoming.function.arguments);
        }
    }
}

#[derive(Debug)]
pub enum ChatCompletionDeltaMergeError {
    DifferentCompletionIds,
    DifferentCompletionChoiceIndices,
    FunctionCallArgumentTypeMismatch,
}

impl std::fmt::Display for ChatCompletionDeltaMergeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatCompletionDeltaMergeError::DifferentCompletionIds => {
                f.write_str("Different completion IDs")
            }
            ChatCompletionDeltaMergeError::DifferentCompletionChoiceIndices => {
                f.write_str("Different completion choice indices")
            }
            ChatCompletionDeltaMergeError::FunctionCallArgumentTypeMismatch => {
                f.write_str("Function call argument type mismatch")
            }
        }
    }
}

impl std::error::Error for ChatCompletionDeltaMergeError {}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ChatCompletionMessageDelta {
    pub role: Option<ChatCompletionMessageRole>,
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    #[serde(skip_serializing_if = "is_none_or_empty_vec")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Default)]
pub struct ChatCompletionMessage {
    pub role: ChatCompletionMessageRole,
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    #[serde(skip_serializing_if = "is_none_or_empty_vec")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

fn is_none_or_empty_vec<T>(opt: &Option<Vec<T>>) -> bool {
    opt.as_ref().map(|v| v.is_empty()).unwrap_or(true)
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, Eq, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ChatCompletionMessageRole {
    System,
    #[default]
    User,
    Assistant,
    Function,
    Tool,
    Developer,
}
