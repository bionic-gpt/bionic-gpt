#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::hero::Hero;
use crate::routes;
use crate::ConfirmModal;
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
                    Button {
                        button_type: ButtonType::Link,
                        prefix_image_src: "{button_plus_svg.name}",
                        href: routes::prompts::New{team_id}.to_string(),
                        button_scheme: ButtonScheme::Primary,
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
                                        Button {
                                            button_type: ButtonType::Link,
                                            button_scheme: ButtonScheme::Neutral,
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
                                                popover_target: format!("delete-trigger-{}-{}",
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
                ConfirmModal {
                    action: crate::routes::prompts::Delete{team_id, id: item.id}.to_string(),
                    trigger_id: format!("delete-trigger-{}-{}", item.id, team_id),
                    submit_label: "Delete".to_string(),
                    heading: "Delete this Assistant?".to_string(),
                    warning: "Are you sure you want to delete this Assistant?".to_string(),
                    hidden_fields: vec![
                        ("team_id".into(), team_id.to_string()),
                        ("id".into(), item.id.to_string()),
                    ],
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
