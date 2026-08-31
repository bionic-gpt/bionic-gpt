#![allow(non_snake_case)]
use crate::components::confirm_modal::ConfirmModal;
use crate::{
    app_layout::{AdminLayout, SideBar},
    charts::{ApiRequestChartCard, TokenUsageChartCard},
    render,
};
use assets::files::*;
use daisy_rsx::*;
use db::queries::models::ModelConfig;
use db::{authz::Rbac, ApiKey};
use dioxus::prelude::*;

#[derive(Clone)]
pub struct GeneratedKey {
    pub name: String,
    pub value: String,
    pub model_name: Option<String>,
}

#[allow(clippy::too_many_arguments)]
pub fn page(
    rbac: Rbac,
    team_id: String,
    api_keys: Vec<ApiKey>,
    models: Vec<ModelConfig>,
    token_usage_data: Vec<db::queries::token_usage_metrics::DailyTokenUsage>,
    api_request_data: Vec<db::queries::token_usage_metrics::DailyApiRequests>,
    generated_key: Option<GeneratedKey>,
) -> String {
    let page = rsx! {
        AdminLayout {
            section_class: "p-4",
            selected_item: SideBar::ApiKeys,
            team_id: team_id.clone(),
            rbac: rbac,
            title: "API Keys",
            header: rsx! {
                Breadcrumb {
                    items: vec![BreadcrumbItem {
                        text: "API Keys".into(),
                        href: None
                    }]
                }
                div {
                    class: "flex gap-4",
                    Button {
                        prefix_image_src: "{button_plus_svg.name}",
                        popover_target: "create-model-key",
                        button_scheme: ButtonScheme::Primary,
                        "Create Model Key"
                    }
                }
            },
            // Add graphs section - always show regardless of API keys
            div {
                if let Some(created) = generated_key.clone() {
                    Alert {
                        alert_color: AlertColor::Success,
                        class: "mb-6 flex flex-col gap-2",
                        div { class: "font-semibold", "API Key Created" }
                        if let Some(name) = created.model_name.clone() {
                            div { class: "text-sm opacity-90", "Copy and store this key for {name}. This is the only time it will be shown." }
                        } else {
                            div { class: "text-sm opacity-90", "Copy and store this key. This is the only time it will be shown." }
                        }
                        Input {
                            input_type: InputType::Text,
                            class: "w-full",
                            value: created.value.clone(),
                            readonly: true,
                            name: "generated-api-key",
                        }
                    }
                }
                div {
                    class: "grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8",

                    // Token Usage Graph Card
                    TokenUsageChartCard {
                        data: token_usage_data.clone(),
                        title: "Token Usage (Last 7 Days)".to_string()
                    }

                    // API Request Rate Graph Card
                    ApiRequestChartCard {
                        data: api_request_data.clone(),
                        title: "API Requests (Last 7 Days)".to_string()
                    }
                }
                if !api_keys.is_empty() {
                    ApiKeysTable {
                        api_keys: api_keys.clone(),
                        team_id: team_id.clone()
                    }
                }

                for item in &api_keys {
                    ConfirmModal {
                        action: crate::routes::api_keys::Delete {team_id: team_id.clone(), id: item.id}.to_string(),
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

                super::form::ModelForm {
                    team_id: team_id.clone(),
                    prompts: models.clone()
                }
            }
        }

    };

    render(page)
}

#[component]
fn ApiKeysTable(api_keys: Vec<ApiKey>, team_id: String) -> Element {
    rsx! {
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
                        th { "Key Suffix" }
                        th { "Model" }
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
                                    span { class: "font-mono text-sm", "{mask_hash(&key.api_key)}" }
                                }
                                td {
                                    if let Some(name) = key.model_name.clone() {
                                        "{name}"
                                    } else {
                                        "-"
                                    }
                                }
                                td {
                                    class: "text-right",
                                    DropDown {
                                        direction: Direction::Left,
                                        button_text: "...",
                                        DropDownLink {
                                            popover_target: format!("delete-trigger-{}-{}",
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
    }
}

fn mask_hash(hash: &str) -> String {
    if hash.is_empty() {
        return "Unknown".to_string();
    }

    let len = hash.chars().count();
    let suffix_len = if len <= 8 { len.min(4) } else { 6 };
    let suffix: String = hash
        .chars()
        .rev()
        .take(suffix_len)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    format!("••••{}", suffix)
}
