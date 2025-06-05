#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use db::{authz::Rbac, Integration};
use dioxus::prelude::*;
use openai_api::BionicToolDefinition;

pub fn view(
    team_id: i32,
    rbac: Rbac,
    integration: Integration,
    logo_url: String,
    description: String,
    tool_definitions: Vec<BionicToolDefinition>,
) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4 max-w-3xl w-full mx-auto",
            selected_item: SideBar::Integrations,
            team_id: team_id,
            rbac: rbac,
            title: "Integrations",
            header: rsx!(
                h3 { "Integration" }
            ),

            div {
                class: "flex",
                img {
                    class: "border border-neutral-content rounded p-2",
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
                        "{description}"
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

                        // Parameter display content
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
                                                    class: "font-semibold text-sm text-gray-700 mb-2",
                                                    "Parameters"
                                                }
                                                div {
                                                    class: "space-y-2",
                                                    for (param_name, param_schema) in properties_obj {
                                                        div {
                                                            class: "border-l-2 border-blue-200 pl-3 py-1",
                                                            div {
                                                                class: "flex items-center gap-2",
                                                                span {
                                                                    class: "font-mono text-sm",
                                                                    "{param_name}"
                                                                    if required_params.contains(param_name.as_str()) {
                                                                        span { class: "text-red-500", "*" }
                                                                    }
                                                                }
                                                                if let Some(param_type) = param_schema.get("type").and_then(|t| t.as_str()) {
                                                                    span {
                                                                        class: "px-2 py-1 text-xs rounded bg-blue-100 text-blue-700",
                                                                        "{param_type}"
                                                                    }
                                                                }
                                                            }
                                                            if let Some(description) = param_schema.get("description").and_then(|d| d.as_str()) {
                                                                p {
                                                                    class: "text-sm text-gray-600 mt-1",
                                                                    "{description}"
                                                                }
                                                            }
                                                        }
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
    };

    crate::render(page)
}
