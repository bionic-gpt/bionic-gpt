#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use db::authz::Rbac;
use dioxus::prelude::*;
use oas3::spec::{ObjectOrReference, Parameter, ParameterIn, PathItem};

use super::IntegrationOas3;

#[derive(Debug, Clone)]
struct ParameterInfo {
    name: String,
    location: String,
    description: Option<String>,
    required: bool,
    source: ParameterSource,
}

#[derive(Debug, Clone)]
enum ParameterSource {
    PathItem,
    Operation(String),
}

fn extract_summaries_and_descriptions(item: &PathItem) -> Vec<(String, String, String)> {
    [&item.get, &item.put, &item.post, &item.delete]
        .iter()
        .filter_map(|op| op.as_ref())
        .map(|op| {
            let summary = op.summary.clone().unwrap_or_default();
            let description = op.description.clone().unwrap_or_default();
            let operationId = op.operation_id.clone().unwrap_or_default();
            (summary, description, operationId)
        })
        .collect()
}

fn resolve_parameter(param_ref: &ObjectOrReference<Parameter>) -> Option<Parameter> {
    match param_ref {
        ObjectOrReference::Object(param) => Some(param.clone()),
        ObjectOrReference::Ref { .. } => {
            // For now, skip references as they require spec resolution
            // Could be enhanced later to resolve references
            None
        }
    }
}

fn parameter_location_to_string(location: &ParameterIn) -> String {
    match location {
        ParameterIn::Path => "path".to_string(),
        ParameterIn::Query => "query".to_string(),
        ParameterIn::Header => "header".to_string(),
        ParameterIn::Cookie => "cookie".to_string(),
    }
}

fn extract_parameters_info(_path: &str, item: &PathItem) -> Vec<ParameterInfo> {
    let mut parameters = Vec::new();

    // Extract PathItem-level parameters
    for param_ref in &item.parameters {
        if let Some(param) = resolve_parameter(param_ref) {
            parameters.push(ParameterInfo {
                name: param.name,
                location: parameter_location_to_string(&param.location),
                description: param.description,
                required: param.required.unwrap_or(false),
                source: ParameterSource::PathItem,
            });
        }
    }

    // Extract Operation-level parameters for each HTTP method
    let operations = [
        ("GET", &item.get),
        ("PUT", &item.put),
        ("POST", &item.post),
        ("DELETE", &item.delete),
        ("OPTIONS", &item.options),
        ("HEAD", &item.head),
        ("PATCH", &item.patch),
        ("TRACE", &item.trace),
    ];

    for (method, operation_opt) in operations {
        if let Some(operation) = operation_opt {
            for param_ref in &operation.parameters {
                if let Some(param) = resolve_parameter(param_ref) {
                    parameters.push(ParameterInfo {
                        name: param.name,
                        location: parameter_location_to_string(&param.location),
                        description: param.description,
                        required: param.required.unwrap_or(false),
                        source: ParameterSource::Operation(method.to_string()),
                    });
                }
            }
        }
    }

    parameters
}

pub fn view(team_id: i32, rbac: Rbac, integration: Option<IntegrationOas3>) -> String {
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

            if let Some(integration) = integration {
                div {
                    class: "flex",
                    img {
                        class: "border rounded p-2",
                        src: super::get_logo_url(&integration.spec.info.extensions),
                        width: "48",
                        height: "48"
                    }
                    div {
                        class: "ml-4",
                        h2 {
                            class: "text-xl font-semibold",
                            "{integration.spec.info.title.clone()}"
                        }
                        p {
                            if let Some(description) = integration.spec.info.description.clone() {
                                "{description}"
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
                if let Some(map) = integration.spec.paths {
                    for (path, item) in map {
                        details { class: "card mt-5",
                            summary {
                                class: "cursor-pointer px-4 py-3 flex items-center justify-between",
                                div {
                                    class: "flex",
                                    div {
                                        class: "",
                                        img {
                                            class: "border rounded p-1",
                                            src: super::get_logo_url(&integration.spec.info.extensions),
                                            width: "24",
                                            height: "24"
                                        }
                                    }
                                    div {
                                        class: "ml-4",
                                        for (summary, description, operationId) in extract_summaries_and_descriptions(&item) {
                                            h2 {
                                                class: "font-semibold",
                                                "{operationId}"
                                            }
                                            p {
                                                "{description}{summary}"
                                            }
                                        }
                                    }
                                }
                            }

                            // Parameter display content
                            {
                                let parameters = extract_parameters_info(&path, &item);
                                if parameters.is_empty() {
                                    rsx! {
                                        div {
                                            class: "p-4",
                                            p {
                                                class: "text-gray-500 italic",
                                                "No parameters required"
                                            }
                                        }
                                    }
                                } else {
                                    // Group parameters by source
                                    let path_params: Vec<_> = parameters.iter()
                                        .filter(|p| matches!(p.source, ParameterSource::PathItem))
                                        .collect();

                                    let mut operation_params: std::collections::HashMap<String, Vec<_>> = std::collections::HashMap::new();
                                    for param in parameters.iter() {
                                        if let ParameterSource::Operation(method) = &param.source {
                                            operation_params.entry(method.clone()).or_insert_with(Vec::new).push(param);
                                        }
                                    }

                                    rsx! {
                                        div {
                                            class: "p-4",

                                            // PathItem-level parameters section
                                            if !path_params.is_empty() {
                                                div {
                                                    class: "mb-4",
                                                    h4 {
                                                        class: "font-semibold text-sm text-gray-700 mb-2",
                                                        "Path Parameters"
                                                    }
                                                    div {
                                                        class: "space-y-2",
                                                        for param in path_params {
                                                            div {
                                                                class: "border-l-2 border-blue-200 pl-3 py-1",
                                                                div {
                                                                    class: "flex items-center gap-2",
                                                                    span {
                                                                        class: "font-mono text-sm",
                                                                        "{param.name}"
                                                                        if param.required {
                                                                            span { class: "text-red-500", "*" }
                                                                        }
                                                                    }
                                                                    span {
                                                                        class: "px-2 py-1 text-xs rounded bg-blue-100 text-blue-700",
                                                                        "{param.location}"
                                                                    }
                                                                }
                                                                if let Some(description) = &param.description {
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

                                            // Operation-level parameters sections
                                            for (method, method_params) in operation_params {
                                                if !method_params.is_empty() {
                                                    div {
                                                        class: "mb-4",
                                                        h4 {
                                                            class: "font-semibold text-sm text-gray-700 mb-2",
                                                            "{method} Parameters"
                                                        }
                                                        div {
                                                            class: "space-y-2",
                                                            for param in method_params {
                                                                div {
                                                                    class: "border-l-2 border-green-200 pl-3 py-1",
                                                                    div {
                                                                        class: "flex items-center gap-2",
                                                                        span {
                                                                            class: "font-mono text-sm",
                                                                            "{param.name}"
                                                                            if param.required {
                                                                                span { class: "text-red-500", "*" }
                                                                            }
                                                                        }
                                                                        span {
                                                                            class: "px-2 py-1 text-xs rounded bg-green-100 text-green-700",
                                                                            "{param.location}"
                                                                        }
                                                                    }
                                                                    if let Some(description) = &param.description {
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
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    crate::render(page)
}
