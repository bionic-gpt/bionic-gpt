#![allow(non_snake_case)]
use super::provider_card::ProviderCard;
use crate::app_layout::{AdminLayout, SideBar};
use crate::components::confirm_modal::ConfirmModal;
use crate::SectionIntroduction;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::Provider;
use dioxus::prelude::*;

pub fn page(team_id: i32, rbac: Rbac, providers: Vec<Provider>) -> String {
    let page = rsx! {
        AdminLayout {
            section_class: "p-4",
            selected_item: SideBar::Providers,
            team_id: team_id,
            rbac: rbac.clone(),
            title: "Providers",
            header: rsx!(
                Breadcrumb {
                    items: vec![BreadcrumbItem {
                        text: "Providers".into(),
                        href: None
                    }]
                }
                Button {
                    button_type: ButtonType::Link,
                    prefix_image_src: "{button_plus_svg.name}",
                    href: crate::routes::providers::New { team_id }.to_string(),
                    button_scheme: ButtonScheme::Primary,
                    "Add Provider"
                }
            ),

            div {
                class: "p-4 max-w-3xl w-full mx-auto",
                SectionIntroduction {
                    header: "Providers".to_string(),
                    subtitle: "Manage self-hosted LLM providers and their default models.".to_string(),
                    is_empty: providers.is_empty(),
                    empty_text: "No providers configured yet. Add one to get started.".to_string(),
                }
                if !providers.is_empty() {
                    div {
                        class: "space-y-2",
                        for provider in &providers {
                            ProviderCard {
                                team_id,
                                provider: provider.clone(),
                            }
                        }
                    }
                }
            }

            for provider in &providers {
                ConfirmModal {
                    action: crate::routes::providers::Delete{team_id, id: provider.id}.to_string(),
                    trigger_id: format!("delete-provider-{}-{}", provider.id, team_id),
                    submit_label: "Delete".to_string(),
                    heading: "Delete this Provider?".to_string(),
                    warning: "Are you sure you want to delete this Provider?".to_string(),
                    hidden_fields: vec![
                        ("team_id".into(), team_id.to_string()),
                        ("id".into(), provider.id.to_string()),
                    ],
                }
            }
        }
    };

    crate::render(page)
}
