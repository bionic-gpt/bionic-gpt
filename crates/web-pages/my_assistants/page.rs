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

pub fn page(team_id: i32, rbac: Rbac, prompts: Vec<MyPrompt>, locale: &str) -> String {
    let assistants_label = crate::i18n::assistants(locale);
    let assistant_label = crate::i18n::assistant(locale);
    let prompts_label = crate::i18n::prompts(locale);
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Prompts,
            team_id: team_id,
            rbac: rbac.clone(),
            title: format!("My {}", assistants_label.clone()),
            locale: Some(locale.to_string()),
            header: rsx!(
                Breadcrumb {
                    items: vec![
                        BreadcrumbItem {
                            text: assistants_label.clone(),
                            href: Some(crate::routes::prompts::Index{team_id}.to_string())
                        },
                        BreadcrumbItem {
                            text: format!("My {}", assistants_label.clone()),
                            href: None
                        }
                    ]
                }
                div {
                    a {
                        href: crate::routes::prompts::Index{team_id}.to_string(),
                        class: "btn btn-ghost btn-sm font-bold! mr-4",
                        {prompts_label.clone()}
                    }
                    Button {
                        button_type: ButtonType::Link,
                        prefix_image_src: "{button_plus_svg.name}",
                        href: routes::prompts::New{team_id}.to_string(),
                        button_scheme: ButtonScheme::Primary,
                        {format!("New {}", assistant_label.clone())}
                    }
                }
            ),

            div {
                class: "p-4 max-w-3xl w-full mx-auto",


                SectionIntroduction {
                    header: format!("Your {}", assistants_label.clone()),
                    subtitle: "Discover and create custom chat bots that combine instructions,
                        extra knowledge, and any combination of skills.".to_string(),
                    is_empty: prompts.is_empty(),
                    empty_text: format!(
                        "You haven't created any {} yet.",
                        assistants_label.to_lowercase()
                    ),
                }

                div {
                    class: "space-y-2",
                        for prompt in &prompts {
                        MyAssistantCard {
                            team_id,
                            prompt: prompt.clone(),
                            locale: locale.to_string()
                        }
                    }
                }
            }


            for item in &prompts {
                ConfirmModal {
                    action: crate::routes::prompts::Delete{team_id, id: item.id}.to_string(),
                    trigger_id: format!("delete-trigger-{}-{}", item.id, team_id),
                    submit_label: "Delete".to_string(),
                    heading: format!("Delete this {}?", assistant_label.clone()),
                    warning: format!(
                        "Are you sure you want to delete this {}?",
                        assistant_label.clone()
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
