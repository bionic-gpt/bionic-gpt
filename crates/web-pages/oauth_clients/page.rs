#![allow(non_snake_case)]
use crate::app_layout::{AdminLayout, SideBar};
use crate::routes;
use crate::SectionIntroduction;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use dioxus::prelude::*;

pub fn page(team_id: String, rbac: Rbac, oauth_clients: Vec<db::OauthClient>) -> String {
    let page = rsx! {
        AdminLayout {
            section_class: "p-4",
            selected_item: SideBar::OauthClients,
            team_id: team_id.clone(),
            rbac: rbac.clone(),
            title: "OAuth Clients",
            header: rsx!(
                Breadcrumb {
                    items: vec![BreadcrumbItem {
                        text: "OAuth Clients".into(),
                        href: Some(routes::oauth_clients::Index { team_id: team_id.clone() }.to_string())
                    }]
                }
                if rbac.is_sys_admin {
                    Button {
                        button_type: ButtonType::Link,
                        prefix_image_src: "{button_plus_svg.name}",
                        href: routes::oauth_clients::New{team_id: team_id.clone()}.to_string(),
                        button_scheme: ButtonScheme::Primary,
                        "Add OAuth Client"
                    }
                }
            ),

            div {
                class: "p-4 max-w-3xl w-full mx-auto",
                SectionIntroduction {
                    header: "OAuth Clients".to_string(),
                    subtitle: "Configure OAuth client credentials for external service integrations.".to_string(),
                    is_empty: oauth_clients.is_empty(),
                    empty_text: "No OAuth clients configured yet. Add your first OAuth client to enable external service integrations.".to_string(),
                }

                if !oauth_clients.is_empty() {
                    for oauth_client in oauth_clients {
                        super::oauth_client_card::OauthClientCard {
                            oauth_client: oauth_client,
                            team_id: team_id.clone(),
                            rbac: rbac.clone()
                        }
                    }
                }
            }
        }
    };

    crate::render(page)
}
