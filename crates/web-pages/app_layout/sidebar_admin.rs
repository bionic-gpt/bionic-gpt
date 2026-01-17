use super::{SideBar, SidebarLabels, SidebarParams};
use crate::menu::{NavGroup, NavItem};
use assets::files::*;
use dioxus::prelude::*;

pub fn render(params: &SidebarParams, labels: &SidebarLabels) -> Element {
    let selected_item = params.selected_item.to_string();
    let ai_assistants_label = labels.ai_assistants.clone();
    let datasets_label = labels.datasets.clone();

    let team_id = params.team_id;
    let rbac = &params.rbac;
    let show_automations_menu = params.show_automations_menu;
    let setup_required = params.setup_required;

    rsx!(
        if rbac.can_manage_mcp_keys() || rbac.can_view_datasets() {
            NavGroup {
                heading: ai_assistants_label,
                content: rsx!(
                    if rbac.can_view_datasets() {
                        NavItem {
                            id: SideBar::Datasets.to_string(),
                            selected_item_id: selected_item.clone(),
                            href: crate::routes::datasets::Index { team_id },
                            icon: nav_ccsds_data_svg.name,
                            title: datasets_label.clone(),
                            disabled: setup_required
                        }
                    }
                )
            }
        }
        if rbac.can_use_api_keys() {
            NavGroup {
                heading: "Developers",
                content:  rsx!(
                    NavItem {
                        id: SideBar::ApiKeys.to_string(),
                        selected_item_id: selected_item.clone(),
                        href: crate::routes::api_keys::Index { team_id },
                        icon: nav_api_keys_svg.name,
                        title: "API Keys",
                        disabled: setup_required
                    }
                    if show_automations_menu {
                        NavItem {
                            id: SideBar::Automations.to_string(),
                            selected_item_id: selected_item.clone(),
                            href: crate::routes::automations::Index { team_id },
                            icon: nav_automations_svg.name,
                            title: "Automations",
                            disabled: setup_required
                        }
                    }
                    if rbac.can_manage_document_pipelines() {
                        NavItem {
                            id: SideBar::DocumentPipelines.to_string(),
                            selected_item_id: selected_item.clone(),
                            href: crate::routes::document_pipelines::Index { team_id },
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
                        href: crate::routes::teams::Switch { team_id },
                        icon: nav_teams_svg.name,
                        title: "Teams",
                        disabled: setup_required
                    }
                )
            }
        }
        if rbac.can_view_audit_trail() || rbac.can_setup_models() {
            NavGroup {
                heading: "System Admin",
                content:  rsx!(
                    NavItem {
                        id: SideBar::Models.to_string(),
                        selected_item_id: selected_item.clone(),
                        href: crate::routes::models::Index { team_id },
                        icon: nav_phonebook_svg.name,
                        title: "Model Setup",
                        disabled: false
                    }
                    NavItem {
                        id: SideBar::AuditTrail.to_string(),
                        selected_item_id: selected_item.clone(),
                        href: crate::routes::audit_trail::Index { team_id },
                        icon: nav_audit_svg.name,
                        title: "Audit Trail",
                        disabled: setup_required
                    }
                    NavItem {
                        id: SideBar::RateLimits.to_string(),
                        selected_item_id: selected_item.clone(),
                        href: crate::routes::rate_limits::Index { team_id },
                        icon: limits_svg.name,
                        title: "Rate Limits",
                        disabled: setup_required
                    }
                    if rbac.is_sys_admin {
                        NavItem {
                            id: SideBar::OauthClients.to_string(),
                            selected_item_id: selected_item.clone(),
                            href: crate::routes::oauth_clients::Index { team_id },
                            icon: nav_api_keys_svg.name,
                            title: "OAuth Clients",
                            disabled: setup_required
                        }
                        NavItem {
                            id: SideBar::OpenapiSpecs.to_string(),
                            selected_item_id: selected_item.clone(),
                            href: crate::routes::openapi_specs::Index { team_id },
                            icon: nav_audit_svg.name,
                            title: "OpenAPI Specs",
                            disabled: setup_required
                        }
                        NavItem {
                            id: SideBar::Categories.to_string(),
                            selected_item_id: selected_item.clone(),
                            href: crate::routes::categories::Index { team_id },
                            icon: nav_audit_svg.name,
                            title: "Categories",
                            disabled: setup_required
                        }
                        NavItem {
                            id: SideBar::Licence.to_string(),
                            selected_item_id: selected_item,
                            href: crate::routes::licence::Index { team_id },
                            icon: nav_audit_svg.name,
                            title: "System Info",
                            disabled: setup_required
                        }
                    }
                )
            }
        }
    )
}
