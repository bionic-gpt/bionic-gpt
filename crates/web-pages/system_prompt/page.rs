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
pub struct PromptSizePreview {
    pub default_prompt_tokens: i32,
    pub runtime_additions_tokens: i32,
    pub integration_context_tokens: i32,
    pub combined_system_message_tokens: i32,
    pub tool_metadata_tokens: i32,
    pub total_foundation_tokens: i32,
    pub tool_count: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SystemPromptPageData {
    pub setting: RuntimeSetting,
    pub runtime_additions: Option<String>,
    pub integration_context: Option<String>,
    pub prompt_size_preview: PromptSizePreview,
    pub tools_preview: Vec<ToolPreview>,
    pub vfs_preview: String,
}

pub fn page(team_id: String, rbac: Rbac, data: SystemPromptPageData) -> String {
    let SystemPromptPageData {
        setting,
        runtime_additions,
        integration_context,
        prompt_size_preview,
        tools_preview,
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
                        CardHeaderWithEstimate {
                            title: "Default runtime system prompt".to_string(),
                            token_estimate: prompt_size_preview.default_prompt_tokens
                        }
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
                ToolDefinitionCards {
                    title: "Tools".to_string(),
                    description: "These tool definitions are sent as model tool metadata. They are not appended to the system prompt.".to_string(),
                    tools: tools_preview,
                    token_estimate: prompt_size_preview.tool_metadata_tokens
                }
                DiscoverableSkills {
                    runtime_additions,
                    token_estimate: prompt_size_preview.runtime_additions_tokens
                }
                DiscoverableFunctions {
                    integration_context,
                    token_estimate: prompt_size_preview.integration_context_tokens
                }
                DebugPreview {
                    title: "Virtual filesystem".to_string(),
                    description: "This is the Bashkit VFS layout. Datasets are prompt scoped; attachments and outputs are conversation scoped.".to_string(),
                    body: vfs_preview
                }
                TotalPromptSizeCard {
                    preview: prompt_size_preview
                }
            }
        }
    };
    crate::render(page)
}

#[component]
fn CardHeaderWithEstimate(title: String, token_estimate: i32) -> Element {
    rsx!(
        div {
            class: "flex flex-wrap items-center justify-between gap-3 border-b border-base-300 px-6 py-4",
            h2 { class: "card-title text-base", "{title}" }
            TokenEstimate {
                value: token_estimate,
                label: "estimated tokens".to_string()
            }
        }
    )
}

#[component]
fn TokenEstimate(value: i32, label: String) -> Element {
    rsx!(
        Badge {
            badge_style: BadgeStyle::Outline,
            "{value} {label}"
        }
    )
}

#[component]
fn TotalPromptSizeCard(preview: PromptSizePreview) -> Element {
    rsx!(
        Card {
            CardHeader { title: "Total request foundation" }
            CardBody {
                class: "text-sm text-base-content/80",
                div {
                    class: "flex flex-wrap items-center justify-between gap-3",
                    div {
                        p { class: "text-xs text-base-content/60", "Combined system message plus model tool metadata." }
                        p { class: "mt-1 text-xs text-base-content/60", "Estimated before conversation history, uploaded file contents, generated outputs, or retrieved document chunks." }
                    }
                    div {
                        class: "text-right",
                        p { class: "text-2xl font-semibold text-base-content", "{preview.total_foundation_tokens}" }
                        p { class: "text-xs text-base-content/60", "estimated tokens" }
                    }
                }
                div {
                    class: "mt-4 flex flex-wrap gap-2 text-xs text-base-content/60",
                    Badge {
                        badge_style: BadgeStyle::Outline,
                        "system message {preview.combined_system_message_tokens}"
                    }
                    Badge {
                        badge_style: BadgeStyle::Outline,
                        "tool metadata {preview.tool_metadata_tokens}"
                    }
                    Badge {
                        badge_style: BadgeStyle::Outline,
                        "{preview.tool_count} tools"
                    }
                }
            }
        }
    )
}

#[component]
fn DiscoverableSkills(runtime_additions: Option<String>, token_estimate: i32) -> Element {
    rsx!(
        Card {
            CardHeaderWithEstimate {
                title: "Discoverable skills".to_string(),
                token_estimate
            }
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
fn DiscoverableFunctions(integration_context: Option<String>, token_estimate: i32) -> Element {
    rsx!(
        Card {
            CardHeaderWithEstimate {
                title: "Discoverable functions".to_string(),
                token_estimate
            }
            CardBody {
                class: "text-sm text-base-content/80",
                p { "Connected integrations and built-in web functions are exposed through catalogue files in /home/user/functions. Use run_bash to list the directory, then cat the relevant .md file before calling an integration; it contains the exact function names, parameters, and usage examples." }
                if let Some(integration_context) = integration_context.as_ref() {
                    pre {
                        class: "mt-3 max-h-80 overflow-auto whitespace-pre-wrap rounded border border-base-300 bg-base-100 p-4 font-mono text-xs text-base-content",
                        "{integration_context}"
                    }
                } else {
                    EmptyPreview { message: "No discoverable functions are currently connected.".to_string() }
                }
            }
        }
    )
}

#[component]
fn ToolDefinitionCards(
    title: String,
    description: String,
    tools: Vec<ToolPreview>,
    token_estimate: i32,
) -> Element {
    rsx!(
        Card {
            CardHeaderWithEstimate {
                title,
                token_estimate
            }
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
