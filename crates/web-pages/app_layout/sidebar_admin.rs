use super::{SideBar, SidebarLabels, SidebarParams};
use crate::menu::{NavGroup, NavItem};
use assets::files::*;
use dioxus::prelude::*;

pub fn render(params: &SidebarParams, _labels: &SidebarLabels) -> Element {
    let selected_item = params.selected_item.to_string();

    let team_id = params.team_id.clone();
    let rbac = &params.rbac;
    let setup_required = params.setup_required;

    rsx!(
        if rbac.can_use_api_keys() {
            NavGroup {
                heading: "Developers",
                content:  rsx!(
                    NavItem {
                        id: SideBar::ApiKeys.to_string(),
                        selected_item_id: selected_item.clone(),
                        href: crate::routes::api_keys::Index { team_id: team_id.clone() },
                        icon: nav_api_keys_svg.name,
                        title: "API Keys",
                        disabled: setup_required
                    }
                    if rbac.can_manage_document_pipelines() {
                        NavItem {
                            id: SideBar::DocumentPipelines.to_string(),
                            selected_item_id: selected_item.clone(),
                            href: crate::routes::document_pipelines::Index { team_id: team_id.clone() },
                            icon: nav_ccsds_data_svg.name,
                            title: "Document Pipelines",
                            disabled: setup_required
                        }
                    }
                )
            }
        }
        if rbac.can_view_teams() {
            NavGroup {
                heading: "Collaboration",
                content:  rsx!(
                    NavItem {
                        id: SideBar::Switch.to_string(),
                        selected_item_id: selected_item.clone(),
                        href: crate::routes::teams::Switch { team_id: team_id.clone() },
                        icon: nav_teams_svg.name,
                        title: "Teams",
                        disabled: setup_required
                    }
                )
            }
        }
        if rbac.can_setup_models() || rbac.is_sys_admin {
            NavGroup {
                heading: "Model Gateway",
                content:  rsx!(
                    NavItem {
                        id: SideBar::Models.to_string(),
                        selected_item_id: selected_item.clone(),
                        href: crate::routes::models::Index { team_id: team_id.clone() },
                        icon: nav_phonebook_svg.name,
                        title: "Model Setup",
                        disabled: false
                    }
                    NavItem {
                        id: SideBar::RateLimits.to_string(),
                        selected_item_id: selected_item.clone(),
                        href: crate::routes::rate_limits::Index { team_id: team_id.clone() },
                        icon: limits_svg.name,
                        title: "Rate Limits",
                        disabled: setup_required
                    }
                    if rbac.is_sys_admin {
                        NavItem {
                            id: SideBar::Providers.to_string(),
                            selected_item_id: selected_item.clone(),
                            href: crate::routes::providers::Index { team_id: team_id.clone() },
                            icon: nav_phonebook_svg.name,
                            title: "Providers",
                            disabled: setup_required
                        }
                    }
                )
            }
        }
        if rbac.can_view_audit_trail() || rbac.can_setup_models() {
            NavGroup {
                heading: "System Admin",
                content:  rsx!(
                    NavItem {
                        id: SideBar::AuditTrail.to_string(),
                        selected_item_id: selected_item.clone(),
                        href: crate::routes::audit_trail::Index { team_id: team_id.clone() },
                        icon: nav_audit_svg.name,
                        title: "Audit Trail",
                        disabled: setup_required
                    }
                    if rbac.is_sys_admin {
                        NavItem {
                            id: SideBar::OauthClients.to_string(),
                            selected_item_id: selected_item.clone(),
                            href: crate::routes::oauth_clients::Index { team_id: team_id.clone() },
                            icon: nav_api_keys_svg.name,
                            title: "OAuth Clients",
                            disabled: setup_required
                        }
                        NavItem {
                            id: SideBar::OpenapiSpecs.to_string(),
                            selected_item_id: selected_item.clone(),
                            href: crate::routes::openapi_specs::Index { team_id: team_id.clone() },
                            icon: nav_audit_svg.name,
                            title: "OpenAPI Specs",
                            disabled: setup_required
                        }
                        NavItem {
                            id: SideBar::WebSearch.to_string(),
                            selected_item_id: selected_item.clone(),
                            href: crate::routes::web_search::Index { team_id: team_id.clone() },
                            icon: nav_audit_svg.name,
                            title: "Web Search",
                            disabled: setup_required
                        }
                        NavItem {
                            id: SideBar::SystemPrompt.to_string(),
                            selected_item_id: selected_item.clone(),
                            href: crate::routes::system_prompt::Index { team_id: team_id.clone() },
                            icon: nav_audit_svg.name,
                            title: "System Prompt",
                            disabled: setup_required
                        }
                    }
                )
            }
        }
    )
}
