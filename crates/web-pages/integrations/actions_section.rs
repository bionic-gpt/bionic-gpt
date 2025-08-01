#![allow(non_snake_case)]
use super::parameter_renderer::render_parameter;
use daisy_rsx::*;
use dioxus::prelude::*;
use openai_api::BionicToolDefinition;

#[component]
pub fn ActionsSection(
    logo_url: Option<String>,
    tool_definitions: Vec<BionicToolDefinition>,
) -> Element {
    rsx! {
        div {
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
                                    if let Some(url) = logo_url.clone() {
                                        img {
                                            class: "border border-neutral-content  rounded p-1",
                                            src: "{url}",
                                            width: "32",
                                            height: "32"
                                        }
                                    } else {
                                        Avatar {
                                            avatar_size: AvatarSize::Medium,
                                            name: "{tool.function.name}"
                                        }
                                    }
                                }
                                div {
                                    class: "ml-4",
                                    h2 {
                                        class: "font-semibold",
                                        "{tool.function.name}"
                                    }
                                    p {
                                        "{tool.function.description}"
                                    }
                                }
                            }
                        }

                        // Enhanced parameter display content
                        {
                            // Parse the JSON schema parameters
                            if let Some(properties) = tool.function.parameters.get("properties") {
                                if let Some(properties_obj) = properties.as_object() {
                                    let required_params = tool.function.parameters.get("required")
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
}
