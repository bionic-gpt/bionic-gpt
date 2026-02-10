#![allow(non_snake_case)]
use crate::routes;

use assets::files::*;
use daisy_rsx::*;
use db::{authz::Rbac, ChatRole};
use dioxus::prelude::*;
use openai_api::ToolCall;
use std::collections::HashMap;

use super::response_timeline::ResponseTimeline;
use super::tool_call_timeline::ToolCallTimeline;
use super::{ChatWithChunks, PendingChatState};

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
                        class: "flex flex-col pl-2 pr-2 md:pr-0 md:pl-0 md:min-w-[65ch] max-w-prose mx-auto",
                        // Show each pending tool chat
                        for tool_chat in tool_chats {
                            ToolCallTimeline {
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
                        class: "flex flex-col pl-2 pr-2 md:pr-0 md:pl-0 md:min-w-[65ch] max-w-prose mx-auto",
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
                    class: "flex flex-col-reverse pl-2 pr-2 md:pr-0 md:pl-0 md:min-w-[65ch] max-w-prose mx-auto",

                    match chat_with_chunks.chat.role {
                        ChatRole::Assistant => rsx! {
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
            if let Ok(tool_calls) = serde_json::from_str::<Vec<ToolCall>>(tool_calls_json) {
                for tool_call in tool_calls {
                    index.insert(tool_call.id.clone(), tool_call);
                }
            }
        }
    }

    index
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
