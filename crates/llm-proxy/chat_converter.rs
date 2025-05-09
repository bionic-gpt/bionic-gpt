use db::Chat;
use openai_api::{ChatCompletionMessage, ChatCompletionMessageRole};

pub fn convert_chat_to_messages(conversation: Vec<Chat>) -> Vec<ChatCompletionMessage> {
    let mut messages: Vec<ChatCompletionMessage> = Default::default();
    for chat in conversation {
        if let Some(tool_calls_json) = chat.tool_calls {
            messages.push(ChatCompletionMessage {
                role: ChatCompletionMessageRole::User,
                content: Some(chat.user_request),
                tool_call_id: None,
                tool_calls: None,
                name: None,
            });
            // Parse the function call JSON to extract necessary information
            if let Ok(tool_calls) =
                serde_json::from_str::<Vec<openai_api::ToolCall>>(&tool_calls_json)
            {
                // Create an assistant message with tool_calls
                messages.push(ChatCompletionMessage {
                    role: ChatCompletionMessageRole::Assistant,
                    content: None,
                    tool_call_id: None,
                    tool_calls: Some(tool_calls),
                    name: None,
                });

                // Add tool response if results exist
                if let Some(results) = chat.tool_call_results {
                    if let Ok(tool_call_results) =
                        serde_json::from_str::<Vec<openai_api::ToolCallResult>>(&results)
                    {
                        for result in tool_call_results {
                            messages.push(ChatCompletionMessage {
                                role: ChatCompletionMessageRole::Tool,
                                content: Some(result.result),
                                tool_call_id: Some(result.id),
                                name: Some(result.name),
                                tool_calls: None,
                            });
                        }
                    }
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
            if let Some(attachments) = chat.attachments {
                messages.push(ChatCompletionMessage {
                    role: ChatCompletionMessageRole::Assistant,
                    content: Some(format!(
                        "The user has uploaded the following attachments {}",
                        attachments
                    )),
                    tool_call_id: None,
                    tool_calls: None,
                    name: None,
                });
            }
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
