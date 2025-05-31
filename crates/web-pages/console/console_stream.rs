#![allow(non_snake_case)]
use crate::routes;

use assets::files::*;
use daisy_rsx::*;
use db::{authz::Rbac, ChatRole};
use dioxus::prelude::*;
use openai_api::ToolCall;

use super::{ChatWithChunks, PendingChatState};

// Main ConsoleStream Component
#[component]
pub fn ConsoleStream(
    team_id: i32,
    chat_history: Vec<ChatWithChunks>,
    pending_chat_state: PendingChatState,
    is_tts_disabled: bool,
    rbac: Rbac,
) -> Element {
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
                            FunctionCallTimeline {
                                name: get_function_name_from_tool_calls(&tool_chat.tool_call_id, &chat_history.clone()),
                                chat_id: tool_chat.id as i64,
                                team_id,
                                pending: true
                            }
                        }
                        // This component has an id of 'streaming-chat' which
                        // gets picked up by the javascript and call the chat stream
                        ProcessingTimeline {
                            chat_id: last_chat_id as i64,
                            team_id
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
                            team_id
                        }
                    }
                },
                PendingChatState::None => rsx! { div {} }
            }

            // Show any chat history, these should all have been processed.
            for chat_with_chunks in chat_history.clone() {
                if rbac.can_view_system_prompt() {
                    super::prompt_drawer::PromptDrawer {
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
                            let function_name = get_function_name_from_tool_calls(
                                &chat_with_chunks.chat.tool_call_id,
                                &chat_history.clone()
                            );
                            rsx! {
                                FunctionCallTimeline {
                                    name: function_name,
                                    chat_id: chat_with_chunks.chat.id as i64,
                                    team_id,
                                    pending: false
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

// Helper function to extract function name from tool calls
fn get_function_name_from_tool_calls(
    tool_call_id: &Option<String>,
    chat_history: &Vec<ChatWithChunks>,
) -> String {
    if let Some(id) = tool_call_id {
        // Search through chat history for Assistant chats with tool_calls
        for chat_with_chunks in chat_history {
            if chat_with_chunks.chat.role == ChatRole::Assistant {
                if let Some(tool_calls_json) = &chat_with_chunks.chat.tool_calls {
                    // Parse the tool_calls JSON
                    if let Ok(tool_calls) = serde_json::from_str::<Vec<ToolCall>>(tool_calls_json) {
                        // Find the tool call with matching ID
                        for tool_call in tool_calls {
                            if tool_call.id == *id {
                                return tool_call.function.name;
                            }
                        }
                    }
                }
            }
        }
    }

    // Fallback if function name cannot be resolved
    format!("Tool Call {}", tool_call_id.as_deref().unwrap_or("Unknown"))
}

// Function Call Timeline Component
#[component]
fn FunctionCallTimeline(name: String, chat_id: i64, team_id: i32, pending: bool) -> Element {
    rsx! {
        TimeLine {
            TimeLineBadge {
                image_src: if pending { spinner_svg.name } else { tools_svg.name }
            }
            TimeLineBody {
                Label {
                    "Function Call:"
                    strong {
                        class: "ml-2",
                        "{name}"
                    }
                }
            }
        }
    }
}

// Response Timeline Component
#[component]
fn ResponseTimeline(response: String, is_tts_disabled: bool) -> Element {
    // Set up the markdown with the needed textensions
    let mut options = comrak::Options::default();
    options.extension.table = true;
    options.extension.strikethrough = true;
    options.extension.tagfilter = true;
    options.extension.tasklist = true;
    options.extension.autolink = true;
    options.extension.superscript = true;
    options.extension.footnotes = true;
    options.extension.multiline_block_quotes = true;
    options.extension.description_lists = true;
    options.extension.multiline_block_quotes = true;
    options.extension.math_code = true;
    options.extension.math_dollars = true;
    options.extension.shortcodes = true;
    options.extension.underline = true;
    options.extension.subscript = true;
    let response = comrak::markdown_to_html(&response, &options);

    rsx! {
        TimeLine {
            TimeLineBadge {
                image_src: handshake_svg.name
            }
            TimeLineBody {
                class: "prose",
                div {
                    class: "response-formatter",
                    dangerous_inner_html: "{response}"
                }
                div {
                    class: "hidden",
                    "{response}"
                }
                div {
                    if !is_tts_disabled {
                        ToolTip {
                            text: "Read aloud",
                            class: "mr-2",
                            img {
                                class: "read-aloud svg-icon mt-0 mb-0",
                                "data-loading-img": read_aloud_loading_svg.name,
                                "data-stop-img": read_aloud_stop_svg.name,
                                "data-play-img": read_aloud_svg.name,
                                src: read_aloud_svg.name,
                                width: "16",
                                height: "16"
                            }
                        }
                    }
                    ToolTip {
                        text: "Copy",
                        img {
                            class: "copy-response svg-icon mt-0 mb-0",
                            "clicked-img": tick_copy_svg.name,
                            src: copy_svg.name,
                            width: "16",
                            height: "16"
                        }
                    }
                }
            }
        }
    }
}

// Processing Timeline Component
#[component]
fn ProcessingTimeline(chat_id: i64, team_id: i32) -> Element {
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
fn ProcessingForm(chat_id: i64, team_id: i32) -> Element {
    rsx! {
        form {
            method: "post",
            id: "chat-form-{chat_id}",
            action: routes::console::UpdateResponse{team_id}.to_string(),
            input {
                name: "response",
                id: "chat-result-{chat_id}",
                "type": "hidden"
            }
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
