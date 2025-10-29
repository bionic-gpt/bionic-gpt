#![allow(non_snake_case)]

use super::snackbar::Snackbar;
use crate::components::logout_form::LogoutForm;
use crate::i18n;
use crate::menu::{NavGroup, NavItem};
use crate::profile_popup::ProfilePopup;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::Licence;
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
    McpApiKeys,
    Licence,
    Models,
    Categories,
    OauthClients,
    Prompts,
    Profile,
    RateLimits,
    Switch,
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

    let licence = Licence::global();
    let app_logo_src: String = if licence.app_logo_svg.is_empty() {
        bionic_logo_svg.name.to_string()
    } else {
        format!("data:image/svg+xml;base64,{}", licence.app_logo_svg)
    };

    let app_name = if licence.app_name.is_empty() {
        "Bionic".to_string()
    } else {
        licence.app_name.clone()
    };

    let switch_teams_href = super::routes::teams::Switch {
        team_id: props.team_id,
    }
    .to_string();

    let current_team_label = props
        .rbac
        .current_team_name
        .clone()
        .unwrap_or_else(|| "Switch teams".to_string());

    rsx! {
        super::base_layout::BaseLayout {
            title: props.title,
            stylesheets: stylesheets,
            js_href: index_js.name,
            section_class: props.section_class,
            fav_icon_src: app_logo_src.clone(),
            collapse_svg_src: collapse_svg.name,
            header: rsx!(
                {props.header}
            ),
            sidebar: rsx!(
                if props.rbac.can_view_chats() {
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
                }
                if props.rbac.can_view_datasets() || props.rbac.can_view_integrations() {
                    NavGroup {
                        heading: i18n::ai_assistants().to_string(),
                        content:  rsx!(
                            if props.rbac.can_view_prompts() {
                                NavItem {
                                    id: SideBar::Prompts.to_string(),
                                    selected_item_id: props.selected_item.to_string(),
                                    href: super::routes::prompts::Index{team_id: props.team_id},
                                    icon: assistant_svg.name,
                                    title: i18n::prompts().to_string()
                                }
                            }
                            if props.rbac.can_view_integrations() {
                                NavItem {
                                    id: SideBar::Integrations.to_string(),
                                    selected_item_id: props.selected_item.to_string(),
                                    href: super::routes::integrations::Index { team_id: props.team_id },
                                    icon: nav_audit_svg.name,
                                    title: i18n::integrations().to_string()
                                }
                                if props.rbac.can_manage_mcp_keys() {
                                    NavItem {
                                        id: SideBar::McpApiKeys.to_string(),
                                        selected_item_id: props.selected_item.to_string(),
                                        href: super::routes::mcp_api_keys::Index { team_id: props.team_id },
                                        icon: nav_api_keys_svg.name,
                                        title: "API Keys"
                                    }
                                }
                            }
                            if props.rbac.can_view_datasets() {
                                NavItem {
                                    id: SideBar::Datasets.to_string(),
                                    selected_item_id: props.selected_item.to_string(),
                                    href: super::routes::datasets::Index{team_id: props.team_id},
                                    icon: nav_ccsds_data_svg.name,
                                    title: i18n::datasets().to_string()
                                }
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
                            if props.rbac.can_manage_document_pipelines() {
                                NavItem {
                                    id: SideBar::DocumentPipelines.to_string(),
                                    selected_item_id: props.selected_item.to_string(),
                                    href: super::routes::document_pipelines::Index { team_id: props.team_id },
                                    icon: nav_ccsds_data_svg.name,
                                    title: "Document Pipelines"
                                }
                            }
                        )
                    }
                }
                if props.rbac.can_view_teams() {
                    NavGroup {
                        heading: "Collaboration",
                        content:  rsx!(
                            NavItem {
                                id: SideBar::Switch.to_string(),
                                selected_item_id: props.selected_item.to_string(),
                                href: super::routes::teams::Switch{team_id:props.team_id},
                                icon: nav_teams_svg.name,
                                title: "Teams"
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
                                NavItem {
                                    id: SideBar::Categories.to_string(),
                                    selected_item_id: props.selected_item.to_string(),
                                    href: super::routes::categories::Index { team_id: props.team_id },
                                    icon: nav_audit_svg.name,
                                    title: "Categories"
                                }
                                NavItem {
                                    id: SideBar::Licence.to_string(),
                                    selected_item_id: props.selected_item.to_string(),
                                    href: super::routes::licence::Index { team_id: props.team_id },
                                    icon: nav_audit_svg.name,
                                    title: "System Info"
                                }
                            }
                        )
                    }
                }
            ),
            sidebar_header: rsx!(
                if props.rbac.has_multiple_teams {
                    DropDown {
                        direction: Direction::Bottom,
                        button_text: "{current_team_label}",
                        suffix_image_src: button_select_svg.name,
                        class: "w-full",
                        DropDownLink {
                            href: "{switch_teams_href}",
                            target: "_top",
                            "Switch teams"
                        }
                    }
                } else {
                    div {
                        class: "flex gap-2 height-full w-full items-center",
                        img {
                            height: "16",
                            width: "16",
                            src: "{app_logo_src}"
                        }
                        h4 {
                            "{app_name}"
                        }
                    }
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
