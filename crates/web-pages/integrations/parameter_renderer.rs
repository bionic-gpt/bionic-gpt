#![allow(non_snake_case)]
use dioxus::prelude::*;

/// Render a parameter with support for nested objects and enhanced formatting
pub fn render_parameter(
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
