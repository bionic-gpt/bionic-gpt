#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use db::{authz::Rbac, Integration};
use dioxus::prelude::*;
use integrations::open_api_v3::{OpenApiOperation, ParameterSource};

pub fn view(
    team_id: i32,
    rbac: Rbac,
    integration: Option<Integration>,
    operations: Vec<OpenApiOperation>,
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

            if let Some(integration) = integration {
                div {
                    class: "flex",
                    img {
                        class: "border rounded p-2",
                        src: super::get_logo_url(&std::collections::BTreeMap::new()),
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
                            "Integration Type: {integration.integration_type:?}"
                        }
                        p {
                            "Status: {integration.integration_status:?}"
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

                if !operations.is_empty() {
                    for operation in operations {
                        details { class: "card mt-5",
                            summary {
                                class: "cursor-pointer px-4 py-3 flex items-center justify-between",
                                div {
                                    class: "flex",
                                    div {
                                        class: "",
                                        img {
                                            class: "border rounded p-1",
                                            src: super::get_logo_url(&std::collections::BTreeMap::new()),
                                            width: "24",
                                            height: "24"
                                        }
                                    }
                                    div {
                                        class: "ml-4",
                                        h2 {
                                            class: "font-semibold",
                                            if let Some(operation_id) = &operation.operation_id {
                                                "{operation_id}"
                                            } else {
                                                "{operation.definition.function.name}"
                                            }
                                        }
                                        p {
                                            if let Some(description) = &operation.description {
                                                "{description}"
                                            } else if let Some(summary) = &operation.summary {
                                                "{summary}"
                                            } else if let Some(desc) = &operation.definition.function.description {
                                                "{desc}"
                                            }
                                        }
                                        p {
                                            class: "text-sm text-gray-500",
                                            "{operation.method.to_uppercase()} {operation.path}"
                                        }
                                    }
                                }
                            }

                            // Parameter display content
                            {
                                let parameters = &operation.parameters;
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
                } else {
                    div {
                        class: "p-4",
                        p {
                            class: "text-gray-500 italic",
                            "No operations found in this integration"
                        }
                    }
                }
            }
        }
    };

    crate::render(page)
}
