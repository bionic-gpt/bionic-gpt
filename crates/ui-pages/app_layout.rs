#![allow(non_snake_case)]
use super::logout_form::LogoutForm;
use assets::files::*;
use daisy_rsx::{AppLayout, NavGroup, NavItem};
use db::rls::Rbac;
use dioxus::prelude::*;

#[derive(PartialEq, Eq, Debug)]
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

#[derive(Props)]
pub struct LayoutProps<'a> {
    selected_item: SideBar,
    title: &'a str,
    header: Element<'a>,
    children: Element<'a>,
    team_id: i32,
    rbac: &'a Rbac,
    section_class: &'a str,
}

pub fn Layout<'a>(cx: Scope<'a, LayoutProps<'a>>) -> Element {
    let stylesheets = vec![index_css.name.to_string(), output_css.name.to_string()];

    cx.render(rsx! {
        AppLayout {
            title: cx.props.title,
            stylesheets: stylesheets,
            js_href: index_js.name,
            section_class: cx.props.section_class,
            fav_icon_src: favicon_svg.name,
            collapse_svg_src: collapse_svg.name,
            header: cx.render(rsx!(
                &cx.props.header
            )),
            sidebar: cx.render(rsx!(
                NavGroup {
                    heading: "AI Chat",
                    content:  cx.render(rsx!(
                        NavItem {
                            id: SideBar::Console.to_string(),
                            selected_item_id: cx.props.selected_item.to_string(),
                            href: super::routes::console::index_route(cx.props.team_id),
                            icon: nav_service_requests_svg.name,
                            title: "Chat Console"
                        }
                        if cx.props.rbac.can_view_prompts() {
                            cx.render(rsx!(
                                NavItem {
                                    id: SideBar::Prompts.to_string(),
                                    selected_item_id: cx.props.selected_item.to_string(),
                                    href: super::routes::prompts::index_route(cx.props.team_id),
                                    icon: nav_dashboard_svg.name,
                                    title: "Prompts"
                                }
                            ))
                        }
                    ))
                }
                if cx.props.rbac.can_view_datasets() {
                    cx.render(rsx!(
                        NavGroup {
                            heading: "Retrieval Augmented Generation",
                            content:  cx.render(rsx!(
                                NavItem {
                                    id: SideBar::Datasets.to_string(),
                                    selected_item_id: cx.props.selected_item.to_string(),
                                    href: super::routes::datasets::index_route(cx.props.team_id),
                                    icon: nav_ccsds_data_svg.name,
                                    title: "Team Datasets"
                                }
                                NavItem {
                                    id: SideBar::DocumentPipelines.to_string(),
                                    selected_item_id: cx.props.selected_item.to_string(),
                                    href: super::routes::document_pipelines::index_route(cx.props.team_id),
                                    icon: nav_ccsds_data_svg.name,
                                    title: "Document Pipelines"
                                }
                            ))
                        }
                    ))
                }
                if cx.props.rbac.can_use_api_keys() {
                    cx.render(rsx!(
                        NavGroup {
                            heading: "Developers",
                            content:  cx.render(rsx!(
                                NavItem {
                                    id: SideBar::ApiKeys.to_string(),
                                    selected_item_id: cx.props.selected_item.to_string(),
                                    href: super::routes::api_keys::index_route(cx.props.team_id),
                                    icon: nav_api_keys_svg.name,
                                    title: "LLM API Keys"
                                }
                            ))
                        }
                    ))
                }
                if cx.props.rbac.can_view_teams() {
                    cx.render(rsx!(
                        NavGroup {
                            heading: "Collaboration",
                            content:  cx.render(rsx!(
                                NavItem {
                                    id: SideBar::Team.to_string(),
                                    selected_item_id: cx.props.selected_item.to_string(),
                                    href: super::routes::team::index_route(cx.props.team_id),
                                    icon: nav_members_svg.name,
                                    title: "Team Members"
                                }
                                NavItem {
                                    id: SideBar::Switch.to_string(),
                                    selected_item_id: cx.props.selected_item.to_string(),
                                    href: super::routes::team::switch_route(cx.props.team_id),
                                    icon: nav_teams_svg.name,
                                    title: "Your Teams"
                                }
                            ))
                        }
                    ))
                }
                if cx.props.rbac.is_sys_admin {
                    cx.render(rsx!(
                        NavGroup {
                            heading: "System Admin",
                            content:  cx.render(rsx!(
                                NavItem {
                                    id: SideBar::Models.to_string(),
                                    selected_item_id: cx.props.selected_item.to_string(),
                                    href: super::routes::models::index_route(cx.props.team_id),
                                    icon: nav_phonebook_svg.name,
                                    title: "Model Setup"
                                }
                                NavItem {
                                    id: SideBar::AuditTrail.to_string(),
                                    selected_item_id: cx.props.selected_item.to_string(),
                                    href: super::routes::audit_trail::index_route(cx.props.team_id),
                                    icon: nav_audit_svg.name,
                                    title: "Audit Trail"
                                }
                            ))
                        }
                    ))
                }
            )),
            sidebar_header: cx.render(rsx!(
                turbo-frame {
                    id: "teams-popup",
                    class: "min-w-full",
                    src: "{super::routes::team::teams_popup_route(cx.props.team_id)}"
                }
            )),
            sidebar_footer: cx.render(rsx!(
                turbo-frame {
                    id: "profile-popup",
                    class: "min-w-full",
                    src: "{super::routes::profile::profile_popup_route(cx.props.team_id)}"
                }
            )),
            &cx.props.children
            snack-bar {}
            LogoutForm {}
        }
    })
}
