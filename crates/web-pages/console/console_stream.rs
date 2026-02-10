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
    team_id: String,
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
                            ToolCallTimeline {
                                chat_id: tool_chat.id as i64,
                                team_id: team_id.clone(),
                                pending: true,
                                tool_call_id: tool_chat.tool_call_id.clone(),
                                chat_history: chat_history.clone(),
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
                            rsx! {
                                ToolCallTimeline {
                                    chat_id: chat_with_chunks.chat.id as i64,
                                    team_id: team_id.clone(),
                                    pending: false,
                                    tool_call_id: chat_with_chunks.chat.tool_call_id.clone(),
                                    chat_history: chat_history.clone(),
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

// Helper to extract tool call details from chat history
fn resolve_tool_call(
    tool_call_id: &Option<String>,
    chat_history: &Vec<ChatWithChunks>,
) -> (String, Option<ToolCall>) {
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
                                let display_name = if tool_call.function.name.is_empty() {
                                    format!("Tool Call {}", id)
                                } else {
                                    tool_call.function.name.clone()
                                };
                                return (display_name, Some(tool_call));
                            }
                        }
                    }
                }
            }
        }
    }

    // Fallback if function name cannot be resolved
    (
        format!("Tool Call {}", tool_call_id.as_deref().unwrap_or("Unknown")),
        None,
    )
}

fn format_json_string(raw: &str) -> String {
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(raw) {
        serde_json::to_string_pretty(&value).unwrap_or_else(|_| raw.to_string())
    } else {
        raw.to_string()
    }
}

// Tool Call Timeline Component
#[component]
fn ToolCallTimeline(
    chat_id: i64,
    team_id: String,
    pending: bool,
    tool_call_id: Option<String>,
    chat_history: Vec<ChatWithChunks>,
    response: Option<String>,
) -> Element {
    let (display_name, tool_call) = resolve_tool_call(&tool_call_id, &chat_history);
    let modal_tool_name = display_name.clone();
    let trigger_suffix = tool_call_id
        .clone()
        .filter(|id| !id.is_empty())
        .unwrap_or_else(|| chat_id.to_string());
    let trigger_id = format!("tool-call-details-{}", trigger_suffix);
    let request_body = tool_call
        .as_ref()
        .map(|call| format_json_string(&call.function.arguments));
    let response_body = response
        .as_ref()
        .map(|body| format_json_string(body))
        .filter(|body| !body.trim().is_empty());

    rsx! {
        TimeLine {
            TimeLineBadge {
                image_src: if pending { spinner_svg.name } else { tools_svg.name }
            }
            TimeLineBody {
                div {
                    class: "flex items-center gap-2",
                    Badge {
                        badge_style: BadgeStyle::Outline,
                        badge_size: BadgeSize::Sm,
                        "Tool Call:"
                        strong {
                            class: "ml-2",
                            "{display_name}"
                        }
                    }
                    Button {
                        class: "btn-xs",
                        button_style: ButtonStyle::Outline,
                        button_shape: ButtonShape::Circle,
                        popover_target: trigger_id.clone(),
                        button_scheme: ButtonScheme::Neutral,
                        img {
                            class: "svg-icon",
                            src: button_plus_svg.name
                        }
                        span {
                            class: "sr-only",
                            "View tool call details"
                        }
                    }
                }
            }
        }
        Modal {
            trigger_id: trigger_id.clone(),
            ModalBody {
                h3 {
                    class: "font-bold text-lg mb-4",
                    "Tool Call Details"
                }
                dl {
                    class: "space-y-4",
                    if let Some(call) = tool_call.as_ref() {
                        div {
                            class: "space-y-2",
                            dt { class: "font-semibold text-sm uppercase text-base-content/70", "Tool" }
                            dd { class: "text-sm break-words", "{modal_tool_name}" }
                        }
                        div {
                            class: "space-y-2",
                            dt { class: "font-semibold text-sm uppercase text-base-content/70", "Call ID" }
                            dd { class: "text-sm break-all", "{call.id}" }
                        }
                    } else if let Some(id) = tool_call_id.clone() {
                        div {
                            class: "space-y-2",
                            dt { class: "font-semibold text-sm uppercase text-base-content/70", "Call ID" }
                            dd { class: "text-sm break-all", "{id}" }
                        }
                    }
                    div {
                        class: "space-y-2",
                        dt { class: "font-semibold text-sm uppercase text-base-content/70", "Request" }
                        if let Some(body) = request_body.as_ref() {
                            pre {
                                class: "json bg-base-200 p-4 rounded overflow-auto max-h-96 text-sm",
                                "{body}"
                            }
                        } else {
                            dd {
                                class: "text-sm text-base-content/70",
                                "No request payload available."
                            }
                        }
                    }
                    div {
                        class: "space-y-2",
                        dt { class: "font-semibold text-sm uppercase text-base-content/70", "Response" }
                        if let Some(body) = response_body.as_ref() {
                            pre {
                                class: "json bg-base-200 p-4 rounded overflow-auto max-h-96 text-sm",
                                "{body}"
                            }
                        } else if pending {
                            dd {
                                class: "text-sm text-base-content/70",
                                "Awaiting tool response..."
                            }
                        } else {
                            dd {
                                class: "text-sm text-base-content/70",
                                "No response recorded."
                            }
                        }
                    }
                }
                ModalAction {
                    Button {
                        class: "cancel-modal",
                        button_scheme: ButtonScheme::Warning,
                        "Close"
                    }
                }
            }
        }
    }
}

// Response Timeline Component
#[component]
fn ResponseTimeline(response: String, is_tts_disabled: bool) -> Element {
    // Set up the markdown with the needed extensions
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
    let markdown = response;
    let html = comrak::markdown_to_html(&markdown, &options);

    rsx! {
        TimeLine {
            TimeLineBadge {
                image_src: handshake_svg.name
            }
            TimeLineBody {
                class: "prose",
                div {
                    class: "response-formatter",
                    dangerous_inner_html: "{html}"
                }
                div {
                    class: "hidden markdown-response",
                    "{markdown}"
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
