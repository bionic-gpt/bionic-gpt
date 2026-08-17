#![allow(non_snake_case)]
use crate::routes;

use assets::files::*;
use daisy_rsx::*;
use db::{authz::Rbac, ChatRole};
use dioxus::prelude::*;
use std::collections::HashMap;
use tool_runtime::{parse_reasoning, parse_tool_calls, ToolCall};

use super::reasoning_timeline::ReasoningTimeline;
use super::response_timeline::ResponseTimeline;
use super::tool_call_timeline::ToolCallTimeline;
use super::{ChatWithChunks, PendingChatState};

const CONSOLE_CONTENT_WIDTH: &str =
    "mx-auto pl-2 pr-2 md:max-w-3xl lg:max-w-160 xl:max-w-3xl w-full";

// Main ConsoleStream Component
#[component]
pub fn ConsoleStream(
    team_id: String,
    chat_history: Vec<ChatWithChunks>,
    pending_chat_state: PendingChatState,
    is_tts_disabled: bool,
    rbac: Rbac,
) -> Element {
    let tool_call_index = build_tool_call_index(&chat_history);

    rsx! {
        div {
            class: "flex-1 flex flex-col-reverse overflow-y-auto",

            // Handle different pending chat states
            match pending_chat_state {
                PendingChatState::PendingToolChats(tool_chats, last_chat_id) => rsx! {
                    div {
                        class: "flex flex-col {CONSOLE_CONTENT_WIDTH}",
                        // Show each pending tool chat
                        for tool_chat in tool_chats {
                            ToolCallTimeline {
                                team_id: team_id.clone(),
                                chat_id: tool_chat.id as i64,
                                pending: true,
                                tool_call_id: tool_chat.tool_call_id.clone(),
                                tool_call: tool_chat
                                    .tool_call_id
                                    .as_ref()
                                    .and_then(|id| tool_call_index.get(id))
                                    .cloned(),
                                response: tool_chat.content.clone(),
                            }
                        }
                        // This component has an id of 'streaming-chat' which
                        // gets picked up by the javascript and call the chat stream
                        ProcessingTimeline {
                            chat_id: last_chat_id as i64,
                            team_id: team_id.clone()
                        }
                    }
                },
                PendingChatState::PendingUserChat(pending_chat) => rsx! {
                    div {
                        class: "flex flex-col {CONSOLE_CONTENT_WIDTH}",
                        // Show user request and processing
                        UserRequestTimeline {
                            user_request: pending_chat.chat.content.clone().unwrap_or_default()
                        }
                        // This component has an id of 'streaming-chat' which
                        // gets picked up by the javascript and call the chat stream
                        ProcessingTimeline {
                            chat_id: pending_chat.chat.id as i64,
                            team_id: team_id.clone()
                        }
                    }
                },
                PendingChatState::None => rsx! { div {} }
            }

            // Show any chat history, these should all have been processed.
            for chat_with_chunks in chat_history.clone() {
                if rbac.can_view_system_prompt() {
                    super::prompt_modal::PromptModal {
                        trigger_id: format!("show-prompt-{}", chat_with_chunks.chat.id),
                        prompt: "{}".to_string(),
                        chunks: chat_with_chunks.chunks.clone(),
                        rbac: rbac.clone()
                    }
                }
                div {
                    class: "flex flex-col-reverse {CONSOLE_CONTENT_WIDTH}",

                    match chat_with_chunks.chat.role {
                        ChatRole::Assistant => rsx! {
                            {
                                let reasoning = parse_reasoning(chat_with_chunks.chat.tool_calls.as_deref());
                                if !reasoning.is_empty() {
                                    rsx! {
                                        ReasoningTimeline {
                                            reasoning
                                        }
                                    }
                                } else {
                                    rsx! {}
                                }
                            }
                            if let Some(content) = chat_with_chunks.chat.content.clone() {
                                if !content.is_empty() {
                                    ResponseTimeline {
                                        response: content,
                                        is_tts_disabled
                                    }
                                }
                            }
                        },
                        ChatRole::Tool => {
                            rsx! {
                                ToolCallTimeline {
                                    team_id: team_id.clone(),
                                    chat_id: chat_with_chunks.chat.id as i64,
                                    pending: false,
                                    tool_call_id: chat_with_chunks.chat.tool_call_id.clone(),
                                    tool_call: chat_with_chunks
                                        .chat
                                        .tool_call_id
                                        .as_ref()
                                        .and_then(|id| tool_call_index.get(id))
                                        .cloned(),
                                    response: chat_with_chunks.chat.content.clone(),
                                }
                            }
                        },
                        _ => rsx! {
                            UserRequestTimeline {
                                user_request: chat_with_chunks.chat.content.clone().unwrap_or_default()
                            }
                        }
                    }
                }
            }
        }
    }
}

