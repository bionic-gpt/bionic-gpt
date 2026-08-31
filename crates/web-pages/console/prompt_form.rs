#![allow(non_snake_case)]
use crate::routes;

use assets::files::*;
use daisy_rsx::*;
use db::queries::capabilities::Capability;
use db::types::ModelCapability;
use dioxus::prelude::*;

use super::CONSOLE_CONTENT_WIDTH;

#[component]
pub fn Form(
    team_id: String,
    model_id: i32,
    conversation_id: Option<i64>,
    lock_console: bool,
    disclaimer: String,
    capabilities: Vec<Capability>,
) -> Element {
    // Check if tool_use capability is present
    let has_tool_use = capabilities
        .iter()
        .any(|cap| cap.capability == ModelCapability::tool_use);

    let show_attach_button = has_tool_use;

    rsx! {
        div {
            class: "{CONSOLE_CONTENT_WIDTH}",

            Card {
                class: "flex flex-col gap-2 remember w-full rounded-2xl border border-base-300 bg-base-100 p-2 shadow-sm",
                form {
                    method: "post",
                    action: routes::console::SendMessage{team_id: team_id.clone()}.to_string(),
                    enctype: "multipart/form-data",

                    if let Some(conversation_id) = conversation_id {
                        input {
                            "type": "hidden",
                            name: "conversation_id",
                            value: "{conversation_id}"
                        }
                    }
                    input {
                        "type": "hidden",
                        name: "model_id",
                        value: "{model_id}"
                    }

                    div {
                        class: "flex flex-col",
                        TextArea {
                            class: "min-h-12 pt-3 auto-expand w-full max-h-96 text-sm submit-on-enter resize-none",
                            rows: "1",
                            placeholder: "Ask a question...",
                            name: "message",
                            disabled: lock_console,
                            required: true
                        }
                    }
                    div {
                        class: "flex flex-row pt-2 justify-between",

                        div {
                            class: "flex flex-row gap-2",
                            if show_attach_button {
                                AttachButton {
                                    lock_console,
                                    id: "attach-button"
                                }
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
fn SpeechToTextButton(lock_console: bool) -> Element {
    rsx! {
        Button {
            id: "speech-to-text-button",
            class: "hidden",
            disabled: lock_console,
            button_style: ButtonStyle::Outline,
            button_shape: ButtonShape::Circle,
            prefix_image_src: microphone_svg.name,
            suffix_image_src: stop_recording_svg.name,
        }
    }
}

#[component]
fn AttachButton(lock_console: bool, id: &'static str) -> Element {
    let max_files: usize = std::env::var("MAX_ATTACHMENTS")
        .unwrap_or_else(|_| "5".to_string())
        .parse()
        .unwrap_or(5);
    rsx! {
        button {
            id: id,
            class: "btn btn-outline btn-circle btn-sm",
            disabled: lock_console,
            "data-max-files": "{max_files}",
            img {
                class: "svg-icon",
                src: attach_svg.name,
                width: "16"
            }
        }
    }
}

#[component]
fn SendMessageButton(lock_console: bool) -> Element {
    rsx! {
        if lock_console {
            Button {
                button_scheme: ButtonScheme::Primary,
                button_shape: ButtonShape::Circle,
                id: "streaming-button",
                prefix_image_src: streaming_stop_svg.name
            }
        } else {
            Button {
                button_type: ButtonType::Submit,
                button_scheme: ButtonScheme::Primary,
                button_shape: ButtonShape::Circle,
                id: "prompt-submit-button",
                disabled: lock_console,
                prefix_image_src: submit_button_svg.name
            }
        }
    }
}
