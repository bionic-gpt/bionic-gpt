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

#[derive(Clone, Debug, PartialEq)]
pub struct PromptSizePreview {
    pub default_prompt_tokens: i32,
    pub runtime_additions_tokens: i32,
    pub combined_system_message_tokens: i32,
    pub tool_metadata_tokens: i32,
    pub total_foundation_tokens: i32,
    pub tool_count: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SystemPromptPageData {
    pub setting: RuntimeSetting,
    pub runtime_additions: Option<String>,
    pub prompt_size_preview: PromptSizePreview,
    pub tools_preview: Vec<ToolPreview>,
    pub integration_functions_preview: Vec<IntegrationFunctionPreview>,
    pub vfs_preview: String,
}

pub fn page(team_id: String, rbac: Rbac, data: SystemPromptPageData) -> String {
    let SystemPromptPageData {
        setting,
        runtime_additions,
        prompt_size_preview,
        tools_preview,
        integration_functions_preview,
        vfs_preview,
    } = data;

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
                PromptSizeCard {
                    preview: prompt_size_preview
                }
                ToolDefinitionCards {
                    title: "Tools".to_string(),
                    description: "These tool definitions are sent as model tool metadata. They are not appended to the system prompt.".to_string(),
                    tools: tools_preview
                }
                DiscoverableSkills {
                    runtime_additions
                }
                ToolCards {
                    title: "Integration functions".to_string(),
                    description: "These functions are discoverable through search_tool_functions and callable from run_python through toolbox.integrations.".to_string(),
                    functions: integration_functions_preview
                }
                DebugPreview {
                    title: "Virtual filesystem".to_string(),
                    description: "This is the Bashkit VFS layout. Datasets are prompt scoped; attachments and outputs are conversation scoped.".to_string(),
                    body: vfs_preview
                }
            }
        }
    };
    crate::render(page)
}

#[component]
fn PromptSizeCard(preview: PromptSizePreview) -> Element {
    rsx!(
        Card {
            CardHeader { title: "Prompt size" }
            CardBody {
                class: "text-sm text-base-content/80",
                p {
                    "Estimated token usage for the stable request foundation before conversation history, uploaded file contents, generated outputs, or retrieved document chunks."
                }
                div {
                    class: "mt-4 grid grid-cols-1 gap-3 md:grid-cols-2",
                    PromptSizeMetric {
                        label: "Default prompt".to_string(),
                        value: preview.default_prompt_tokens,
                        help: "The editable default runtime system prompt.".to_string()
                    }
                    PromptSizeMetric {
                        label: "Discoverable skills".to_string(),
                        value: preview.runtime_additions_tokens,
                        help: "A compact skills catalogue appended after the default prompt.".to_string()
                    }
                    PromptSizeMetric {
                        label: "Combined system message".to_string(),
                        value: preview.combined_system_message_tokens,
                        help: "The text sent as the system message.".to_string()
                    }
                    PromptSizeMetric {
                        label: "Tool metadata".to_string(),
                        value: preview.tool_metadata_tokens,
                        help: format!("{} tool definitions sent separately from the system message.", preview.tool_count)
                    }
                }
                div {
                    class: "mt-4 rounded border border-base-300 bg-base-100 p-4",
                    div {
                        class: "flex flex-wrap items-center justify-between gap-3",
                        div {
                            p { class: "font-semibold text-base-content", "Total request foundation" }
                            p { class: "text-xs text-base-content/60", "Combined system message plus model tool metadata." }
                        }
                        div {
                            class: "text-right",
                            p { class: "text-2xl font-semibold text-base-content", "{preview.total_foundation_tokens}" }
                            p { class: "text-xs text-base-content/60", "estimated tokens" }
                        }
                    }
                }
                p {
                    class: "mt-3 text-xs text-base-content/60",
                    "Tools, integrations, and skills are discoverable without dumping every instruction or integration function into the system prompt, so the foundation stays relatively stable as capabilities grow."
                }
            }
        }
    )
}

#[component]
fn PromptSizeMetric(label: String, value: i32, help: String) -> Element {
    rsx!(
        div {
            class: "rounded border border-base-300 bg-base-100 p-4",
            div {
                class: "flex items-start justify-between gap-3",
                div {
                    p { class: "font-semibold text-base-content", "{label}" }
                    p { class: "mt-1 text-xs text-base-content/60", "{help}" }
                }
                div {
                    class: "shrink-0 text-right",
                    p { class: "font-mono text-lg font-semibold text-base-content", "{value}" }
                    p { class: "text-xs text-base-content/60", "tokens" }
                }
            }
        }
    )
}

#[component]
fn DiscoverableSkills(runtime_additions: Option<String>) -> Element {
    rsx!(
        Card {
            CardHeader { title: "Discoverable skills" }
            CardBody {
                class: "text-sm text-base-content/80",
                p { "Bionic takes the currently visible skills and appends a compact name: description catalogue to the prompt. The model can use this catalogue to discover relevant skills, then read the full SKILL.md instructions from /home/user/skills when needed." }
                if let Some(runtime_additions) = runtime_additions.as_ref() {
                    pre {
                        class: "mt-3 max-h-80 overflow-auto whitespace-pre-wrap rounded border border-base-300 bg-base-100 p-4 font-mono text-xs text-base-content",
                        "{runtime_additions}"
                    }
                } else {
                    EmptyPreview { message: "No discoverable skills are currently visible.".to_string() }
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
