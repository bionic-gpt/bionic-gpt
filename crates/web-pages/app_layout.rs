#![allow(non_snake_case)]
use super::logout_form::LogoutForm;
use crate::profile_popup::ProfilePopup;
use assets::files::*;
use daisy_rsx::{AppLayout, NavGroup, NavItem};
use db::authz::Rbac;
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Eq, Debug)]
pub enum SideBar {
    None,
    ApiKeys,
    AuditTrail,
    Console,
    Training,
    Prompts,
    Models,
    Datasets,
    DocumentPipelines,
    Team,
    Profile,
    Switch,
}

impl std::fmt::Display for SideBar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct LayoutProps {
    selected_item: SideBar,
    title: String,
    header: Element,
    children: Element,
    team_id: i32,
    rbac: Rbac,
    section_class: String,
}

pub fn Layout(props: LayoutProps) -> Element {
    let stylesheets = vec![index_css.name.to_string(), output_css.name.to_string()];

    rsx! {
        AppLayout {
            title: props.title,
            stylesheets: stylesheets,
            js_href: index_js.name,
            section_class: props.section_class,
            fav_icon_src: favicon_svg.name,
            collapse_svg_src: collapse_svg.name,
            header: rsx!(
                {props.header}
            ),
            sidebar: rsx!(
                NavGroup {
                    heading: "AI Chat",
                    content:  rsx!(
                        NavItem {
                            id: SideBar::Console.to_string(),
                            selected_item_id: props.selected_item.to_string(),
                            href: super::routes::console::index_route(props.team_id),
                            icon: nav_service_requests_svg.name,
                            title: "Chat Console"
                        }
                        if props.rbac.can_view_prompts() {
                            NavItem {
                                id: SideBar::Prompts.to_string(),
                                selected_item_id: props.selected_item.to_string(),
                                href: super::routes::prompts::index_route(props.team_id),
                                icon: nav_dashboard_svg.name,
                                title: "Prompts"
                            }
                        }
                    )
                }
                if props.rbac.can_view_datasets() {
                    NavGroup {
                        heading: "Retrieval Augmented Generation",
                        content:  rsx!(
                            NavItem {
                                id: SideBar::Datasets.to_string(),
                                selected_item_id: props.selected_item.to_string(),
                                href: super::routes::datasets::index_route(props.team_id),
                                icon: nav_ccsds_data_svg.name,
                                title: "Team Datasets"
                            }
                            NavItem {
                                id: SideBar::DocumentPipelines.to_string(),
                                selected_item_id: props.selected_item.to_string(),
                                href: super::routes::document_pipelines::index_route(props.team_id),
                                icon: nav_ccsds_data_svg.name,
                                title: "Document Pipelines"
                            }
                        )
                    }
                }
                if props.rbac.can_use_api_keys() {
                    NavGroup {
                        heading: "Developers",
                        content:  rsx!(
                            NavItem {
                                id: SideBar::ApiKeys.to_string(),
                                selected_item_id: props.selected_item.to_string(),
                                href: super::routes::api_keys::Index { team_id: props.team_id },
                                icon: nav_api_keys_svg.name,
                                title: "LLM API Keys"
                            }
                        )
                    }
                }
                if props.rbac.can_view_teams() {
                    NavGroup {
                        heading: "Collaboration",
                        content:  rsx!(
                            NavItem {
                                id: SideBar::Team.to_string(),
                                selected_item_id: props.selected_item.to_string(),
                                href: super::routes::team::index_route(props.team_id),
                                icon: nav_members_svg.name,
                                title: "Team Members"
                            }
                            NavItem {
                                id: SideBar::Switch.to_string(),
                                selected_item_id: props.selected_item.to_string(),
                                href: super::routes::team::switch_route(props.team_id),
                                icon: nav_teams_svg.name,
                                title: "Your Teams"
                            }
                        )
                    }
                }
                if props.rbac.can_view_audit_trail() || props.rbac.can_setup_models() {
                    NavGroup {
                        heading: "System Admin",
                        content:  rsx!(
                            NavItem {
                                id: SideBar::Models.to_string(),
                                selected_item_id: props.selected_item.to_string(),
                                href: super::routes::models::index_route(props.team_id),
                                icon: nav_phonebook_svg.name,
                                title: "Model Setup"
                            }
                            NavItem {
                                id: SideBar::AuditTrail.to_string(),
                                selected_item_id: props.selected_item.to_string(),
                                href: super::routes::audit_trail::Index { team_id: props.team_id },
                                icon: nav_audit_svg.name,
                                title: "Audit Trail"
                            }
                        )
                    }
                }
            ),
            sidebar_header: rsx!(
                turbo-frame {
                    id: "teams-popup",
                    class: "min-w-full",
                    src: "{super::routes::team::teams_popup_route(props.team_id)}"
                }
            ),
            sidebar_footer: rsx!(
                ProfilePopup {
                    email: props.rbac.email.clone(),
                    first_name: props.rbac.first_name.clone().unwrap_or("".to_string()),
                    last_name: props.rbac.last_name.clone().unwrap_or("".to_string()),
                    team_id: props.team_id
                }
            ),
            {props.children}
            snack-bar {}
            LogoutForm {}
        }
    }
}
