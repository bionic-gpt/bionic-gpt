#![allow(non_snake_case)]
use crate::console::tools_modal::ToolsModal;
use crate::routes;

use assets::files::*;
use daisy_rsx::*;
use db::queries::capabilities::Capability;
use db::types::public::ModelCapability;
use dioxus::prelude::*;

#[component]
pub fn Form(
    team_id: i32,
    prompt_id: i32,
    conversation_id: Option<i64>,
    lock_console: bool,
    disclaimer: String,
    capabilities: Vec<Capability>,
) -> Element {
    // Check if tool_use capability is present
    let has_tool_use = capabilities
        .iter()
        .any(|cap| cap.capability == ModelCapability::tool_use);

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
                            class: "pt-3 auto-expand max-h-96 text-sm submit-on-enter resize-none",
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
                            //AttachButton {
                            //    lock_console
                            //}
                            if has_tool_use {
                                ToolsButton {
                                    lock_console
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

        ToolsModal {}
    }
}

#[component]
fn ToolsButton(lock_console: bool) -> Element {
    rsx! {
        super::button::Button {
            button_scheme: super::button::ButtonScheme::Outline,
            disabled: lock_console, // Enable if tool_use capability is present
            prefix_image_src: tools_svg.name,
            modal_trigger: "tool-modal",
            "Tools"
        }
    }
}

#[component]
fn SpeechToTextButton(lock_console: bool) -> Element {
    rsx! {
        super::button::Button {
            id: "speech-to-text-button",
            class: "hidden",
            button_scheme: super::button::ButtonScheme::Outline,
            button_shape: super::button::ButtonShape::Circle,
            prefix_image_src: microphone_svg.name,
            suffix_image_src: stop_recording_svg.name,
        }
    }
}

#[component]
fn AttachButton(lock_console: bool) -> Element {
    rsx! {
        super::button::Button {
            button_scheme: super::button::ButtonScheme::Outline,
            button_shape: super::button::ButtonShape::Circle,
            disabled: true,
            prefix_image_src: attach_svg.name
        }
    }
}

#[component]
fn SendMessageButton(lock_console: bool) -> Element {
    rsx! {
        if lock_console {
            super::button::Button {
                button_type: super::button::ButtonType::Submit,
                button_scheme: super::button::ButtonScheme::Primary,
                button_shape: super::button::ButtonShape::Circle,
                id: "streaming-button",
                disabled: lock_console,
                prefix_image_src: streaming_stop_svg.name
            }
        } else {
            super::button::Button {
                button_type: super::button::ButtonType::Submit,
                button_scheme: super::button::ButtonScheme::Primary,
                button_shape: super::button::ButtonShape::Circle,
                id: "prompt-submit-button",
                disabled: lock_console,
                prefix_image_src: submit_button_svg.name
            }
        }
    }
}
