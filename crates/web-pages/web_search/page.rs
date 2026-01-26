#![allow(non_snake_case)]
use crate::app_layout::{AdminLayout, SideBar};
use crate::components::card_item::CardItem;
use crate::shared::openapi_spec_api_keys::{OpenapiSpecApiKeyForm, OpenapiSpecKeySummary};
use crate::SectionIntroduction;
use daisy_rsx::*;
use db::authz::Rbac;
use dioxus::prelude::*;
use integrations::bionic_openapi::BionicOpenAPI;

pub fn page(
    team_id: String,
    rbac: Rbac,
    specs: Vec<OpenapiSpecKeySummary>,
    selected_spec_id: Option<i32>,
) -> String {
    let cards: Vec<Element> = specs
        .iter()
        .map(|summary| {
            let spec = &summary.spec;
            let is_selected = selected_spec_id == Some(spec.id);
            let avatar_initial = spec.title.chars().next().unwrap_or('W').to_string();
            let api_key_trigger = format!("web-search-api-key-{}", spec.id);
            let logo_url = BionicOpenAPI::new(&spec.spec)
                .ok()
                .and_then(|openapi| openapi.get_logo_url())
                .or(spec.logo_url.clone());
            rsx!(
                div {
                    CardItem {
                        class: Some("mt-0".into()),
                        title: spec.title.clone(),
                        description: Some(rsx!(
                            div {
                                class: "flex flex-wrap items-center gap-2 text-xs",
                                code { "{spec.slug}" }
                                if is_selected {
                                    Badge {
                                        badge_style: BadgeStyle::Outline,
                                        badge_size: BadgeSize::Sm,
                                        badge_color: BadgeColor::Info,
                                        "Selected"
                                    }
                                }
                                if summary.has_api_key {
                                    Badge {
                                        badge_style: BadgeStyle::Outline,
                                        badge_size: BadgeSize::Sm,
                                        badge_color: BadgeColor::Accent,
                                        "API Key"
                                    }
                                    Badge {
                                        badge_style: BadgeStyle::Outline,
                                        badge_size: BadgeSize::Sm,
                                        badge_color: if summary.has_key_configured { BadgeColor::Success } else { BadgeColor::Warning },
                                        {if summary.has_key_configured { "Configured" } else { "Missing Key" }}
                                    }
                                }
                            }
                        )),
                        footer: None,
                        image_src: logo_url.clone(),
                        avatar_name: if logo_url.is_some() {
                            None
                        } else {
                            Some(avatar_initial)
                        },
                        count_labels: vec![],
                        action: Some(rsx!(
                            div {
                                class: "flex flex-row gap-2 items-center",
                                if summary.has_api_key && summary.has_key_configured {
                                    form {
                                        method: "post",
                                        action: crate::routes::web_search::Select { team_id: team_id.clone(), id: spec.id }.to_string(),
                                        Button {
                                            button_type: ButtonType::Submit,
                                            button_scheme: ButtonScheme::Primary,
                                            button_size: ButtonSize::Small,
                                            disabled: !spec.is_active || is_selected,
                                            "Select"
                                        }
                                    }
                                    form {
                                        method: "post",
                                        action: crate::routes::web_search::DeleteApiKey { team_id: team_id.clone(), id: spec.id }.to_string(),
                                        Button {
                                            button_type: ButtonType::Submit,
                                            button_scheme: ButtonScheme::Secondary,
                                            button_size: ButtonSize::Small,
                                            "Delete Key"
                                        }
                                    }
                                } else if summary.has_api_key {
                                    Button {
                                        button_scheme: ButtonScheme::Secondary,
                                        button_size: ButtonSize::Small,
                                        popover_target: api_key_trigger.clone(),
                                        "Configure Key"
                                    }
                                }
                            }
                        )),
                        popover_target: None,
                        clickable_link: None,
                    }
                    if summary.has_api_key {
                        OpenapiSpecApiKeyForm {
                            trigger_id: api_key_trigger,
                            action: crate::routes::web_search::ConfigureApiKey { team_id: team_id.clone(), id: spec.id }.to_string(),
                            spec_title: spec.title.clone(),
                        }
                    }
                }
            )
        })
        .collect();

    let page = rsx! {
        AdminLayout {
            section_class: "p-4",
            selected_item: SideBar::WebSearch,
            team_id,
            title: "Web Search",
            rbac: rbac.clone(),
            header: rsx!(
                Breadcrumb {
                    items: vec![BreadcrumbItem {
                        text: "Web Search".into(),
                        href: None,
                    }]
                }
            ),
            div {
                class: "p-4 max-w-3xl w-full mx-auto",
                SectionIntroduction {
                    header: "Web Search".to_string(),
                    subtitle: "Pick the OpenAPI spec used for web search tooling.".to_string(),
                    is_empty: specs.is_empty(),
                    empty_text: "No Web Search specs available yet.".to_string(),
                }

                if !specs.is_empty() {
                    div {
                        class: "space-y-2",
                        for card in cards {
                            {card}
                        }
                    }
                }
            }
        }
    };

    crate::render(page)
}
