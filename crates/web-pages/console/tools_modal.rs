#![allow(non_snake_case)]
use crate::routes;
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn ToolsModal(enabled_tools: Vec<String>, available_tools: Vec<(String, String)>) -> Element {
    // Get the list of available tools
    // In a real implementation, we would get this from the integrations crate
    // For now, we'll use a hardcoded list of tools
    let available_tools = if available_tools.is_empty() {
        // Fallback to hardcoded tools if none are provided
        vec![
            ("get_weather".to_string(), "Weather Tool".to_string()),
            (
                "get_current_time_and_date".to_string(),
                "Time & Date Tool".to_string(),
            ),
        ]
    } else {
        available_tools
    };

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
                        for (tool_id, tool_name) in &available_tools {
                            div {
                                class: "flex items-center mb-2",
                                input {
                                    r#type: "checkbox",
                                    name: "tools",
                                    value: "{tool_id}",
                                    class: "checkbox checkbox-primary mr-2",
                                    checked: enabled_tools.contains(tool_id),
                                }
                                label {
                                    class: "cursor-pointer label",
                                    "{tool_name}"
                                }
                            }
                        }
                    }

                    ModalAction {
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
