use db::Chat;
use openai_api::{ChatCompletionMessage, ChatCompletionMessageRole, ToolCall, ToolCallFunction};

pub fn convert_chat_to_messages(conversation: Vec<Chat>) -> Vec<ChatCompletionMessage> {
    let mut messages: Vec<ChatCompletionMessage> = Default::default();
    for chat in conversation {
        if let Some(function_call) = chat.function_call {
            messages.push(ChatCompletionMessage {
                role: ChatCompletionMessageRole::User,
                content: Some(chat.user_request),
                tool_call_id: None,
                tool_calls: None,
                name: None,
            });
            // Parse the function call JSON to extract necessary information
            if let Ok(function_call_json) =
                serde_json::from_str::<serde_json::Value>(&function_call)
            {
                let id = function_call_json["id"]
                    .as_str()
                    .unwrap_or("call_id")
                    .to_string();
                let function_type = function_call_json["type"]
                    .as_str()
                    .unwrap_or("function")
                    .to_string();
                let function_name = function_call_json["function"]["name"]
                    .as_str()
                    .unwrap_or("")
                    .to_string();
                let function_arguments = function_call_json["function"]["arguments"].to_string();

                // Create an assistant message with tool_calls
                messages.push(ChatCompletionMessage {
                    role: ChatCompletionMessageRole::Assistant,
                    content: None,
                    tool_call_id: None,
                    tool_calls: Some(vec![ToolCall {
                        id: id.clone(),
                        r#type: function_type,
                        function: ToolCallFunction {
                            name: function_name.clone(),
                            arguments: function_arguments,
                        },
                    }]),
                    name: None,
                });

                // Add tool response if results exist
                if let Some(results) = chat.function_call_results {
                    messages.push(ChatCompletionMessage {
                        role: ChatCompletionMessageRole::Tool,
                        content: Some(results),
                        tool_call_id: Some(id),
                        name: Some(function_name),
                        tool_calls: None,
                    });
                }
            } else {
                // Fallback if JSON parsing fails
                messages.push(ChatCompletionMessage {
                    role: ChatCompletionMessageRole::Function,
                    content: Some(function_call),
                    tool_call_id: None,
                    tool_calls: None,
                    name: None,
                });

                if let Some(results) = chat.function_call_results {
                    messages.push(ChatCompletionMessage {
                        role: ChatCompletionMessageRole::Tool,
                        content: Some(results),
                        tool_call_id: None,
                        tool_calls: None,
                        name: None,
                    });
                }
            }
        } else {
            messages.push(ChatCompletionMessage {
                role: ChatCompletionMessageRole::User,
                content: Some(chat.user_request),
                tool_call_id: None,
                tool_calls: None,
                name: None,
            });
            if let Some(response) = chat.response {
                messages.push(ChatCompletionMessage {
                    role: ChatCompletionMessageRole::Assistant,
                    content: Some(response),
                    tool_call_id: None,
                    tool_calls: None,
                    name: None,
                });
            }
        };
    }
    messages
}
