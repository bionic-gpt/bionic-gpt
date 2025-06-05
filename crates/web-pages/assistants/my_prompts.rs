#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::hero::Hero;
use crate::routes;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::prompts::Prompt;
use dioxus::prelude::*;

pub fn page(team_id: i32, rbac: Rbac, prompts: Vec<Prompt>) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Prompts,
            team_id: team_id,
            rbac: rbac.clone(),
            title: "My Assistants",
            header: rsx!(
                h3 {
                    span {
                        class: "hidden md:block",
                        "My Assistants"
                    }
                }
                div {
                    a {
                        href: crate::routes::prompts::Index{team_id}.to_string(),
                        class: "btn btn-ghost btn-sm font-bold! mr-4",
                        "Explore Assistants"
                    }
                    crate::button::Button {
                        button_type: crate::button::ButtonType::Link,
                        prefix_image_src: "{button_plus_svg.name}",
                        href: routes::prompts::New{team_id}.to_string(),
                        button_scheme: crate::button::ButtonScheme::Primary,
                        "New Assistant"
                    }
                }
            ),

            Hero {
                heading: "Your Assistants".to_string(),
                subheading: "Discover and create custom chat bots that combine instructions,
                    extra knowledge, and any combination of skills.".to_string()
            }

            Card {
                class: "has-data-table max-w-5xl mx-auto",
                CardHeader {
                    title: "My Assistants"
                }
                CardBody {
                    table {
                        class: "table table-sm table-layout-fixed",
                        thead {
                            th {
                                class: "hidden md:block",
                                "Last Updated"
                            }
                            th {
                                class: "w-full",
                                "Name"
                            }
                            th { "Visibility" }
                            th {
                                "Edit"
                            }
                            th {
                                class: "hidden md:block",
                                class: "text-right",
                                "Action"
                            }
                        }
                        tbody {
                            for prompt in &prompts {
                                tr {
                                    td {
                                        class: "hidden md:block",
                                        RelativeTime {
                                            format: RelativeTimeFormat::Relative,
                                            datetime: "{prompt.updated_at}"
                                        }
                                    }
                                    td {
                                        strong {
                                            "{prompt.name}"
                                        }
                                    }
                                    td {
                                        super::visibility::VisLabel {
                                            visibility: prompt.visibility
                                        }
                                    }
                                    td {
                                        crate::button::Button {
                                            button_type: crate::button::ButtonType::Link,
                                            button_scheme: crate::button::ButtonScheme::Default,
                                            href: routes::prompts::Edit{team_id, prompt_id: prompt.id}.to_string(),
                                            "Edit"
                                        }

                                    }
                                    td {
                                        class: "text-right hidden md:block",
                                        DropDown {
                                            direction: Direction::Left,
                                            button_text: "...",
                                            DropDownLink {
                                                drawer_trigger: format!("delete-trigger-{}-{}",
                                                    prompt.id, team_id),
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


            for item in &prompts {
                super::delete::DeleteDrawer {
                    team_id: team_id,
                    id: item.id,
                    trigger_id: format!("delete-trigger-{}-{}", item.id, team_id)
                }
            }




        }
    };

    crate::render(page)
}

// Comma separated dataset to vec of i32
fn _split_datasets(datasets: &str) -> Vec<i32> {
    let ids: Vec<i32> = datasets
        .split(',')
        .map(|dataset_id| dataset_id.parse::<i32>().unwrap_or(-1))
        .filter(|x| x != &-1)
        .collect();
    ids
}
