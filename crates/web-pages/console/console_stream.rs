#![allow(non_snake_case)]
use crate::routes;

use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use dioxus::prelude::*;

use super::{ChatWithChunks, PendingChat};

// Main ConsoleStream Component
#[component]
pub fn ConsoleStream(
    team_id: i32,
    chat_history: Vec<ChatWithChunks>,
    pending_chat: Option<PendingChat>,
    is_tts_disabled: bool,
    rbac: Rbac,
) -> Element {
    rsx! {
        div {
            class: "flex-1 flex flex-col-reverse overflow-y-auto",

            // If we are waiting for the model, then deal with it here.
            if let Some(pending_chat) = pending_chat {
                div {
                    class: "flex flex-col pl-2 pr-2 md:pr-0 md:pl-0 md:min-w-[65ch] max-w-prose mx-auto",
                    // Are we sending the result of tool calls to the model?
                    if let Some(tool_calls) = pending_chat.tool_calls {

                        UserRequestTimeline {
                            user_request: pending_chat.chat.user_request.clone()
                        }

                        for tool_call in tool_calls {
                            FunctionCallTimeline {
                                name: tool_call.function.name.clone(),
                                chat_id: pending_chat.chat.id as i64,
                                team_id
                            }
                        }
                        // This component has an id of 'streaming-chat' which
                        // get picked up by the javascript and call the chat stream
                        // At this stage we are sending the model results of the function calls
                        ProcessingTimeline {
                            chat_id: pending_chat.chat.id as i64,
                            team_id: team_id
                        }
                    } else {
                        UserRequestTimeline {
                            user_request: pending_chat.chat.user_request.clone()
                        }
                        // This component has an id of 'streaming-chat' which
                        // get picked up by the javascript and call the chat stream
                        ProcessingTimeline {
                            chat_id: pending_chat.chat.id as i64,
                            team_id: team_id
                        }
                    }
                }
            }

            // Show any chat history, these should all have been processed.
            for chat_with_chunks in chat_history {
                if rbac.can_view_system_prompt() {
                    super::prompt_drawer::PromptDrawer {
                        trigger_id: format!("show-prompt-{}", chat_with_chunks.chat.id),
                        prompt: chat_with_chunks.chat.prompt.clone(),
                        chunks: chat_with_chunks.chunks.clone(),
                        rbac: rbac.clone()
                    }
                }
                div {
                    class: "flex flex-col-reverse pl-2 pr-2 md:pr-0 md:pl-0 md:min-w-[65ch] max-w-prose mx-auto",

                    ResponseTimeline {
                        response: chat_with_chunks.chat.response.clone().unwrap_or_else(|| "The chat was interrupted".to_string()),
                        is_tts_disabled
                    }

                    ModelInfoTimeline {
                        model_name: chat_with_chunks.chat.model_name.clone(),
                        chat_id: chat_with_chunks.chat.id as i64,
                        has_response: chat_with_chunks.chat.response.is_some(),
                        rbac: rbac.clone()
                    }

                    UserRequestTimeline {
                        user_request: chat_with_chunks.chat.user_request.clone()
                    }
                }
            }
        }
    }
}

// Function Call Timeline Component
#[component]
fn FunctionCallTimeline(name: String, chat_id: i64, team_id: i32) -> Element {
    rsx! {
        TimeLine {
            TimeLineBadge {
                image_src: spinner_svg.name
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

// Model Info Timeline Component
#[component]
fn ModelInfoTimeline(model_name: String, chat_id: i64, has_response: bool, rbac: Rbac) -> Element {
    rsx! {
        TimeLine {
            class: "TimelineItem--condensed",
            TimeLineBadge {
                image_src: commit_svg.name
            }
            TimeLineBody {
                Label {
                    "Model:"
                    strong {
                        class: "ml-2",
                        "{model_name}"
                    }
                }

                if has_response && rbac.can_view_system_prompt() {
                    Label {
                        class: "ml-2",
                        a {
                            "data-drawer-target": "show-prompt-{chat_id}",
                            "View Prompt"
                        }
                    }
                }
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
