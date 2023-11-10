use super::super::routes;
use crate::app_layout::{Layout, SideBar};
use assets::files::{commit_svg, handshake_svg, profile_svg};
use db::queries::{chats::Chat, conversations::History, prompts::Prompt};
use dioxus::prelude::*;
use primer_rsx::*;

struct Props {
    organisation_id: i32,
    chats: Vec<Chat>,
    prompts: Vec<Prompt>,
    conversation_id: i64,
    history: Vec<History>,
    lock_console: bool,
    send_message_action: String,
    update_response_action: String,
}

pub fn index(
    organisation_id: i32,
    conversation_id: i64,
    chats: Vec<Chat>,
    prompts: Vec<Prompt>,
    history: Vec<History>,
    lock_console: bool,
) -> String {
    fn app(cx: Scope<Props>) -> Element {
        cx.render(rsx! {
            Layout {
                section_class: "console",
                selected_item: SideBar::Console,
                team_id: cx.props.organisation_id,
                title: "AI Chat Console",
                header: cx.render(rsx!(
                    h3 { "AI Chat Console" }
                    div {
                        class: "d-flex flex-row",
                        form {
                            method: "post",
                            action: "{crate::routes::console::new_chat_route(cx.props.organisation_id)}",
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
                            organisation_id: cx.props.organisation_id,
                            history: cx.props.history.clone()
                        }
                    }
                )),
                div {
                    id: "console-panel",
                    div {
                        id: "console-stream",
                        class: "d-flex flex-column-reverse",
                        cx.props.chats.iter().rev().map(|chat| {
                            cx.render(rsx!(
                                super::prompt_drawer::PromptDrawer {
                                    trigger_id: format!("show-prompt-{}", chat.id),
                                    prompt: chat.prompt.clone()
                                }
                                TimeLine {
                                    TimeLineBadge {
                                        image_src: handshake_svg.name
                                    }
                                    TimeLineBody {
                                        if let Some(response) = &chat.response {
                                            cx.render(rsx!(
                                                response-formatter {
                                                    response: "{convert_quotes(response)}"
                                                }
                                            ))
                                        } else {
                                            cx.render(rsx!(
                                                streaming-chat {
                                                    prompt: "{chat.prompt}",
                                                    "chat-id": "{chat.id}",
                                                    span {
                                                        "Processing prompt"
                                                    }
                                                    span {
                                                        class: "AnimatedEllipsis"
                                                    }
                                                }
                                                form {
                                                    method: "post",
                                                    id: "chat-form-{chat.id}",
                                                    action: "{cx.props.update_response_action}",
                                                    input {
                                                        name: "response",
                                                        id: "chat-result-{chat.id}",
                                                        "type": "hidden"
                                                    }
                                                    input {
                                                        name: "chat_id",
                                                        value: "{chat.id}",
                                                        "type": "hidden"
                                                    }
                                                }
                                            ))
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
                                                "{chat.model_name}"
                                            }
                                        }

                                        if chat.response.is_none() {
                                            cx.render(rsx!(
                                                Label {
                                                    class: "ml-2",
                                                    label_color: LabelColor::Danger,
                                                    a {
                                                        id: "stop-processing",
                                                        "Stop Processing"
                                                    }
                                                }
                                            ))
                                        } else {
                                            cx.render(rsx!(
                                                Label {
                                                    class: "ml-2",
                                                    a {
                                                        "data-drawer-target": "show-prompt-{chat.id}",
                                                        "View Prompt"
                                                    }
                                                }
                                            ))
                                        }
                                    }
                                }
                                TimeLine {
                                    TimeLineBadge {
                                        image_src: profile_svg.name
                                    }
                                    TimeLineBody {
                                        span {
                                            "{chat.user_request} "
                                        }
                                    }
                                }
                            ))
                        })
                    }
                    div {
                        class: "position-relative width-full bottom-0 p-2 border-top color-bg-subtle",
                        form {
                            class: "remember width-full d-flex",
                            method: "post",
                            "data-remember-name": "console-prompt",
                            "data-remember-reset": "false",
                            action: "{cx.props.send_message_action}",
                            if cx.props.lock_console {
                                cx.render(rsx!(
                                    textarea {
                                        class: "flex-1 mr-2 form-control",
                                        rows: "4",
                                        name: "message",
                                        disabled: true
                                    }
                                    div {
                                        class: "d-flex flex-column flex-justify-between",
                                        div {
                                            class: "d-flex flex-row ",
                                            label {
                                                class: "mr-2",
                                                "Prompt"
                                            }
                                            Select {
                                                name: "prompt_id",
                                                disabled: true,
                                                cx.props.prompts.iter().map(|prompt| rsx!(
                                                    option {
                                                        value: "{prompt.id}",
                                                        "{prompt.name}"
                                                    }
                                                ))
                                            }
                                        }
                                        Button {
                                            disabled: true,
                                            button_type: ButtonType::Submit,
                                            button_scheme: ButtonScheme::Default,
                                            "Send Message"
                                        }
                                    }
                                ))
                            } else {
                                cx.render(rsx!(
                                    textarea {
                                        class: "submit-on-enter flex-1 mr-2 form-control",
                                        rows: "4",
                                        name: "message"
                                    }
                                    div {
                                        class: "d-flex flex-column flex-justify-between",
                                        div {
                                            class: "d-flex flex-row ",
                                            label {
                                                class: "mr-2",
                                                "Prompt"
                                            }
                                            input {
                                                "type": "hidden",
                                                name: "conversation_id",
                                                value: "{cx.props.conversation_id}"
                                            }
                                            Select {
                                                name: "prompt_id",
                                                cx.props.prompts.iter().map(|prompt| rsx!(
                                                    option {
                                                        value: "{prompt.id}",
                                                        "{prompt.name}"
                                                    }
                                                ))
                                            }
                                        }
                                        Button {
                                            button_type: ButtonType::Submit,
                                            button_scheme: ButtonScheme::Primary,
                                            "Send Message"
                                        }
                                    }
                                ))
                            }
                        }
                    }
                }
            }
        })
    }

    crate::render(VirtualDom::new_with_props(
        app,
        Props {
            organisation_id,
            chats,
            prompts,
            conversation_id,
            history,
            lock_console,
            send_message_action: routes::console::send_message_route(organisation_id),
            update_response_action: routes::console::update_response_route(organisation_id),
        },
    ))
}

fn convert_quotes(str: &str) -> String {
    str.replace('"', "&quot;")
}
