#![allow(non_snake_case)]
use crate::routes;

use assets::files::*;
use daisy_rsx::*;
use dioxus::prelude::*;

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

            div {
                class: "flex flex-col gap-2 remember w-full p-2 rounded-lg border",
                form {
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
                        name: "prompt_id",
                        value: "{prompt_id}"
                    }

                    div {
                        class: "flex flex-col",
                        TextArea {
                            class: "pt-3 auto-expand max-h-96 text-sm submit-on-enter flex-1 resize-none",
                            rows: "1",
                            placeholder: "Ask a question...",
                            name: "message",
                            disabled: lock_console
                        }
                    }
                    div {
                        class: "flex flex-row pt-5 justify-between",

                        div {
                            class: "flex flex-row gap-2",
                            AttachButton {
                                lock_console
                            }
                            ToolsButton {
                                lock_console
                            }
                        }

                        div {
                            class: "flex flex-row gap-2",
                            SpeechToTextButton {
                                lock_console
                            }

                            SendMessageButton {
                                lock_console
                            }
                        }
                    }
                }
            }
            p {
                class: "text-xs text-center p-2",
                "{disclaimer}"
            }
        }
    }
}

#[component]
fn ToolsButton(lock_console: bool) -> Element {
    rsx! {
        Button {
            disabled: lock_console,
            prefix_image_src: microphone_svg.name,
            "Tools"
        }
    }
}

#[component]
fn SpeechToTextButton(lock_console: bool) -> Element {
    rsx! {
        Button {
            disabled: lock_console,
            prefix_image_src: microphone_svg.name
        }
    }
}

#[component]
fn AttachButton(lock_console: bool) -> Element {
    rsx! {
        Button {
            disabled: lock_console,
            prefix_image_src: attach_svg.name
        }
    }
}

#[component]
fn SendMessageButton(lock_console: bool) -> Element {
    rsx! {
        if lock_console {
            Button {
                id: "streaming-button",
                disabled: lock_console,
                prefix_image_src: streaming_stop_svg.name
            }
        } else {
            Button {
                id: "prompt-submit-button",
                disabled: lock_console,
                prefix_image_src: submit_button_svg.name
            }
        }
    }
}