fn build_tool_call_index(chat_history: &[ChatWithChunks]) -> HashMap<String, ToolCall> {
    let mut index = HashMap::new();
    for chat_with_chunks in chat_history {
        if chat_with_chunks.chat.role != ChatRole::Assistant {
            continue;
        }

        if let Some(tool_calls_json) = &chat_with_chunks.chat.tool_calls {
            for tool_call in parse_tool_calls(Some(tool_calls_json)) {
                index.insert(tool_call.id.clone(), tool_call.clone());
                if let Some(call_id) = tool_call.call_id.clone() {
                    index.insert(call_id, tool_call);
                }
            }
        }
    }

    index
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::DateTime;
    use db::{Chat, ChatStatus};
    use serde_json::json;
    use tool_runtime::ToolCallFunction;

    fn epoch() -> DateTime<chrono::FixedOffset> {
        DateTime::UNIX_EPOCH.fixed_offset()
    }

    fn chat_with_tool_calls(tool_calls: Vec<ToolCall>) -> ChatWithChunks {
        ChatWithChunks {
            chat: Chat {
                id: 1,
                conversation_id: 1,
                content: None,
                role: ChatRole::Assistant,
                tool_call_id: None,
                tool_calls: serde_json::to_string(&tool_calls).ok(),
                prompt_id: 1,
                model_name: "test-model".to_string(),
                attachments: None,
                status: ChatStatus::Success,
                created_at: epoch(),
                updated_at: epoch(),
            },
            chunks: Vec::new(),
        }
    }

    #[test]
    fn build_tool_call_index_indexes_id_and_call_id() {
        let tool_call = ToolCall {
            id: "provider_item_id".to_string(),
            call_id: Some("provider_call_id".to_string()),
            signature: None,
            additional_params: None,
            function: ToolCallFunction {
                name: "search_tool_functions".to_string(),
                arguments: json!({"query": "bitcoin price"}),
            },
        };
        let history = vec![chat_with_tool_calls(vec![tool_call])];

        let index = build_tool_call_index(&history);

        assert_eq!(
            index["provider_item_id"].function.arguments,
            json!({"query": "bitcoin price"})
        );
        assert_eq!(
            index["provider_call_id"].function.arguments,
            json!({"query": "bitcoin price"})
        );
    }
}

// Processing Timeline Component
#[component]
fn ProcessingTimeline(chat_id: i64, team_id: String) -> Element {
    rsx! {
        TimeLine {
            TimeLineBadge {
                image_src: spinner_svg.name
            }
            TimeLineBody {
                class: "prose",
                div {
                    id: "streaming-chat",
                    class: "whitespace-pre-wrap break-words",
                    "data-chatid": "{chat_id}",
                    span {
                        "Processing prompt"
                    }
                }
                ProcessingForm {
                    chat_id,
                    team_id
                }
            }
        }
    }
}

// Processing Timeline Component
#[component]
fn ProcessingForm(chat_id: i64, team_id: String) -> Element {
    rsx! {
        form {
            method: "post",
            id: "chat-form-{chat_id}",
            action: routes::console::UpdateResponse{team_id: team_id.clone()}.to_string(),
            input {
                name: "chat_id",
                value: "{chat_id}",
                "type": "hidden"
            }
        }
    }
}

// User Request Timeline Component
#[component]
fn UserRequestTimeline(user_request: String) -> Element {
    rsx! {
        TimeLine {
            TimeLineBadge {
                image_src: profile_svg.name
            }
            TimeLineBody {
                span {
                    class: "prose",
                    "{user_request} "
                }
            }
        }
    }
}
