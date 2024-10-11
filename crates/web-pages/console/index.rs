#![allow(non_snake_case)]
use super::ChatWithChunks;
use crate::app_layout::{Layout, SideBar};
use crate::routes;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::{conversations::History, prompts::Prompt};
use dioxus::prelude::*;

#[component]
pub fn Page(
    team_id: i32,
    rbac: Rbac,
    chats_with_chunks: Vec<ChatWithChunks>,
    prompts: Vec<Prompt>,
    conversation_id: i64,
    history: Vec<History>,
    lock_console: bool,
    is_tts_disabled: bool,
) -> Element {
    // Rerverse it because that's how we display it.
    let chats_with_chunks: Vec<ChatWithChunks> = chats_with_chunks.into_iter().rev().collect();
    rsx! {
        Layout {
            section_class: "console flex flex-col justify-start h-[calc(100%-79px)]",
            selected_item: SideBar::Console,
            team_id: team_id,
            rbac: rbac.clone(),
            title: "AI Chat Console",
            header: rsx!(
                Head {
                    team_id: team_id,
                    rbac: rbac.clone(),
                    conversation_id: conversation_id,
                    history: history.clone(),
                }
            ),
            div {
                id: "console-panel",
                class: "h-full",
                ConsolePanel {
                    team_id: team_id,
                    chats_with_chunks: chats_with_chunks,
                    is_tts_disabled: is_tts_disabled,
                    lock_console: lock_console,
                },
                Form {
                    team_id: team_id,
                    prompts: prompts,
                    conversation_id: conversation_id,
                    lock_console: lock_console,
                }
            }
        }
    }
}

#[component]
fn ConsolePanel(
    team_id: i32,
    chats_with_chunks: Vec<ChatWithChunks>,
    is_tts_disabled: bool,
    lock_console: bool,
) -> Element {
    rsx! {
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
                                                class: "read-aloud mt-0 mb-0",
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
                                            class: "copy-response mt-0 mb-0",
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

#[component]
fn Head(team_id: i32, rbac: Rbac, conversation_id: i64, history: Vec<History>) -> Element {
    rsx! {
        h3 { "AI Chat Console" }
        div {
            class: "flex flex-row",
            if rbac.can_delete_chat() {
                Button {
                    class: "btn-circle mr-2 p-1",
                    drawer_trigger: "delete-conv-{conversation_id}",
                    button_scheme: ButtonScheme::Default,
                    img {
                        class: "svg-icon",
                        src: delete_svg.name
                    }
                }
                super::delete::DeleteDrawer{
                    trigger_id: format!("delete-conv-{}", conversation_id),
                    team_id: team_id,
                    id: conversation_id
                }
            }
            form {
                method: "post",
                action: crate::routes::console::NewChat{team_id}.to_string(),
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
                "Recent Chats"
            }
            super::history_drawer::HistoryDrawer{
                trigger_id: "history-selector".to_string(),
                team_id: team_id,
                history: history.clone()
            }
        }
    }
}

#[component]
fn Form(team_id: i32, prompts: Vec<Prompt>, conversation_id: i64, lock_console: bool) -> Element {
    rsx! {
        div {
            class: "position-relative w-full bottom-0 p-2 border-t color-bg-subtle",
            form {
                class: "remember w-full flex max-h-[79px]",
                method: "post",
                "data-remember-name": "console-prompt",
                "data-remember-reset": "false",
                action: routes::console::SendMessage{team_id}.to_string(),

                TextArea {
                    class: "submit-on-enter flex-1 mr-2",
                    rows: "4",
                    name: "message",
                    disabled: lock_console
                }
                div {
                    class: "flex flex-col justify-between",
                    div {
                        class: "flex flex-row",
                        label {
                            class: "my-auto mr-2",
                            "Model"
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
