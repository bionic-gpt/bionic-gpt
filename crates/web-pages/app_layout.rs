#![allow(non_snake_case)]
use super::snackbar::Snackbar;
use crate::components::logout_form::LogoutForm;
use crate::menu::{NavGroup, NavItem};
use crate::profile_popup::ProfilePopup;
use assets::files::*;
use db::authz::Rbac;
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Eq, Debug)]
pub enum SideBar {
    None,
    ApiKeys,
    AuditTrail,
    Automations,
    Console,
    Datasets,
    DocumentPipelines,
    Guardrails,
    History,
    Integrations,
    Licence,
    Models,
    OauthClients,
    Prompts,
    Profile,
    RateLimits,
    Switch,
    Team,
    Security,
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

    let show_automations_menu = std::env::var("AUTOMATIONS_FEATURE").is_ok();

    rsx! {
        super::base_layout::BaseLayout {
            title: props.title,
            stylesheets: stylesheets,
            js_href: index_js.name,
            section_class: props.section_class,
            fav_icon_src: bionic_logo_svg.name,
            collapse_svg_src: collapse_svg.name,
            header: rsx!(
                {props.header}
            ),
            sidebar: rsx!(
                NavGroup {
                    heading: "Generative AI",
                    content:  rsx!(
                        NavItem {
                            id: SideBar::Console.to_string(),
                            selected_item_id: props.selected_item.to_string(),
                            href: super::routes::console::Index { team_id: props.team_id },
                            icon: nav_service_requests_svg.name,
                            title: "Chat"
                        }
                        NavItem {
                            id: SideBar::History.to_string(),
                            selected_item_id: props.selected_item.to_string(),
                            href: super::routes::history::Index { team_id: props.team_id },
                            icon: nav_history_svg.name,
                            title: "Chat History"
                        }
                    )
                }
                if props.rbac.can_view_datasets() {
                    NavGroup {
                        heading: "AI Assistants",
                        content:  rsx!(
                            if props.rbac.can_view_prompts() {
                                NavItem {
                                    id: SideBar::Prompts.to_string(),
                                    selected_item_id: props.selected_item.to_string(),
                                    href: super::routes::prompts::Index{team_id: props.team_id},
                                    icon: assistant_svg.name,
                                    title: "Explore Assistants"
                                }
                            }
                            if props.rbac.can_view_integrations() {
                                NavItem {
                                    id: SideBar::Integrations.to_string(),
                                    selected_item_id: props.selected_item.to_string(),
                                    href: super::routes::integrations::Index { team_id: props.team_id },
                                    icon: nav_audit_svg.name,
                                    title: "Integrations"
                                }
                            }
                            NavItem {
                                id: SideBar::Datasets.to_string(),
                                selected_item_id: props.selected_item.to_string(),
                                href: super::routes::datasets::Index{team_id: props.team_id},
                                icon: nav_ccsds_data_svg.name,
                                title: "Datasets & Documents"
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
                                title: "API Keys"
                            }
                            if show_automations_menu {
                                NavItem {
                                    id: SideBar::Automations.to_string(),
                                    selected_item_id: props.selected_item.to_string(),
                                    href: super::routes::automations::Index { team_id: props.team_id },
                                    icon: nav_automations_svg.name,
                                    title: "Automations"
                                }
                            }
                            NavItem {
                                id: SideBar::DocumentPipelines.to_string(),
                                selected_item_id: props.selected_item.to_string(),
                                href: super::routes::document_pipelines::Index { team_id: props.team_id },
                                icon: nav_ccsds_data_svg.name,
                                title: "Document Pipelines"
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
                                href: super::routes::team::Index{team_id:props.team_id},
                                icon: nav_members_svg.name,
                                title: "Team Members"
                            }
                            NavItem {
                                id: SideBar::Switch.to_string(),
                                selected_item_id: props.selected_item.to_string(),
                                href: super::routes::teams::Switch{team_id:props.team_id},
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
                                href: super::routes::models::Index{team_id: props.team_id},
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
                            NavItem {
                                id: SideBar::RateLimits.to_string(),
                                selected_item_id: props.selected_item.to_string(),
                                href: super::routes::rate_limits::Index { team_id: props.team_id },
                                icon: limits_svg.name,
                                title: "Rate Limits"
                            }
                            if props.rbac.is_sys_admin {
                                NavItem {
                                    id: SideBar::OauthClients.to_string(),
                                    selected_item_id: props.selected_item.to_string(),
                                    href: super::routes::oauth_clients::Index { team_id: props.team_id },
                                    icon: nav_api_keys_svg.name,
                                    title: "OAuth Clients"
                                }
                            }
                        )
                    }
                }
            ),
            sidebar_header: rsx!(
                turbo-frame {
                    id: "teams-popup",
                    class: "min-w-full",
                    src: super::routes::team::Popup{ team_id: props.team_id}.to_string()
                }
            ),
            sidebar_footer: rsx!(
                ProfilePopup {
                    email: props.rbac.email.clone(),
                    first_name: props.rbac.first_name.clone().unwrap_or("".to_string()),
                    last_name: props.rbac.last_name.clone().unwrap_or("".to_string()),
                    team_id: props.team_id,
                    unlicensed: props.rbac.unlicensed,
                }
            ),
            {props.children}
            Snackbar {}
            LogoutForm {}
        }
    }
}
