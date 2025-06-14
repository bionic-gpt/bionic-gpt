#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::routes;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use dioxus::prelude::*;

pub fn page(team_id: i32, rbac: Rbac, oauth_clients: Vec<db::OauthClient>) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::OauthClients,
            team_id: team_id,
            rbac: rbac.clone(),
            title: "OAuth Clients",
            header: rsx!(
                Breadcrumb {
                    items: vec![BreadcrumbItem {
                        text: "OAuth Clients".into(),
                        href: Some(routes::oauth_clients::Index { team_id }.to_string())
                    }]
                }
                if rbac.is_sys_admin {
                    Button {
                        button_type: ButtonType::Link,
                        prefix_image_src: "{button_plus_svg.name}",
                        href: routes::oauth_clients::New{team_id}.to_string(),
                        button_scheme: ButtonScheme::Primary,
                        "Add OAuth Client"
                    }
                }
            ),

            div {
                class: "p-4 max-w-4xl w-full mx-auto",
                h1 {
                    class: "text-xl font-semibold",
                    "OAuth Clients"
                }
                p {
                    "Configure OAuth client credentials for external service integrations."
                }

                if oauth_clients.is_empty() {
                    div {
                        class: "text-center py-8",
                        p { class: "text-base-content/70", "No OAuth clients configured yet." }
                        if rbac.is_sys_admin {
                            p { class: "mt-2",
                                Button {
                                    button_type: ButtonType::Link,
                                    href: routes::oauth_clients::New { team_id }.to_string(),
                                    button_scheme: ButtonScheme::Primary,
                                    "Create your first OAuth client"
                                }
                            }
                        }
                    }
                } else {
                    for oauth_client in oauth_clients {
                        super::oauth_client_card::OauthClientCard {
                            oauth_client: oauth_client,
                            team_id: team_id,
                            rbac: rbac.clone()
                        }
                    }
                }
            }
        }
    };

    crate::render(page)
}
