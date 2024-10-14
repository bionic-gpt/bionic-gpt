#![allow(non_snake_case)]
use crate::routes;

use assets::files::*;
use daisy_rsx::*;
use dioxus::prelude::*;

use super::ChatWithChunks;

#[component]
pub fn ConsolePanel(
    team_id: i32,
    chats_with_chunks: Vec<ChatWithChunks>,
    is_tts_disabled: bool,
    lock_console: bool,
) -> Element {
    rsx! {
        div {
            class: "flex flex-col-reverse h-[calc(100%-100px)]",
            id: "console-stream",
            div {
                class: "flex flex-col-reverse h-full overflow-y-auto",
                for chat_with_chunks in chats_with_chunks {
                    super::prompt_drawer::PromptDrawer {
                        trigger_id: format!("show-prompt-{}", chat_with_chunks.chat.id),
                        prompt: chat_with_chunks.chat.prompt.clone(),
                        chunks: chat_with_chunks.chunks.clone()
                    }
                    div {
                        class: "min-w-[65ch] max-w-prose m-auto",
                        TimeLine {
                            if let Some(response) = &chat_with_chunks.chat.response {
                                // We are generating text
                                TimeLineBadge {
                                    image_src: handshake_svg.name
                                }
                                TimeLineBody {
                                    class: "prose",
                                    div {
                                        class: "response-formatter",
                                        dangerous_inner_html: "{comrak::markdown_to_html(response, &comrak::Options::default())}"
                                    }
                                    div {
                                        class: "hidden",
                                        "{response}"
                                    }
                                    div {
                                        if ! is_tts_disabled {
                                            ToolTip {
                                                text: "Read aloud",
                                                class: "mr-2",
                                                img {
                                                    class: "read-aloud svg-icon mt-0 mb-0",
                                                    "data-loading-img": loading_svg.name,
                                                    "data-stop-img": stop_svg.name,
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
                            } else {
                                // The generated text
                                TimeLineBadge {
                                    image_src: spinner_svg.name
                                }
                                TimeLineBody {
                                    class: "prose",
                                    div {
                                        id: "streaming-chat",
                                        "data-prompt": "{chat_with_chunks.chat.prompt}",
                                        "data-chatid": "{chat_with_chunks.chat.id}",
                                        span {
                                            "Processing prompt"
                                        }
                                    }
                                    form {
                                        method: "post",
                                        id: "chat-form-{chat_with_chunks.chat.id}",
                                        action: routes::console::UpdateResponse{team_id}.to_string(),
                                        input {
                                            name: "response",
                                            id: "chat-result-{chat_with_chunks.chat.id}",
                                            "type": "hidden"
                                        }
                                        input {
                                            name: "chat_id",
                                            value: "{chat_with_chunks.chat.id}",
                                            "type": "hidden"
                                        }
                                    }
                                }
                            }
                        }
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
                                        "{chat_with_chunks.chat.model_name}"
                                    }
                                }

                                if chat_with_chunks.chat.response.is_none() {
                                    Label {
                                        class: "ml-2",
                                        label_role: LabelRole::Highlight,
                                        a {
                                            id: "stop-processing",
                                            "Stop Processing"
                                        }
                                    }
                                } else {
                                    Label {
                                        class: "ml-2",
                                        a {
                                            "data-drawer-target": "show-prompt-{chat_with_chunks.chat.id}",
                                            "View Prompt"
                                        }
                                    }
                                }
                            }
                        }
                        TimeLine {
                            TimeLineBadge {
                                image_src: profile_svg.name
                            }
                            TimeLineBody {
                                span {
                                    class: "prose",
                                    "{chat_with_chunks.chat.user_request} "
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
