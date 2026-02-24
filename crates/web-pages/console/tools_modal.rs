#![allow(non_snake_case)]
use crate::routes;
use daisy_rsx::*;
use dioxus::prelude::*;
use tool_runtime::ToolDefinition;

#[component]
pub fn ToolsModal(enabled_tools: Vec<String>, available_tools: Vec<ToolDefinition>) -> Element {
    rsx!(
        form {
            action: routes::console::SetTools{}.to_string(),
            method: "post",
            Modal {
                trigger_id: "tool-modal",
                ModalBody {
                    h3 {
                        class: "font-bold text-lg mb-4",
                        "Enable/Disable Tools"
                    }
                    div {
                        class: "form-control",
                        for tool in &available_tools {
                            div {
                                class: "flex items-center mb-2",
                                input {
                                    r#type: "checkbox",
                                    name: "tools",
                                    value: "{tool.name}",
                                    class: "checkbox checkbox-primary mr-2",
                                    checked: enabled_tools.contains(&tool.name),
                                }
                                label {
                                    class: "cursor-pointer label",
                                    "{tool.name}"
                                }
                            }
                        }
                    }

                    ModalAction {
                        Button {
                            class: "cancel-modal",
                            button_scheme: ButtonScheme::Warning,
                            button_size: ButtonSize::Small,
                            "Cancel"
                        }
                        Button {
                            button_type: ButtonType::Submit,
                            button_scheme: ButtonScheme::Primary,
                            "Save"
                        }
                    }
                }
            }
        }
    )
}
