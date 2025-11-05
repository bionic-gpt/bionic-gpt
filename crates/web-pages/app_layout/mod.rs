#![allow(non_snake_case)]

mod base;
mod sidebar;
mod sidebar_mcp;

pub use base::{AppLayoutProps as BaseProps, BaseLayout};

use crate::components::logout_form::LogoutForm;
use crate::i18n;
use crate::profile_popup::ProfilePopup;
use crate::snackbar::Snackbar;
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
    OpenapiSpecs,
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
    #[props(default)]
    locale: Option<String>,
}

#[derive(Clone)]
pub(super) struct SidebarLabels {
    pub ai_assistants: String,
    pub prompts: String,
    pub integrations: String,
    pub history: String,
    pub datasets: String,
}

#[derive(Clone)]
pub(super) struct SidebarParams {
    pub team_id: i32,
    pub selected_item: SideBar,
    pub rbac: Rbac,
    pub show_automations_menu: bool,
    pub can_view_chats: bool,
    pub can_view_chat_history: bool,
}

pub fn Layout(props: LayoutProps) -> Element {
    let stylesheets = vec![index_css.name.to_string(), output_css.name.to_string()];

    let locale = props
        .locale
        .clone()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "en".to_string());

    let ai_assistants_label = i18n::ai_assistants(&locale);
    let prompts_label = i18n::prompts(&locale);
    let integrations_label = i18n::integrations(&locale);
    let history_label = i18n::histories(&locale);
    let datasets_label = i18n::datasets(&locale);

    let licence = Licence::global();
    let show_automations_menu = licence.features.automations;
    let use_mcp_sidebar = licence.features.mcp;
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

    let switch_teams_href = crate::routes::teams::Switch {
        team_id: props.team_id,
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
        ai_assistants: ai_assistants_label.clone(),
        prompts: prompts_label.clone(),
        integrations: integrations_label.clone(),
        history: history_label.clone(),
        datasets: datasets_label.clone(),
    };

    let sidebar_params = SidebarParams {
        team_id: props.team_id,
        selected_item: props.selected_item.clone(),
        rbac: props.rbac.clone(),
        show_automations_menu,
        can_view_chats,
        can_view_chat_history,
    };

    let sidebar_content = if use_mcp_sidebar {
        sidebar_mcp::render(&sidebar_params, &sidebar_labels)
    } else {
        sidebar::render(&sidebar_params, &sidebar_labels)
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
