#![allow(non_snake_case)]
use super::ChatWithChunks;
use crate::app_layout::{Layout, SideBar};
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::{conversations::History, prompts::Prompt};
use dioxus::prelude::*;
use assets::files::*;
use crate::routes;

#[component]
pub fn Page(
    team_id: i32,
    rbac: Rbac,
    chats_with_chunks: Vec<ChatWithChunks>,
    prompts: Vec<Prompt>,
    conversation_id: i64,
    history: Vec<History>,
    lock_console: bool,
) -> Element {
    rsx! {
        Layout {
            section_class: "console flex flex-col justify-start h-[calc(100%-79px)]",
            selected_item: SideBar::Console,
            team_id: team_id,
            rbac: rbac,
            title: "AI Chat Console",
            header: rsx!(
                h3 { "AI Chat Console" }
                div {
                    class: "flex flex-row",
                    Button {
                        class: "btn-circle mr-2 p-1",
                        drawer_trigger: "delete-conv-{conversation_id}",
                        button_scheme: ButtonScheme::Default,
                        img {
                            src: delete_svg.name
                        }
                    }
                    super::delete::DeleteDrawer{
                        trigger_id: format!("delete-conv-{}", conversation_id),
                        team_id: team_id,
                        id: conversation_id
                    }
                    form {
                        method: "post",
                        action: "{crate::routes::console::new_chat_route(team_id)}",
                        Button {
                            class: "mr-2",
                            button_scheme: ButtonScheme::Default,
                            button_type: ButtonType::Submit,
                            "New Chat"
                        }
                    }
                    Button {
                        drawer_trigger: "history-selector",
                        button_scheme: ButtonScheme::Default,
                        "Show History"
                    }
                    super::history_drawer::HistoryDrawer{
                        trigger_id: "history-selector".to_string(),
                        team_id: team_id,
                        history: history.clone()
                    }
                }
            ),
            div {
                id: "console-panel",
                class: "h-full",
                div {
                    class: "flex flex-col-reverse h-[calc(100%-100px)] overflow-y-auto",
                    id: "console-stream",
                    div {
                        class: "flex flex-col-reverse min-w-[65ch] max-w-prose m-auto h-full",
                        for chat_with_chunks in chats_with_chunks {
                            super::prompt_drawer::PromptDrawer {
                                trigger_id: format!("show-prompt-{}", chat_with_chunks.chat.id),
                                prompt: chat_with_chunks.chat.prompt.clone(),
                                chunks: chat_with_chunks.chunks.clone()
                            }
                            TimeLine {
                                if let Some(response) = &chat_with_chunks.chat.response {
                                    // We are generating text
                                    TimeLineBadge {
                                        image_src: handshake_svg.name
                                    }
                                    TimeLineBody {
                                        class: "prose",
                                        div  {
                                            class: "response-formatter",
                                            "{response}"
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
                                            action: "{routes::console::update_response_route(team_id)}",
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
                                        "Model: "
                                        strong {
                                            " {chat_with_chunks.chat.model_name}"
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
                                        "{chat_with_chunks.chat.user_request} "
                                    }
                                }
                            }
                        }
                    }
                }
                div {
                    class: "position-relative w-full bottom-0 p-2 border-t color-bg-subtle",
                    form {
                        class: "remember w-full flex max-h-[79px]",
                        method: "post",
                        "data-remember-name": "console-prompt",
                        "data-remember-reset": "false",
                        action: "{routes::console::send_message_route(team_id)}",

                        TextArea {
                            class: "submit-on-enter flex-1 mr-2",
                            rows: "4",
                            name: "message",
                            disabled: lock_console
                        }
                        div {
                            class: "flex flex-col justify-between",
                            div {
                                class: "flex flex-row ",
                                label {
                                    class: "mr-2",
                                    "Prompt"
                                }
                                input {
                                    "type": "hidden",
                                    name: "conversation_id",
                                    value: "{conversation_id}"
                                }
                                Select {
                                    name: "prompt_id",
                                    disabled: lock_console,
                                    for prompt in prompts {
                                        option {
                                            value: "{prompt.id}",
                                            "{prompt.name}"
                                        }
                                    }
                                }
                            }
                            Button {
                                disabled: lock_console,
                                button_type: ButtonType::Submit,
                                button_scheme: ButtonScheme::Primary,
                                "Send Message"
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn index(props: PageProps) -> String {
    crate::render(VirtualDom::new_with_props(Page, props))
}
