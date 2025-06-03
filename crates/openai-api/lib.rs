use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    pub tools: Option<Vec<BionicToolDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct BionicToolDefinition {
    pub r#type: String,
    /// The function that the model called.
    pub function: ChatCompletionFunctionDefinition,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct ChatCompletionFunctionDefinition {
    /// The name of the function
    pub name: String,
    /// The description of the function
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The parameters of the function formatted in JSON Schema
    /// [API Reference](https://platform.openai.com/docs/api-reference/chat/create#chat/create-parameters)
    /// [See more information about JSON Schema.](https://json-schema.org/understanding-json-schema/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Value>,
}

/// A delta chat completion, which is streamed token by token.
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
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ChatCompletionChoiceDelta {
    pub index: u64,
    pub finish_reason: Option<String>,
    pub delta: ChatCompletionMessageDelta,
}

impl ChatCompletionDelta {
    /// Merges the input delta completion into `self`.
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
                // Set role to other_role.
                self.delta.role = Some(other_role);
            }
        }
        if self.delta.name.is_none() {
            if let Some(other_name) = &other.delta.name {
                // Set name to other_name.
                self.delta.name = Some(other_name.clone());
            }
        }
        // Merge contents.
        match self.delta.content.as_mut() {
            Some(content) => {
                if let Some(other_content) = &other.delta.content {
                    // Push other content into this one.
                    content.push_str(other_content)
                }
            }
            None => {
                if let Some(other_content) = &other.delta.content {
                    // Set this content to other content.
                    self.delta.content = Some(other_content.clone());
                }
            }
        };

        // Merge tool calls.
        match self.delta.tool_calls.as_mut() {
            Some(tool_calls) => {
                if let Some(other_tool_calls) = &other.delta.tool_calls {
                    // Ensure we have enough tool calls in our vector
                    while tool_calls.len() < other_tool_calls.len() {
                        // Add empty tool calls to match the length
                        tool_calls.push(ToolCall {
                            id: String::new(),
                            r#type: "function".to_string(),
                            function: ToolCallFunction {
                                name: String::new(),
                                arguments: String::new(),
                            },
                        });
                    }

                    // Merge tool calls by index
                    for (i, other_tool_call) in other_tool_calls.iter().enumerate() {
                        if i < tool_calls.len() {
                            // If the ID is set in the other tool call but not in ours, copy it
                            if tool_calls[i].id.is_empty() && !other_tool_call.id.is_empty() {
                                tool_calls[i].id = other_tool_call.id.clone();
                            }

                            // If the type is empty in ours but set in the other, copy it
                            if tool_calls[i].r#type.is_empty() && !other_tool_call.r#type.is_empty()
                            {
                                tool_calls[i].r#type = other_tool_call.r#type.clone();
                            }

                            // If the function name is empty in ours but set in the other, copy it
                            if tool_calls[i].function.name.is_empty()
                                && !other_tool_call.function.name.is_empty()
                            {
                                tool_calls[i].function.name = other_tool_call.function.name.clone();
                            }

                            // Merge the arguments
                            if !other_tool_call.function.arguments.is_empty() {
                                tool_calls[i]
                                    .function
                                    .arguments
                                    .push_str(&other_tool_call.function.arguments);
                            }
                        }
                    }
                }
            }
            None => {
                if let Some(other_tool_calls) = &other.delta.tool_calls {
                    // Set this tool_calls to other tool_calls
                    self.delta.tool_calls = Some(other_tool_calls.clone());
                }
            }
        };

        Ok(())
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

/// Same as ChatCompletionMessage, but received during a response stream.
#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ChatCompletionMessageDelta {
    /// The role of the author of this message.
    pub role: Option<ChatCompletionMessageRole>,
    /// The contents of the message
    pub content: Option<String>,
    /// The name of the user in a multi-user chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Tool call that this message is responding to.
    /// Required if the role is `Tool`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    /// Tool calls that the assistant is requesting to invoke.
    /// Can only be populated if the role is `Assistant`,
    /// otherwise it should be empty.
    #[serde(skip_serializing_if = "is_none_or_empty_vec")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Default)]
pub struct ChatCompletionMessage {
    /// The role of the author of this message.
    pub role: ChatCompletionMessageRole,
    /// The contents of the message
    ///
    /// This is always required for all messages, except for when ChatGPT calls
    /// a function.
    pub content: Option<String>,
    /// The name of the user in a multi-user chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Tool call that this message is responding to.
    /// Required if the role is `Tool`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    /// Tool calls that the assistant is requesting to invoke.
    /// Can only be populated if the role is `Assistant`,
    /// otherwise it should be empty.
    #[serde(skip_serializing_if = "is_none_or_empty_vec")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct ToolCall {
    /// The ID of the tool call.
    #[serde(default)]
    pub id: String,
    /// The type of the tool. Currently, only `function` is supported.
    pub r#type: String,
    /// The function that the model called.
    pub function: ToolCallFunction,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct ToolCallResult {
    /// The ID of the tool call we are responding to.
    pub id: String,
    /// This will be the response in json format.
    pub result: serde_json::Value,
    /// The name of the function that was called.
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct ToolCallFunction {
    /// The name of the function to call.
    #[serde(default)]
    pub name: String,
    /// The arguments to call the function with, as generated by the model in
    /// JSON format.
    /// Note that the model does not always generate valid JSON, and may
    /// hallucinate parameters not defined by your function schema.
    /// Validate the arguments in your code before calling your function.
    pub arguments: String,
}

fn is_none_or_empty_vec<T>(opt: &Option<Vec<T>>) -> bool {
    opt.as_ref().map(|v| v.is_empty()).unwrap_or(true)
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ChatCompletionMessageRole {
    System,
    User,
    Assistant,
    Function,
    Tool,
    Developer,
}

impl Default for ChatCompletionMessageRole {
    fn default() -> Self {
        Self::User
    }
}
