#![allow(non_snake_case)]
use crate::app_layout::{AdminLayout, SideBar};
use crate::components::card_item::{CardItem, CountLabel};
use crate::SectionIntroduction;
use daisy_rsx::*;
use db::authz::Rbac;
use db::Provider;
use db::Visibility;
use dioxus::prelude::*;

const DEFAULT_DISCLAIMER: &str = "AI can make mistakes. Check important information.";

pub fn page(team_id: String, rbac: Rbac, setup_required: bool, providers: Vec<Provider>) -> String {
    let default_visibility = if rbac.is_sys_admin {
        Visibility::Company
    } else {
        Visibility::Team
    };

    let page = rsx! {
        AdminLayout {
            section_class: "p-4",
            selected_item: SideBar::Models,
            team_id: team_id.clone(),
            rbac: rbac.clone(),
            setup_required: setup_required,
            title: "Models",
            header: rsx!(
                Breadcrumb {
                    items: vec![
                        BreadcrumbItem {
                            text: "Models".into(),
                            href: Some(crate::routes::models::Index { team_id: team_id.clone() }.to_string())
                        },
                        BreadcrumbItem {
                            text: "Select Provider".into(),
                            href: None
                        }
                    ]
                }
            ),

            div {
                class: "p-4 max-w-3xl w-full mx-auto",
                SectionIntroduction {
                    header: "Choose a provider".to_string(),
                    subtitle: "Select a provider to create a model using its default settings.".to_string(),
                    is_empty: providers.is_empty(),
                    empty_text: "No providers configured yet. Add a provider to get started.".to_string(),
                }

                if !providers.is_empty() {
                    div {
                        class: "space-y-2",
                        for provider in &providers {
                            CardItem {
                                class: Some("cursor-pointer hover:bg-base-200 w-full".into()),
                                popover_target: Some(format!("new-model-provider-{}", provider.id)),
                                image_html: Some(provider.svg_logo.clone()),
                                avatar_name: None,
                                title: provider.name.clone(),
                                description: Some(rsx!(div {
                                    class: "flex flex-col gap-1 text-xs",
                                    span { class: "truncate", "{provider.base_url}" }
                                    if let Some(default_name) = provider.default_model_display_name.clone().or(provider.default_model_name.clone()) {
                                        span { class: "truncate", "Default model: {default_name}" }
                                    } else {
                                        span { class: "truncate", "Default model: Not set" }
                                    }
                                })),
                                footer: None,
                                image_src: None,
                                count_labels: vec![
                                    CountLabel {
                                        count: provider.default_model_context_size as usize,
                                        label: "Context".into()
                                    }
                                ],
                                action: None,
                                clickable_link: None,
                            }
                        }
                    }
                }

                CardItem {
                    class: Some("cursor-pointer hover:bg-base-200 w-full".into()),
                    clickable_link: Some(crate::routes::models::New { team_id: team_id.clone() }.to_string()),
                    image_src: None,
                    image_html: None,
                    avatar_name: Some("C".to_string()),
                    title: "Custom Provider / Model".to_string(),
                    description: Some(rsx!(span {
                        "Create a model with full control over all fields."
                    })),
                    footer: None,
                    count_labels: vec![],
                    action: None,
                    popover_target: None,
                }
            }

            for provider in &providers {
                {provider_modal(team_id.clone(), &rbac, provider, default_visibility)}
            }
        }
    };

    crate::render(page)
}

fn provider_modal(
    team_id: String,
    rbac: &Rbac,
    provider: &Provider,
    default_visibility: Visibility,
) -> Element {
    let default_name = provider
        .default_model_name
        .clone()
        .unwrap_or_else(|| provider.name.clone());
    let default_display_name = provider
        .default_model_display_name
        .clone()
        .or_else(|| provider.default_model_name.clone())
        .unwrap_or_else(|| provider.name.clone());
    let api_key_help = if provider.api_key_optional {
        "Optional for this provider"
    } else {
        "Required for this provider"
    };

    rsx! {
        Modal {
            trigger_id: format!("new-model-provider-{}", provider.id),
            submit_action: crate::routes::models::Upsert { team_id: team_id.clone() }.to_string(),
            ModalBody {
                class: "flex flex-col gap-4",
                h3 { class: "font-bold text-lg", "Create Model: {provider.name}" }
                p { class: "text-sm text-base-content/70", "{provider.default_model_description}" }

                input { "type": "hidden", name: "name", value: "{default_name}" }
                input { "type": "hidden", name: "display_name", value: "{default_display_name}" }
                input { "type": "hidden", name: "model_type", value: "LLM" }
                input { "type": "hidden", name: "base_url", value: "{provider.base_url}" }
                input { "type": "hidden", name: "tpm_limit", value: "10000" }
                input { "type": "hidden", name: "rpm_limit", value: "10000" }
                input { "type": "hidden", name: "context_size", value: "{provider.default_model_context_size}" }
                input { "type": "hidden", name: "description", value: "{provider.default_model_description}" }
                input { "type": "hidden", name: "disclaimer", value: "{DEFAULT_DISCLAIMER}" }
                input { "type": "hidden", name: "example1", value: "" }
                input { "type": "hidden", name: "example2", value: "" }
                input { "type": "hidden", name: "example3", value: "" }
                input { "type": "hidden", name: "example4", value: "" }
                input { "type": "hidden", name: "capability_tool_use", value: "on" }
                input { "type": "hidden", name: "provider_id", value: "{provider.id}" }

                Fieldset {
                    legend: "API Key",
                    help_text: "{api_key_help}",
                    Input {
                        input_type: InputType::Text,
                        name: "api_key",
                        required: !provider.api_key_optional
                    }
                }
                Fieldset {
                    legend: "Visibility",
                    help_text: "Who can use this model",
                    Select {
                        name: "visibility",
                        value: "{crate::visibility_to_string(default_visibility)}",
                        SelectOption {
                            value: "{crate::visibility_to_string(Visibility::Private)}",
                            selected_value: "{crate::visibility_to_string(default_visibility)}",
                            {crate::visibility_to_string(Visibility::Private)}
                        }
                        SelectOption {
                            value: "{crate::visibility_to_string(Visibility::Team)}",
                            selected_value: "{crate::visibility_to_string(default_visibility)}",
                            {crate::visibility_to_string(Visibility::Team)}
                        }
                        if rbac.is_sys_admin {
                            SelectOption {
                                value: "{crate::visibility_to_string(Visibility::Company)}",
                                selected_value: "{crate::visibility_to_string(default_visibility)}",
                                {crate::visibility_to_string(Visibility::Company)}
                            }
                        }
                    }
                }
                ModalAction {
                    Button {
                        class: "cancel-modal",
                        button_scheme: ButtonScheme::Warning,
                        "Cancel"
                    }
                    Button {
                        button_type: ButtonType::Submit,
                        button_scheme: ButtonScheme::Primary,
                        "Create Model"
                    }
                }
            }
        }
    }
}
