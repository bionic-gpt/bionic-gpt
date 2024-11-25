#![allow(non_snake_case)]
use crate::routes;

use assets::files::*;
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
fn AttachButton() -> Element {
    rsx! {
        div {
            class: "h-8 w-8 p-2 bg-secondary rounded-full",
            input {
                id: "fileInput",
                "type": "file",
                name: "attachments",
                multiple: "multiple",
                class: "hidden"
            }
            label {
                "for": "fileInput",
                img {
                    class: "svg-icon",
                    width: "48",
                    height: "48",
                    src: attach_svg.name
                }
            }
        }
    }
}

#[component]
fn SendMessageButton(lock_console: bool) -> Element {
    rsx! {
        if lock_console {
            button {
                id: "streaming-button",
                class: "h-8 w-8 p-2 bg-primary rounded-full",
                "type": "submit",
                img {
                    class: "svg-icon",
                    width: "48",
                    height: "48",
                    src: streaming_stop_svg.name
                }
            }
        } else {
            button {
                id: "prompt-submit-button",
                class: "h-8 w-8 p-2 bg-primary rounded-full",
                "type": "submit",
                img {
                    class: "svg-icon",
                    width: "48",
                    height: "48",
                    src: submit_button_svg.name
                }
            }
        }
    }
}

#[component]
pub fn Form(
    team_id: i32,
    prompt_id: i32,
    conversation_id: Option<i64>,
    lock_console: bool,
    disclaimer: String,
) -> Element {
    rsx! {
        div {
            class: "mx-auto pl-2 pr-2 md:max-w-3xl lg:max-w-[40rem] xl:max-w-[48rem]",

            form {
                class: "flex items-center gap-2 remember w-full bg-base-200 p-2 rounded-lg",
                method: "post",
                "data-remember-name": "console-prompt",
                "data-remember-reset": "false",
                action: routes::console::SendMessage{team_id}.to_string(),

                if let Some(conversation_id) = conversation_id {
                    input {
                        "type": "hidden",
                        name: "conversation_id",
                        value: "{conversation_id}"
                    }
                }
                input {
                    "type": "hidden",
                    class: "set-my-prompt-id",
                    name: "prompt_id",
                    value: "{prompt_id}"
                }

                TextArea {
                    class: "pt-3 auto-expand max-h-96 text-sm submit-on-enter flex-1 resize-none",
                    rows: "1",
                    placeholder: "Ask a question...",
                    name: "message",
                    disabled: lock_console
                }

                SendMessageButton {
                    lock_console
                }
            }
            p {
                class: "text-xs text-center p-2",
                "{disclaimer}"
            }
        }
    }
}
