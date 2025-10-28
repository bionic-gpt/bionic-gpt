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
            title: format!("My {}", crate::i18n::assistants()),
            header: rsx!(
                Breadcrumb {
                    items: vec![
                        BreadcrumbItem {
                            text: crate::i18n::assistants().to_string(),
                            href: Some(crate::routes::prompts::Index{team_id}.to_string())
                        },
                        BreadcrumbItem {
                            text: format!("My {}", crate::i18n::assistants()),
                            href: None
                        }
                    ]
                }
                div {
                    a {
                        href: crate::routes::prompts::Index{team_id}.to_string(),
                        class: "btn btn-ghost btn-sm font-bold! mr-4",
                        {crate::i18n::prompts()}
                    }
                    Button {
                        button_type: ButtonType::Link,
                        prefix_image_src: "{button_plus_svg.name}",
                        href: routes::prompts::New{team_id}.to_string(),
                        button_scheme: ButtonScheme::Primary,
                        {format!("New {}", crate::i18n::assistant())}
                    }
                }
            ),

            div {
                class: "p-4 max-w-3xl w-full mx-auto",


                SectionIntroduction {
                    header: format!("Your {}", crate::i18n::assistants()),
                    subtitle: "Discover and create custom chat bots that combine instructions,
                        extra knowledge, and any combination of skills.".to_string(),
                    is_empty: prompts.is_empty(),
                    empty_text: format!(
                        "You haven't created any {} yet.",
                        crate::i18n::assistants().to_lowercase()
                    ),
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
                    heading: format!("Delete this {}?", crate::i18n::assistant()),
                    warning: format!(
                        "Are you sure you want to delete this {}?",
                        crate::i18n::assistant()
                    ),
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
