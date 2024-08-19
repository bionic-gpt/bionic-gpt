#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::console::ChatWithChunks;
use crate::routes;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::{conversations::History, prompts::SinglePrompt};
use dioxus::prelude::*;

#[component]
pub fn Page(
    team_id: i32,
    rbac: Rbac,
    chats_with_chunks: Vec<ChatWithChunks>,
    prompt: SinglePrompt,
    conversation_id: i64,
    history: Vec<History>,
    lock_console: bool,
) -> Element {
    // Rerverse it because that's how we display it.
    let chats_with_chunks: Vec<ChatWithChunks> = chats_with_chunks.into_iter().rev().collect();
    rsx! {
        Layout {
            section_class: "console flex flex-col justify-start h-[calc(100%-79px)]",
            selected_item: SideBar::Prompts,
            team_id: team_id,
            rbac: rbac,
            title: "{prompt.name}",
            header: rsx!(
                h3 { "{prompt.name}" }
                if ! chats_with_chunks.is_empty() {
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
                        super::delete_conv::DeleteDrawer{
                            trigger_id: format!("delete-conv-{}", conversation_id),
                            team_id: team_id,
                            prompt_id: prompt.id,
                            conversation_id
                        }
                        form {
                            method: "get",
                            action: crate::routes::prompts::NewChat{team_id, prompt_id: prompt.id}.to_string(),
                            Button {
                                class: "mr-2",
                                button_scheme: ButtonScheme::Default,
                                button_type: ButtonType::Submit,
                                "New Chat"
                            }
                        }
                    }
                }
            ),
            div {
                id: "console-panel",
                class: "h-full",
                if chats_with_chunks.is_empty() {
                    EmptyStream {
                        prompt: prompt.clone(),
                        conversation_id,
                        team_id
                    }
                } else {
                    ConsoleStream {
                        chats_with_chunks,
                        team_id
                    }
                }
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
                            placeholder: "{prompt.disclaimer}",
                            disabled: lock_console
                        }
                        div {
                            input {
                                "type": "hidden",
                                name: "conversation_id",
                                value: "{conversation_id}"
                            }
                            input {
                                "type": "hidden",
                                name: "prompt_id",
                                value: "{prompt.id}"
                            }
                        }
                        div {
                            class: "flex flex-col justify-between",
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

#[component]
pub fn EmptyStream(prompt: SinglePrompt, conversation_id: i64, team_id: i32) -> Element {
    rsx! {
        div {
            class: "flex h-[calc(100%-100px)] overflow-y-auto justify-center items-center",
            div {
                class: "w-1/2 text-center",
                h1 {
                    class: "text-lg",
                    "{prompt.name}"
                }
                p {
                    class: "text-sm mt-2 mb-4",
                    "{prompt.description}"
                }
                div {
                    class: "flex gap-2 justify-center",
                    if let Some(example1) = prompt.example1 {
                        ExampleForm {
                            conversation_id,
                            team_id,
                            prompt_id: prompt.id,
                            example: example1
                        }
                    }
                    if let Some(example2) = prompt.example2 {
                        ExampleForm {
                            conversation_id,
                            team_id,
                            prompt_id: prompt.id,
                            example: example2
                        }
                    }
                    if let Some(example3) = prompt.example3 {
                        ExampleForm {
                            conversation_id,
                            team_id,
                            prompt_id: prompt.id,
                            example: example3
                        }
                    }
                    if let Some(example4) = prompt.example4 {
                        ExampleForm {
                            conversation_id,
                            team_id,
                            prompt_id: prompt.id,
                            example: example4
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ExampleForm(conversation_id: i64, prompt_id: i32, team_id: i32, example: String) -> Element {
    rsx! {
        form {
            method: "post",
            action: routes::console::SendMessage{team_id}.to_string(),
            input {
                "type": "hidden",
                name: "conversation_id",
                value: "{conversation_id}"
            }
            input {
                "type": "hidden",
                name: "prompt_id",
                value: "{prompt_id}"
            }
            input {
                "type": "hidden",
                name: "message",
                value: "{example}"
            }
            Button {
                button_type: ButtonType::Submit,
                "{example}"
            }
        }
    }
}

#[component]
pub fn ConsoleStream(chats_with_chunks: Vec<ChatWithChunks>, team_id: i32) -> Element {
    rsx! {

        div {
            class: "flex flex-col-reverse h-[calc(100%-100px)] overflow-y-auto",
            id: "console-stream",
            div {
                class: "flex flex-col-reverse min-w-[65ch] max-w-prose m-auto h-full",
                for chat_with_chunks in chats_with_chunks {
                    crate::console::prompt_drawer::PromptDrawer {
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
