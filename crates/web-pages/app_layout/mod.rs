#![allow(non_snake_case)]

mod base;
mod project_sidebar;
mod sidebar;
mod sidebar_admin;

pub use base::{AppLayoutProps as BaseProps, BaseLayout};

use crate::components::logout_form::LogoutForm;
use crate::i18n;
use crate::profile_popup::ProfilePopup;
use crate::snackbar::Snackbar;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Eq, Debug)]
pub enum SideBar {
    None,
    ApiKeys,
    AuditTrail,
    Console,
    Datasets,
    DocumentPipelines,
    Guardrails,
    History,
    ScheduledTasks,
    Integrations,
    McpApiKeys,
    Models,
    OauthClients,
    OpenapiSpecs,
    Projects,
    Providers,
    Profile,
    RateLimits,
    Switch,
    Security,
    Skills,
    SystemPrompt,
    WebSearch,
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
    team_id: String,
    rbac: Rbac,
    section_class: String,
    #[props(default)]
    locale: Option<String>,
    #[props(default)]
    setup_required: bool,
    #[props(default)]
    selected_project_id: Option<i32>,
}

#[derive(Clone)]
pub(super) struct SidebarLabels {
    pub history: String,
}

#[derive(Clone)]
pub(super) struct SidebarParams {
    pub team_id: String,
    pub selected_item: SideBar,
    pub rbac: Rbac,
    pub can_view_chats: bool,
    pub can_view_chat_history: bool,
    pub setup_required: bool,
    pub selected_project_id: Option<i32>,
}

pub fn Layout(props: LayoutProps) -> Element {
    layout(props, LayoutMode::Main)
}

pub fn AdminLayout(props: LayoutProps) -> Element {
    layout(props, LayoutMode::Admin)
}

#[derive(Clone, Copy)]
enum LayoutMode {
    Main,
    Admin,
}

