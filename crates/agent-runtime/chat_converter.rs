use db::{Chat, ChatRole};
use rig::message::{AssistantContent, Message, ToolCall as RigToolCall, ToolFunction};
use rig::OneOrMany;
use tool_runtime::ToolCall;

/// Converts database chats into rig-native messages.
pub fn convert_chat_to_messages(conversation: Vec<Chat>) -> Vec<Message> {
    let mut messages: Vec<Message> = Vec::new();

    for chat in conversation {
        let tool_calls: Vec<ToolCall> = chat
            .tool_calls
            .as_ref()
            .and_then(|tool_calls| serde_json::from_str(tool_calls).ok())
            .unwrap_or_default();

        let content = chat.content.unwrap_or_default();

        let message = match chat.role {
            ChatRole::Assistant => {
                let mut items: Vec<AssistantContent> = Vec::new();
                if !content.trim().is_empty() {
                    items.push(AssistantContent::text(content));
                }

                for tool_call in tool_calls {
                    let arguments: serde_json::Value =
                        serde_json::from_str(&tool_call.function.arguments).unwrap_or_default();
                    items.push(AssistantContent::ToolCall(RigToolCall {
                        id: tool_call.id,
                        call_id: None,
                        function: ToolFunction {
                            name: tool_call.function.name,
                            arguments,
                        },
                    }));
                }

                let content = OneOrMany::many(items)
                    .unwrap_or_else(|_| OneOrMany::one(AssistantContent::text("")));
                Message::Assistant { id: None, content }
            }
            ChatRole::Tool => Message::tool_result_with_call_id(
                chat.tool_call_id.unwrap_or_else(|| "tool_call".to_string()),
                None,
                content,
            ),
            ChatRole::User | ChatRole::System | ChatRole::Developer => Message::user(content),
        };

        messages.push(message);
    }

    messages
}
