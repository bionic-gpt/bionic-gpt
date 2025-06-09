#![allow(non_snake_case)]
use crate::console::tools_modal::ToolsModal;
use crate::routes;

use assets::files::*;
use daisy_rsx::*;
use db::queries::capabilities::Capability;
use db::types::public::ModelCapability;
use dioxus::prelude::*;
use openai_api::BionicToolDefinition;

#[component]
pub fn Form(
    team_id: i32,
    prompt_id: i32,
    conversation_id: Option<i64>,
    lock_console: bool,
    disclaimer: String,
    capabilities: Vec<Capability>,
    enabled_tools: Vec<String>,
    available_tools: Vec<BionicToolDefinition>,
) -> Element {
    // Check if tool_use capability is present
    let has_tool_use = capabilities
        .iter()
        .any(|cap| cap.capability == ModelCapability::tool_use);

    let show_attach_button = has_tool_use;

    rsx! {
        div {
            class: "mx-auto pl-2 pr-2 md:max-w-3xl lg:max-w-160 xl:max-w-3xl",

            Card {
                class: "flex flex-col gap-2 remember w-full p-2",
                form {
                    method: "post",
                    action: routes::console::SendMessage{team_id}.to_string(),
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
                        name: "prompt_id",
                        value: "{prompt_id}"
                    }

                    div {
                        class: "flex flex-col",
                        TextArea {
                            class: "pt-3 auto-expand w-full max-h-96 text-sm submit-on-enter resize-none",
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
                            if show_attach_button {
                                AttachButton {
                                    lock_console,
                                    id: "attach-button"
                                }
                            }
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

        // Pass both enabled_tools and available_tools to ToolsModal
        ToolsModal {
            enabled_tools,
            available_tools
        }
    }
}

#[component]
fn ToolsButton(lock_console: bool) -> Element {
    rsx! {
        Button {
            button_style: ButtonStyle::Outline,
            disabled: lock_console, // Enable if tool_use capability is present
            prefix_image_src: tools_svg.name,
            popover_target: "tool-modal",
            "Tools"
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
    rsx! {
        Button {
            id: id,
            button_style: ButtonStyle::Outline,
            button_shape: ButtonShape::Circle,
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
