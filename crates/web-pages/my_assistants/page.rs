#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::components::confirm_modal::ConfirmModal;
use crate::my_assistants::assistant_card::MyAssistantCard;
use crate::routes;
use crate::SectionIntroduction;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::prompts::MyPrompt;
use dioxus::prelude::*;

pub fn page(team_id: i32, rbac: Rbac, prompts: Vec<MyPrompt>) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Prompts,
            team_id: team_id,
            rbac: rbac.clone(),
            title: "My Assistants",
            header: rsx!(
                Breadcrumb {
                    items: vec![
                        BreadcrumbItem {
                            text: "Assistants".into(),
                            href: Some(crate::routes::prompts::Index{team_id}.to_string())
                        },
                        BreadcrumbItem {
                            text: "My Assistants".into(),
                            href: None
                        }
                    ]
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

            div {
                class: "p-4 max-w-3xl w-full mx-auto",


                SectionIntroduction {
                    header: "Your Assistants".to_string(),
                    subtitle: "Discover and create custom chat bots that combine instructions,
                        extra knowledge, and any combination of skills.".to_string(),
                    is_empty: prompts.is_empty(),
                    empty_text: "You haven't created any assistants yet.".to_string(),
                }

                div {
                    class: "space-y-2",
                        for prompt in &prompts {
                        MyAssistantCard {
                            team_id,
                            prompt: prompt.clone()
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
