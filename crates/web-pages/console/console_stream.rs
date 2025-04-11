#![allow(non_snake_case)]
use crate::routes;

use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use dioxus::prelude::*;

use super::ChatWithChunks;

// Main ConsoleStream Component
#[component]
pub fn ConsoleStream(
    team_id: i32,
    chats_with_chunks: Vec<ChatWithChunks>,
    is_tts_disabled: bool,
    rbac: Rbac,
) -> Element {
    rsx! {
        div {
            class: "flex-1 flex flex-col-reverse overflow-y-auto",
            for chat_with_chunks in chats_with_chunks {
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

                    // Render appropriate timeline based on chat state
                    if let Some(function_call_results) = chat_with_chunks.get_function_call_results() {
                        FunctionCallTimeline {
                            name: function_call_results.name.clone()
                        }
                        ProcessingTimeline {
                            chat_id: chat_with_chunks.chat.id as i64,
                            prompt: chat_with_chunks.chat.prompt.clone(),
                            team_id: team_id
                        }
                    }
                    else if let Some(response) = &chat_with_chunks.chat.response {
                        ResponseTimeline {
                            response: response.clone(),
                            is_tts_disabled: is_tts_disabled
                        }
                    } else {
                        ProcessingTimeline {
                            chat_id: chat_with_chunks.chat.id as i64,
                            prompt: chat_with_chunks.chat.prompt.clone(),
                            team_id: team_id
                        }
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
fn FunctionCallTimeline(name: String) -> Element {
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
    rsx! {
        TimeLine {
            TimeLineBadge {
                image_src: handshake_svg.name
            }
            TimeLineBody {
                class: "prose",
                div {
                    class: "response-formatter",
                    dangerous_inner_html: "{comrak::markdown_to_html(&response, &comrak::Options::default())}"
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
fn ProcessingTimeline(chat_id: i64, prompt: String, team_id: i32) -> Element {
    rsx! {
        TimeLine {
            TimeLineBadge {
                image_src: spinner_svg.name
            }
            TimeLineBody {
                class: "prose",
                div {
                    id: "streaming-chat",
                    "data-prompt": "{prompt}",
                    "data-chatid": "{chat_id}",
                    span {
                        "Processing prompt"
                    }
                }
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
