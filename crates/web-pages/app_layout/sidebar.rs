use super::{SideBar, SidebarLabels, SidebarParams};
use crate::menu::{NavGroup, NavItem};
use assets::files::*;
use dioxus::prelude::*;

pub fn render(params: &SidebarParams, labels: &SidebarLabels) -> Element {
    let selected_item = params.selected_item.to_string();
    let ai_assistants_label = labels.ai_assistants.clone();
    let prompts_label = labels.prompts.clone();
    let integrations_label = labels.integrations.clone();
    let history_label = labels.history.clone();
    let team_id = params.team_id;
    let rbac = &params.rbac;
    let can_view_chats = params.can_view_chats;
    let can_view_chat_history = params.can_view_chat_history;
    let setup_required = params.setup_required;

    rsx!(
        if can_view_chats || can_view_chat_history {
            NavGroup {
                heading: "Generative AI",
                content:  rsx!(
                    if can_view_chats {
                        NavItem {
                            id: SideBar::Console.to_string(),
                            selected_item_id: selected_item.clone(),
                            href: crate::routes::console::Index { team_id },
                            icon: nav_service_requests_svg.name,
                            title: "Chat",
                            disabled: setup_required
                        }
                    }
                    if can_view_chat_history {
                        NavItem {
                            id: SideBar::History.to_string(),
                            selected_item_id: selected_item.clone(),
                            href: crate::routes::history::Index { team_id },
                            icon: nav_history_svg.name,
                            title: history_label.clone(),
                            disabled: setup_required
                        }
                    }
                )
            }
        }
        if rbac.can_view_prompts() || rbac.can_view_integrations() {
            NavGroup {
                heading: ai_assistants_label.clone(),
                content:  rsx!(
                    if rbac.can_view_prompts() {
                        NavItem {
                            id: SideBar::Prompts.to_string(),
                            selected_item_id: selected_item.clone(),
                            href: crate::routes::prompts::Index { team_id },
                            icon: assistant_svg.name,
                            title: prompts_label.clone(),
                            disabled: setup_required
                        }
                    }
                    if rbac.can_view_integrations() {
                        NavItem {
                            id: SideBar::Integrations.to_string(),
                            selected_item_id: selected_item.clone(),
                            href: crate::routes::integrations::Index { team_id },
                            icon: nav_audit_svg.name,
                            title: integrations_label.clone(),
                            disabled: setup_required
                        }
                    }
                )
            }
        }
    )
}
