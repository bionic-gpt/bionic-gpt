#![allow(non_snake_case)]
use crate::app_layout::{AdminLayout, SideBar};
use daisy_rsx::*;
use db::authz::Rbac;
use db::RuntimeSetting;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ToolPreview {
    pub name: String,
    pub description: String,
    pub parameters: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IntegrationFunctionPreview {
    pub path: String,
    pub integration: String,
    pub operation: String,
    pub description: String,
    pub parameters: Vec<String>,
}

pub fn page(
    team_id: String,
    rbac: Rbac,
    setting: RuntimeSetting,
    runtime_additions: Option<String>,
    tools_preview: Vec<ToolPreview>,
    integration_functions_preview: Vec<IntegrationFunctionPreview>,
    vfs_preview: String,
) -> String {
    let page = rsx! {
        AdminLayout {
            section_class: "p-4",
            selected_item: SideBar::SystemPrompt,
            team_id: team_id.clone(),
            rbac,
            title: "System Prompt",
            header: rsx!(
                Breadcrumb {
                    items: vec![BreadcrumbItem { text: "System Prompt".into(), href: None }]
                }
            ),
            div {
                class: "p-4 max-w-4xl w-full mx-auto space-y-6",
                form {
                    action: crate::routes::system_prompt::Update { team_id: team_id.clone() }.to_string(),
                    method: "post",
                    class: "space-y-6",
                    Card {
                        CardHeader { title: "Default runtime system prompt" }
                        CardBody {
                            class: "flex flex-col gap-4",
                            Fieldset {
                                legend: "Prompt text",
                                help_text: "This system prompt is sent before conversation history. Runtime skill and tool context is appended automatically.",
                                TextArea {
                                    class: "mt-3 w-full font-mono text-sm",
                                    name: "value",
                                    rows: "18",
                                    required: true,
                                    "{setting.value}"
                                }
                            }
                            div {
                                class: "text-xs text-base-content/60",
                                "Last updated {setting.updated_at}"
                            }
                        }
                    }
                    div {
                        class: "flex justify-end",
                        Button {
                            button_type: ButtonType::Submit,
                            button_scheme: ButtonScheme::Primary,
                            "Save System Prompt"
                        }
                    }
                }
                RuntimeAdditions {
                    runtime_additions
                }
                DebugPreview {
                    title: "Virtual filesystem".to_string(),
                    description: "This is the Bashkit VFS layout. Datasets are prompt scoped; attachments and outputs are conversation scoped.".to_string(),
                    body: vfs_preview
                }
                ToolCards {
                    title: "Integration functions".to_string(),
                    description: "These functions are discoverable through search_tool_functions and callable from run_python through toolbox.integrations.".to_string(),
                    functions: integration_functions_preview
                }
                ToolDefinitionCards {
                    title: "Tools".to_string(),
                    description: "These tool definitions are sent as model tool metadata. They are not appended to the system prompt.".to_string(),
                    tools: tools_preview
                }
            }
        }
    };
    crate::render(page)
}

#[component]
fn RuntimeAdditions(runtime_additions: Option<String>) -> Element {
    rsx!(
        Card {
            CardHeader { title: "Runtime additions" }
            CardBody {
                class: "text-sm text-base-content/80",
                p { "This preview is generated from the current runtime state and appended after the default prompt." }
                if let Some(runtime_additions) = runtime_additions.as_ref() {
                    pre {
                        class: "mt-3 max-h-80 overflow-auto whitespace-pre-wrap rounded border border-base-300 bg-base-100 p-4 font-mono text-xs text-base-content",
                        "{runtime_additions}"
                    }
                } else {
                    EmptyPreview { message: "No runtime skill context is currently appended.".to_string() }
                }
            }
        }
    )
}

#[component]
fn ToolDefinitionCards(title: String, description: String, tools: Vec<ToolPreview>) -> Element {
    rsx!(
        Card {
            CardHeader { title: "{title}" }
            CardBody {
                class: "text-sm text-base-content/80",
                p { "{description}" }
                if tools.is_empty() {
                    EmptyPreview { message: "No tools are currently available.".to_string() }
                } else {
                    div {
                        class: "mt-3 grid grid-cols-1 gap-3",
                        for tool in tools {
                            RuntimeCard {
                                title: tool.name,
                                subtitle: "tool metadata".to_string(),
                                description: tool.description,
                                path: None,
                                parameters: tool.parameters,
                            }
                        }
                    }
                }
            }
        }
    )
}

#[component]
fn ToolCards(
    title: String,
    description: String,
    functions: Vec<IntegrationFunctionPreview>,
) -> Element {
    rsx!(
        Card {
            CardHeader { title: "{title}" }
            CardBody {
                class: "text-sm text-base-content/80",
                p { "{description}" }
                if functions.is_empty() {
                    EmptyPreview { message: "No integration functions are currently available.".to_string() }
                } else {
                    div {
                        class: "mt-3 grid grid-cols-1 gap-3",
                        for function in functions {
                            RuntimeCard {
                                title: function.operation,
                                subtitle: function.integration,
                                description: function.description,
                                path: Some(function.path),
                                parameters: function.parameters,
                            }
                        }
                    }
                }
            }
        }
    )
}

#[component]
fn RuntimeCard(
    title: String,
    subtitle: String,
    description: String,
    path: Option<String>,
    parameters: Vec<String>,
) -> Element {
    rsx!(
        div {
            class: "rounded border border-base-300 bg-base-100 p-4",
            div {
                class: "flex flex-wrap items-start justify-between gap-2",
                div {
                    class: "min-w-0",
                    p { class: "font-semibold text-base-content", "{title}" }
                    p { class: "text-xs text-base-content/60", "{subtitle}" }
                }
                Badge {
                    badge_style: BadgeStyle::Outline,
                    badge_size: BadgeSize::Sm,
                    "{parameters.len()} params"
                }
            }
            if !description.is_empty() {
                p { class: "mt-2 text-sm text-base-content/75", "{description}" }
            }
            if let Some(path) = path {
                code { class: "mt-3 block overflow-auto rounded bg-base-200 px-2 py-1 text-xs", "{path}" }
            }
            ParameterList { parameters }
        }
    )
}

#[component]
fn ParameterList(parameters: Vec<String>) -> Element {
    rsx!(
        div {
            class: "mt-3 flex flex-wrap gap-2",
            if parameters.is_empty() {
                span { class: "text-xs text-base-content/50", "No parameters" }
            } else {
                for parameter in parameters {
                    Badge {
                        badge_style: BadgeStyle::Outline,
                        badge_size: BadgeSize::Sm,
                        "{parameter}"
                    }
                }
            }
        }
    )
}

#[component]
fn EmptyPreview(message: String) -> Element {
    rsx!(
        div {
            class: "mt-3 rounded border border-dashed border-base-300 bg-base-100 p-4 text-base-content/60",
            "{message}"
        }
    )
}

#[component]
fn DebugPreview(title: String, description: String, body: String) -> Element {
    rsx!(
        Card {
            CardHeader { title: "{title}" }
            CardBody {
                class: "text-sm text-base-content/80",
                p { "{description}" }
                pre {
                    class: "mt-3 max-h-80 overflow-auto whitespace-pre-wrap rounded border border-base-300 bg-base-100 p-4 font-mono text-xs text-base-content",
                    "{body}"
                }
            }
        }
    )
}