fn layout(props: LayoutProps, mode: LayoutMode) -> Element {
    let stylesheets = vec![index_css.name.to_string(), output_css.name.to_string()];

    let locale = props
        .locale
        .clone()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "en".to_string());

    let history_label = i18n::histories(&locale);

    let app_logo_src = bionic_logo_svg.name.to_string();
    let app_name = "Bionic".to_string();

    let team_id = props.team_id.clone();
    let switch_teams_href = crate::routes::teams::Switch {
        team_id: team_id.clone(),
    }
    .to_string();

    let current_team_label = props
        .rbac
        .current_team_name
        .clone()
        .unwrap_or_else(|| "Switch teams".to_string());

    let can_view_chats = props.rbac.can_view_chats();
    let can_view_chat_history = props.rbac.can_view_chat_history();

    let sidebar_labels = SidebarLabels {
        history: history_label.clone(),
    };

    let sidebar_params = SidebarParams {
        team_id: team_id.clone(),
        selected_item: props.selected_item.clone(),
        rbac: props.rbac.clone(),
        can_view_chats,
        can_view_chat_history,
        setup_required: props.setup_required,
        selected_project_id: props.selected_project_id,
    };

    let sidebar_content = match mode {
        LayoutMode::Main => sidebar::render(&sidebar_params, &sidebar_labels),
        LayoutMode::Admin => sidebar_admin::render(&sidebar_params, &sidebar_labels),
    };

    let overlays = match mode {
        LayoutMode::Main if props.rbac.can_manage_projects() && !props.setup_required => {
            rsx!(crate::projects::upsert::Upsert {
                id: None,
                trigger_id: "sidebar-new-project",
                name: "".to_string(),
                instructions: "".to_string(),
                visibility: db::Visibility::Private,
                can_set_visibility_to_company: false,
                team_id: team_id.clone(),
            })
        }
        _ => rsx!(),
    };

    let admin_href = admin_home_href(&props.rbac, team_id.clone());
    let main_href = main_home_href(&props.rbac, team_id.clone());

    let profile_section = if props.setup_required {
        rsx!(
            div {
                class: "btn btn-ghost btn-sm w-full justify-start mb-2 flex items-center gap-2 opacity-50 pointer-events-none",
                img {
                    width: "16",
                    height: "16",
                    src: profile_svg.name
                }
                "Profile"
            }
        )
    } else {
        rsx!(ProfilePopup {
            email: props.rbac.email.clone(),
            first_name: props.rbac.first_name.clone().unwrap_or("".to_string()),
            last_name: props.rbac.last_name.clone().unwrap_or("".to_string()),
            team_id: team_id.clone(),
        })
    };

    let sidebar_footer = match mode {
        LayoutMode::Main => rsx!(
            if let Some(href) = admin_href.clone() {
                a {
                    class: "btn btn-ghost btn-sm w-full justify-start mb-2 flex items-center gap-2",
                    href: "{href}",
                    img {
                        width: "16",
                        height: "16",
                        src: settings_svg.name
                    }
                    "Admin Panel"
                }
            }
            {profile_section}
        ),
        LayoutMode::Admin => rsx!(
            if let Some(href) = main_href.clone() {
                if props.setup_required {
                    button {
                        class: "btn btn-ghost btn-sm w-full justify-start mb-2 flex items-center gap-2 opacity-50 pointer-events-none",
                        img {
                            width: "16",
                            height: "16",
                            src: left_arrow_svg.name
                        }
                        "Back to app"
                    }
                } else {
                    a {
                        class: "btn btn-ghost btn-sm w-full justify-start mb-2 flex items-center gap-2",
                        href: "{href}",
                        img {
                            width: "16",
                            height: "16",
                            src: left_arrow_svg.name
                        }
                        "Back to app"
                    }
                }
            }
            {profile_section}
        ),
    };

    rsx! {
        BaseLayout {
            title: props.title,
            stylesheets: stylesheets,
            js_href: index_js.name,
            section_class: props.section_class,
            fav_icon_src: app_logo_src.clone(),
            collapse_svg_src: collapse_svg.name,
            header: rsx!(
                {props.header}
            ),
            sidebar: sidebar_content,
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
            sidebar_footer: sidebar_footer,
            overlays,
            {props.children}
            Snackbar {}
            LogoutForm {}
        }
    }
}

fn admin_home_href(rbac: &Rbac, team_id: String) -> Option<String> {
    if rbac.can_use_api_keys() {
        return Some(
            crate::routes::api_keys::Index {
                team_id: team_id.clone(),
            }
            .to_string(),
        );
    }

    if rbac.can_view_teams() {
        return Some(
            crate::routes::teams::Switch {
                team_id: team_id.clone(),
            }
            .to_string(),
        );
    }
    if rbac.can_setup_models() || rbac.is_sys_admin {
        return Some(
            crate::routes::models::Index {
                team_id: team_id.clone(),
            }
            .to_string(),
        );
    }
    if rbac.can_view_audit_trail() {
        return Some(
            crate::routes::audit_trail::Index {
                team_id: team_id.clone(),
            }
            .to_string(),
        );
    }
    if rbac.is_sys_admin {
        return Some(
            crate::routes::oauth_clients::Index {
                team_id: team_id.clone(),
            }
            .to_string(),
        );
    }

    None
}

fn main_home_href(rbac: &Rbac, team_id: String) -> Option<String> {
    if rbac.can_view_chats() {
        return Some(
            crate::routes::console::Index {
                team_id: team_id.clone(),
            }
            .to_string(),
        );
    }
    if rbac.can_view_chat_history() {
        return Some(
            crate::routes::history::Index {
                team_id: team_id.clone(),
            }
            .to_string(),
        );
    }
    if rbac.can_manage_projects() {
        return Some(
            crate::routes::projects::Index {
                team_id: team_id.clone(),
            }
            .to_string(),
        );
    }
    if rbac.can_view_integrations() {
        return Some(
            crate::routes::integrations::Index {
                team_id: team_id.clone(),
            }
            .to_string(),
        );
    }

    None
}
