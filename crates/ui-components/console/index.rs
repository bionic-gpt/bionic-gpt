use crate::app_layout::{Layout, SideBar};
use assets::files::handshake_svg;
use db::queries::{chats::Chats, prompts::Prompt};
use dioxus::prelude::*;
use primer_rsx::*;

struct Props {
    organisation_id: i32,
    chats: Vec<Chats>,
    prompts: Vec<Prompt>,
    send_message_action: String,
    update_response_action: String,
}

pub fn index(
    organisation_id: i32,
    send_message_action: String,
    update_response_action: String,
    chats: Vec<Chats>,
    prompts: Vec<Prompt>,
) -> String {
    fn app(cx: Scope<Props>) -> Element {
        cx.render(rsx! {
            Layout {
                selected_item: SideBar::Console,
                team_id: cx.props.organisation_id,
                title: "AI Chat Console",
                header: cx.render(rsx!(
                    h3 { "AI Chat Console" }
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
                                        class: "color-bg-warning-emphasis color-fg-on-emphasis",
                                        image_src: handshake_svg.name
                                    }
                                    TimeLineBody {
                                        if let Some(response) = &chat.response {
                                            cx.render(rsx!(
                                                "{response}"
                                            ))
                                        } else {
                                            cx.render(rsx!(
                                                streaming-chat {
                                                    prompt: "{chat.prompt}",
                                                    "chat-id": "{chat.id}"
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
                                    TimeLineBadge {
                                        class: "color-bg-success-emphasis color-fg-on-emphasis",
                                        image_src: handshake_svg.name
                                    }
                                    TimeLineBody {
                                        span {
                                            "{chat.user_request} "
                                        }
                                        a {
                                            "data-drawer-target": "show-prompt-{chat.id}",
                                            "View Prompt"
                                        }
                                    }
                                }
                            ))
                        })
                    }
                    div {
                        class: "position-relative width-full bottom-0 p-2 border-top color-bg-subtle",
                        form {
                            class: "width-full d-flex flex-justify-between flex-items-center",
                            method: "post",
                            action: "{cx.props.send_message_action}",
                            textarea {
                                class: "flex-1 mr-2 form-control",
                                rows: "4",
                                name: "message"
                            }
                            div {
                                class: "d-flex flex-justify-between flex-column",
                                Select {
                                    name: "prompt_id",
                                    label: "Prompt",
                                    cx.props.prompts.iter().map(|prompt| rsx!(
                                        option {
                                            value: "{prompt.id}",
                                            "{prompt.name}"
                                        }
                                    ))
                                }
                                Button {
                                    button_type: ButtonType::Submit,
                                    button_scheme: ButtonScheme::Outline,
                                    "Send Message"
                                }
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
            send_message_action,
            update_response_action,
            chats,
            prompts,
        },
    ))
}
