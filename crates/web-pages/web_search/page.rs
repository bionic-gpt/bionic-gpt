#![allow(non_snake_case)]
use crate::app_layout::{AdminLayout, SideBar};
use crate::components::card_item::CardItem;
use crate::SectionIntroduction;
use daisy_rsx::*;
use db::authz::Rbac;
use db::OpenapiSpec;
use dioxus::prelude::*;

pub fn page(
    team_id: i32,
    rbac: Rbac,
    specs: Vec<OpenapiSpec>,
    selected_spec_id: Option<i32>,
) -> String {
    let cards: Vec<Element> = specs
        .iter()
        .map(|spec| {
            let is_selected = selected_spec_id == Some(spec.id);
            let avatar_initial = spec.title.chars().next().unwrap_or('W').to_string();
            rsx!(
                CardItem {
                    class: Some("mt-0".into()),
                    title: spec.title.clone(),
                    description: Some(rsx!(
                        div {
                            class: "flex flex-wrap items-center gap-2 text-xs",
                            code { "{spec.slug}" }
                            Badge {
                                badge_style: BadgeStyle::Outline,
                                badge_size: BadgeSize::Sm,
                                badge_color: if spec.is_active { BadgeColor::Success } else { BadgeColor::Neutral },
                                {if spec.is_active { "Active" } else { "Inactive" }}
                            }
                            if is_selected {
                                Badge {
                                    badge_style: BadgeStyle::Outline,
                                    badge_size: BadgeSize::Sm,
                                    badge_color: BadgeColor::Info,
                                    "Selected"
                                }
                            }
                        }
                    )),
                    footer: None,
                    image_src: None,
                    avatar_name: Some(avatar_initial),
                    count_labels: vec![],
                    action: Some(rsx!(
                        form {
                            method: "post",
                            action: crate::routes::web_search::Select { team_id, id: spec.id }.to_string(),
                            Button {
                                button_type: ButtonType::Submit,
                                button_scheme: ButtonScheme::Primary,
                                button_size: ButtonSize::Small,
                                disabled: !spec.is_active || is_selected,
                                "Select"
                            }
                        }
                    )),
                    popover_target: None,
                    clickable_link: None,
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
