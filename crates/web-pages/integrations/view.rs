#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::routes;
use crate::ConfirmModal;
use assets::files::{button_edit_svg, menu_delete_svg};
use db::{authz::Rbac, Integration};
use dioxus::prelude::*;
use openai_api::BionicToolDefinition;

/// Render a parameter with support for nested objects and enhanced formatting
fn render_parameter(
    param_name: &str,
    param_schema: &serde_json::Value,
    required_params: &std::collections::HashSet<&str>,
    depth: usize,
) -> Element {
    let indent_class = match depth {
        0 => "border-l-2 border-blue-200 pl-3 py-2",
        1 => "border-l-2 border-green-200 pl-6 py-1 ml-3",
        _ => "border-l-2 border-gray-200 pl-6 py-1 ml-6",
    };

    let param_type = param_schema
        .get("type")
        .and_then(|t| t.as_str())
        .unwrap_or("unknown");
    let format = param_schema.get("format").and_then(|f| f.as_str());
    let description = param_schema.get("description").and_then(|d| d.as_str());
    let example = param_schema.get("example");
    let is_required = required_params.contains(param_name);

    rsx! {
        div {
            class: "{indent_class} text-sm flex flex-col gap-0.5",

            div {
                class: "flex flex-wrap items-center gap-2",

                // Name + required star
                span {
                    class: "font-mono font-medium",
                    "{param_name}"
                    if is_required {
                        span { class: "text-red-500 ml-1", "*" }
                    }
                }

                // Type and format
                span {
                    class: "bg-blue-100 text-blue-700 text-xs px-2 py-0.5 rounded",
                    "{param_type}"
                    if let Some(fmt) = format {
                        span { class: "text-blue-500", ", {fmt}" }
                    }
                }

                // Required/Optional badge
                span {
                    class: if is_required { "bg-red-100 text-red-700" } else { "bg-gray-100 text-gray-700" },
                    class: "text-xs px-2 py-0.5 rounded",
                    if is_required { "required" } else { "optional" }
                }

                // Example value
                if let Some(ex) = example {
                    span {
                        class: "text-xs text-gray-500",
                        "Example: "
                    }
                    code {
                        class: "bg-gray-100 px-1 py-0.5 rounded text-xs",
                        "{ex}"
                    }
                }
            }

            // Optional: description below in smaller font
            if let Some(desc) = description {
                p {
                    class: "text-xs text-gray-500 ml-1",
                    "{desc}"
                }
            }
        }
    }
}

pub fn view(
    team_id: i32,
    rbac: Rbac,
    integration: Integration,
    logo_url: String,
    description: String,
    tool_definitions: Vec<BionicToolDefinition>,
) -> String {
    let modal_trigger = format!("delete-integration-{}", integration.id);
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Integrations,
            team_id: team_id,
            rbac: rbac,
            title: "Integrations",
            header: rsx!(
                h3 { "Integration" }
            ),

            div {
                class: "p-4 max-w-3xl w-full mx-auto",
                div {
                    class: "flex justify-between",
                    div {
                        class: "flex items-center",
                        img {
                            class: "w-12 h-12 object-contain border border-neutral-content rounded p-2",
                            src: "{logo_url}",
                            width: "48",
                            height: "48"
                        }
                        div {
                            class: "ml-4",
                            h2 {
                                class: "text-xl font-semibold",
                                "{integration.name.clone()}"
                            }
                            p {
                                class: "text-sm text-gray-700 whitespace-pre-wrap break-words mt-1",
                                "{description}"
                            }
                        }
                    }
                    div {
                        class: "flex flex-col justify-center",
                        div {
                            class: "flex gap-4",
                            crate::button::Button {
                                button_type: crate::button::ButtonType::Link,
                                prefix_image_src: "{button_edit_svg.name}",
                                href: routes::integrations::Edit{team_id, id: integration.id}.to_string(),
                                button_scheme: crate::button::ButtonScheme::Outline,
                                "Edit"
                            }
                            crate::button::Button {
                                prefix_image_src: "{menu_delete_svg.name}",
                                modal_trigger: modal_trigger.clone(),
                                button_scheme: crate::button::ButtonScheme::Danger
                            }
                            ConfirmModal {
                                action: crate::routes::integrations::Delete{team_id, id: integration.id}.to_string(),
                                trigger_id: modal_trigger,
                                submit_label: "Delete".to_string(),
                                heading: "Delete this Integration?".to_string(),
                                warning: "Are you sure you want to delete this Integration?".to_string(),
                                hidden_fields: vec![
                                    ("team_id".into(), team_id.to_string()),
                                    ("id".into(), integration.id.to_string()),
                                ],
                            }
                        }
                    }
                }
                hr {
                    class: "mt-5 mb-5"
                }
                h2 {
                    class: "font-semibold",
                    "Actions"
                }

                if !tool_definitions.is_empty() {
                    for tool in tool_definitions {
                        details { class: "card mt-5 text-sm",
                            summary {
                                class: "cursor-pointer px-4 py-3 flex items-center justify-between",
                                div {
                                    class: "flex",
                                    div {
                                        class: "flex flex-col justify-center",
                                        img {
                                            class: "border border-neutral-content  rounded p-1",
                                            src: "{logo_url}",
                                            width: "32",
                                            height: "32"
                                        }
                                    }
                                    div {
                                        class: "ml-4",
                                        h2 {
                                            class: "font-semibold",
                                            "{tool.function.name}"
                                        }
                                        p {
                                            if let Some(description) = &tool.function.description {
                                                "{description}"
                                            }
                                        }
                                    }
                                }
                            }

                            // Enhanced parameter display content
                            {
                                if let Some(parameters) = &tool.function.parameters {
                                    // Parse the JSON schema parameters
                                    if let Some(properties) = parameters.get("properties") {
                                        if let Some(properties_obj) = properties.as_object() {
                                            let required_params = parameters.get("required")
                                                .and_then(|r| r.as_array())
                                                .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<std::collections::HashSet<_>>())
                                                .unwrap_or_default();

                                            rsx! {
                                                div {
                                                    class: "p-4",
                                                    h4 {
                                                        class: "font-semibold text-sm text-gray-700 mb-3",
                                                        "API Parameters"
                                                    }
                                                    div {
                                                        class: "space-y-4",
                                                        for (param_name, param_schema) in properties_obj {
                                                            {render_parameter(param_name, param_schema, &required_params, 0)}
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            rsx! {
                                                div {
                                                    class: "p-4",
                                                    p {
                                                        class: "text-gray-500 italic",
                                                        "No parameters required"
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        rsx! {
                                            div {
                                                class: "p-4",
                                                p {
                                                    class: "text-gray-500 italic",
                                                    "No parameters required"
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    rsx! {
                                        div {
                                            class: "p-4",
                                            p {
                                                class: "text-gray-500 italic",
                                                "No parameters required"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    div {
                        class: "p-4",
                        p {
                            class: "text-gray-500 italic",
                            "No tools found in this integration"
                        }
                    }
                }
            }
        }
    };

    crate::render(page)
}
