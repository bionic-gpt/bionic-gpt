#![allow(non_snake_case)]
use crate::{
    app_layout::{Layout, SideBar},
    render, ConfirmModal,
};
use daisy_rsx::*;
use db::{authz::Rbac, ApiKey, Prompt, PromptType as DBPromptType};
use dioxus::prelude::*;
use std::collections::HashMap;

pub fn page(
    rbac: Rbac,
    team_id: i32,
    api_keys: Vec<ApiKey>,
    assistants: Vec<Prompt>,
    models: Vec<Prompt>,
    token_usage_data: Vec<db::queries::token_usage_metrics::DailyTokenUsage>,
    api_request_data: Vec<db::queries::token_usage_metrics::DailyApiRequests>,
) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::ApiKeys,
            team_id: team_id,
            rbac: rbac,
            title: "API Keys",
            header: rsx! {
                h3 { "API Keys" }
            },
            // Add graphs section - always show regardless of API keys
            div {
                class: "grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8",

                    // Token Usage Graph Card
                    Card {
                        CardHeader {
                            title: "Token Usage (Last 7 Days)"
                        }
                        CardBody {
                            TokenUsageChart {
                                data: token_usage_data.clone()
                            }
                            div {
                                class: "flex justify-center mt-4 space-x-4",
                                div {
                                    class: "flex items-center",
                                    div {
                                        class: "w-4 h-4 bg-blue-500 mr-2"
                                    }
                                    span {
                                        class: "text-sm",
                                        "Prompt Tokens"
                                    }
                                }
                                div {
                                    class: "flex items-center",
                                    div {
                                        class: "w-4 h-4 bg-green-500 mr-2"
                                    }
                                    span {
                                        class: "text-sm",
                                        "Completion Tokens"
                                    }
                                }
                            }
                        }
                    }

                    // API Request Rate Graph Card
                    Card {
                        CardHeader {
                            title: "API Requests (Last 7 Days)"
                        }
                        CardBody {
                            ApiRequestChart {
                                data: api_request_data.clone()
                            }
                        }
                    }
                }
            }

            for item in &api_keys {
                ConfirmModal {
                    action: crate::routes::api_keys::Delete {team_id, id: item.id}.to_string(),
                    trigger_id: format!("delete-trigger-{}-{}", item.id, team_id),
                    submit_label: "Delete".to_string(),
                    heading: "Delete this API Key?".to_string(),
                    warning: "Are you sure you want to delete this api key?".to_string(),
                    hidden_fields: vec![
                        ("team_id".into(), team_id.to_string()),
                        ("id".into(), item.id.to_string()),
                    ],
                }
            }

            super::form::AssistantForm {
                team_id: team_id,
                prompts: assistants.clone()
            },
            super::form::ModelForm {
                team_id: team_id,
                prompts: models.clone()
            },

            if ! api_keys.is_empty() {
                Card {
                    class: "has-data-table",
                    CardHeader {
                        title: "API Keys"
                    }
                    CardBody {
                        table {
                            class: "table table-sm",
                            thead {
                                th { "Name" }
                                th { "Type" }
                                th { "API Key" }
                                th { "Assistant/Model" }
                                th {
                                    class: "text-right",
                                    "Action"
                                }
                            }
                            tbody {
                                for key in &api_keys {
                                    tr {
                                        td {
                                            "{key.name}"
                                        }
                                        td {
                                            PromptType {
                                                prompt_type: key.prompt_type
                                            }
                                        }
                                        td {
                                            div {
                                                class: "flex w-full",
                                                Input {
                                                    value: key.api_key.clone(),
                                                    name: "api_key",
                                                    input_type: InputType::Password
                                                }
                                                Button {
                                                    class: "api-keys-toggle-visibility",
                                                    "Show"
                                                }
                                            }
                                        }
                                        td {
                                            "{key.prompt_name}"
                                        }
                                        td {
                                            class: "text-right",
                                            DropDown {
                                                direction: Direction::Left,
                                                button_text: "...",
                                                DropDownLink {
                                                    drawer_trigger: format!("delete-trigger-{}-{}",
                                                        key.id, team_id),
                                                    href: "#",
                                                    target: "_top",
                                                    "Delete"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
            }

            KeySelector {

            }
        }
    };

    render(page)
}

#[component]
pub fn PromptType(prompt_type: DBPromptType) -> Element {
    match prompt_type {
        DBPromptType::Model => rsx!(
            Label {
                class: "mr-2 truncate",
                label_role: LabelRole::Info,
                "Model"
            }
        ),
        DBPromptType::Assistant => rsx!(
            Label {
                class: "mr-2 truncate",
                label_role: LabelRole::Highlight,
                "Assistant"
            }
        ),
    }
}

#[component]
fn KeySelector() -> Element {
    rsx! {
        div {
            class: "grid grid-cols-1 md:grid-cols-2 gap-8 mb-8 mt-8",
            // Assistant Key Card
            Card {
                CardHeader {
                    title: "Assistant Key"
                }
                CardBody {
                    class: "p-5",
                    p { "Turn any of your assistants into an API" }
                    ul { class: "list-disc list-inside mt-4",
                        li { "Access to pre-configured AI assistants" }
                        li { "Simplified integration process" }
                        li { "Ideal for specific use-cases" }
                    }
                    div { class: "card-actions justify-end mt-4",
                        Button {
                            drawer_trigger: "create-assistant-key",
                            "Create an Assistant Key"
                        }
                    }
                }
            }

            // Model Key Card
            Card {
                CardHeader {
                    title: "Model Key"
                }
                CardBody {
                    class: "p-5",
                    p { "Use existing models for your own projects" }
                    ul { class: "list-disc list-inside mt-4",
                        li { "Full control over AI model parameters" }
                        li { "Flexibility for advanced use-cases" }
                        li { "Limits will be applied to ensure fair use" }
                    }
                    div { class: "card-actions justify-end mt-4",
                        Button {
                            drawer_trigger: "create-model-key",
                            "Create a Model Key"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TokenUsageChart(data: Vec<db::queries::token_usage_metrics::DailyTokenUsage>) -> Element {
    // Process data to group by date and separate prompt/completion tokens
    let mut daily_data: HashMap<time::Date, (i64, i64)> = HashMap::new();

    for item in &data {
        let entry = daily_data.entry(item.usage_date).or_insert((0, 0));
        match item.token_type {
            db::TokenUsageType::Prompt => entry.0 += item.total_tokens,
            db::TokenUsageType::Completion => entry.1 += item.total_tokens,
        }
    }

    let mut sorted_data: Vec<_> = daily_data.into_iter().collect();
    sorted_data.sort_by_key(|&(date, _)| date);

    let max_tokens = sorted_data
        .iter()
        .map(|(_, (prompt, completion))| prompt + completion)
        .max()
        .unwrap_or(1);

    rsx! {
        div {
            class: "w-full h-64",
            svg {
                width: "100%",
                height: "100%",
                view_box: "0 0 400 200",
                // SVG chart implementation with stacked bars
                for (i, (date, (prompt_tokens, completion_tokens))) in sorted_data.iter().enumerate() {
                    g {
                        // Stacked bar for prompt tokens (bottom)
                        rect {
                            x: "{i * 50 + 20}",
                            y: "{200 - (prompt_tokens * 180 / max_tokens)}",
                            width: "40",
                            height: "{prompt_tokens * 180 / max_tokens}",
                            fill: "#3b82f6",
                            class: "hover:opacity-80 cursor-pointer"
                        }
                        // Stacked bar for completion tokens (top)
                        rect {
                            x: "{i * 50 + 20}",
                            y: "{200 - ((prompt_tokens + completion_tokens) * 180 / max_tokens)}",
                            width: "40",
                            height: "{completion_tokens * 180 / max_tokens}",
                            fill: "#10b981",
                            class: "hover:opacity-80 cursor-pointer"
                        }
                        text {
                            x: "{i * 50 + 40}",
                            y: "195",
                            text_anchor: "middle",
                            font_size: "10",
                            "{date.month()}/{date.day()}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ApiRequestChart(data: Vec<db::queries::token_usage_metrics::DailyApiRequests>) -> Element {
    let max_requests = data.iter().map(|d| d.request_count).max().unwrap_or(1);

    rsx! {
        div {
            class: "w-full h-64",
            svg {
                width: "100%",
                height: "100%",
                view_box: "0 0 400 200",
                // SVG chart implementation with simple bars
                for (i, day_data) in data.iter().enumerate() {
                    g {
                        rect {
                            x: "{i * 50 + 20}",
                            y: "{200 - (day_data.request_count * 180 / max_requests)}",
                            width: "40",
                            height: "{day_data.request_count * 180 / max_requests}",
                            fill: "#6366f1",
                            class: "hover:opacity-80 cursor-pointer"
                        }
                        text {
                            x: "{i * 50 + 40}",
                            y: "195",
                            text_anchor: "middle",
                            font_size: "10",
                            "{day_data.request_date.month()}/{day_data.request_date.day()}"
                        }
                    }
                }
            }
        }
    }
}
