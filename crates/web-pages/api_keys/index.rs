#![allow(non_snake_case)]
use crate::{
    app_layout::{Layout, SideBar},
    charts::{ApiRequestChartCard, TokenUsageChartCard},
    render, ConfirmModal,
};
use assets::files::*;
use daisy_rsx::*;
use db::{authz::Rbac, ApiKey, Prompt, PromptType as DBPromptType};
use dioxus::prelude::*;

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
                div {
                    class: "flex gap-4",
                    Button {
                        prefix_image_src: "{button_plus_svg.name}",
                        popover_target: "create-assistant-key",
                        button_scheme: ButtonScheme::Neutral,
                        "Create Assistant Key"
                    }
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
                }
                if !api_keys.is_empty() {
                    ApiKeysTable {
                        api_keys: api_keys.clone(),
                        team_id: team_id
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
fn ApiKeysTable(api_keys: Vec<ApiKey>, team_id: i32) -> Element {
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
